#[cfg(test)]
mod test;
mod utils;

use crate::{
    schedule::{event::Event, Schedule},
    Client,
};
use chrono::{DateTime, Duration as ChronoDuration, FixedOffset, Utc};
use derive_builder::Builder;
use metrics::{counter, describe_counter, describe_gauge, gauge};
use tokio::time::{Duration as TokioDuration, Interval};
use tracing::{debug, info, warn};

const EVENT_METRIC_NAME: &str = "schedule_announcer_events";
const SCHEDULE_UPDATE_METRIC_NAME: &str = "schedule_announcer_schedule_updates";
const TIME_TO_NEXT_EVENT_METRIC_NAME: &str = "schedule_announcer_time_to_next_event";

#[derive(Debug, Builder)]
#[builder(default)]
pub struct AnnouncerSettings {
    schedule_refresh: TokioDuration,
    event_start_offset: ChronoDuration,
}

impl Default for AnnouncerSettings {
    fn default() -> Self {
        Self {
            schedule_refresh: TokioDuration::from_secs(60),
            event_start_offset: ChronoDuration::zero(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AnnouncerScheduleChanges {
    Changes,
    NoChanges,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq)]
pub enum AnnouncerPollResult {
    Event(Event),
    ScheduleRefreshed(AnnouncerScheduleChanges),
}

/// A subset of fields from the last event that was notified.
/// Used to select the next event to be notified.
struct LastNotifiedEventMarker {
    start: DateTime<FixedOffset>,
    id: u32,
}

impl LastNotifiedEventMarker {
    fn matches(&self, event: &Event) -> bool {
        self.start == event.start && self.id == event.id
    }
}

impl From<&Event> for LastNotifiedEventMarker {
    fn from(value: &Event) -> Self {
        Self {
            start: value.start,
            id: value.id,
        }
    }
}

pub struct Announcer {
    settings: AnnouncerSettings,
    client: Client,
    schedule: Schedule,
    schedule_update_interval: Interval,
    last_notified_event_marker: Option<LastNotifiedEventMarker>,
}

impl Announcer {
    pub async fn new(settings: AnnouncerSettings, client: Client) -> crate::Result<Self> {
        describe_counter!(
            EVENT_METRIC_NAME,
            "Number of event notifications returned by the announcer"
        );
        describe_counter!(
            SCHEDULE_UPDATE_METRIC_NAME,
            "Number of schedule updates checked for by the announcer"
        );
        describe_gauge!(
            TIME_TO_NEXT_EVENT_METRIC_NAME,
            metrics::Unit::Seconds,
            "Time until the next event needs to be announced"
        );

        let schedule = self::utils::get_sorted_schedule(&client).await?;

        let mut schedule_update_interval = tokio::time::interval(settings.schedule_refresh);
        schedule_update_interval.reset();

        Ok(Self {
            settings,
            client,
            schedule,
            schedule_update_interval,
            last_notified_event_marker: None,
        })
    }

    async fn update_schedule(&mut self) -> crate::Result<AnnouncerScheduleChanges> {
        let schedule = self::utils::get_sorted_schedule(&self.client).await?;

        let changes = if self.schedule == schedule {
            debug!("No changes in new schedule");
            counter!(SCHEDULE_UPDATE_METRIC_NAME, "result" => "ok", "changes" => "no").increment(1);
            AnnouncerScheduleChanges::NoChanges
        } else {
            info!("New schedule is different from previously loaded");
            counter!(SCHEDULE_UPDATE_METRIC_NAME, "result" => "ok", "changes" => "yes")
                .increment(1);
            AnnouncerScheduleChanges::Changes
        };

        self.schedule = schedule;

        Ok(changes)
    }

    pub async fn poll(&mut self) -> crate::Result<AnnouncerPollResult> {
        loop {
            // Determine what the next event to announce is and in how much time it is due to be announced
            let next_event = self.get_next_event_to_announce();
            let event_wait_time = match next_event {
                Some(ref event) => self::utils::get_duration_before_event_notification(
                    Utc::now().into(),
                    self.settings.event_start_offset,
                    event,
                ),
                None => TokioDuration::from_secs(60),
            };
            info!("Time to wait before next event: {:?}", event_wait_time);
            gauge!(TIME_TO_NEXT_EVENT_METRIC_NAME).set(event_wait_time.as_secs_f64());

            // Wait for one of several things to happen...
            tokio::select! {
                // 1. The schedule is refreshed at the requested interval
                _ = self.schedule_update_interval.tick() => {
                    match self.update_schedule().await {
                        Ok(changes) => {
                            return Ok(AnnouncerPollResult::ScheduleRefreshed(changes))
                        },
                        Err(e) => {
                            warn!("Failed to update schedule: {e}");
                            counter!(SCHEDULE_UPDATE_METRIC_NAME, "result" => "error").increment(1);
                        },
                    }
                }
                // 2. The next event to be announced needs to be announced
                _ = tokio::time::sleep(event_wait_time) => {
                    if let Some(event) = next_event {
                        metrics::counter!(EVENT_METRIC_NAME).increment(1);
                        self.update_event_marker(&event);
                        return Ok(AnnouncerPollResult::Event(event))
                    }
                }
            }
        }
    }

    fn get_next_event_to_announce(&self) -> Option<Event> {
        get_next_event_to_announce(
            &self.schedule.events,
            self.settings.event_start_offset,
            &self.last_notified_event_marker,
            Utc::now().into(),
        )
    }

    fn update_event_marker(&mut self, event: &Event) {
        self.last_notified_event_marker = Some(event.into());
    }
}

fn get_next_event_to_announce(
    events: &[Event],
    event_offset: ChronoDuration,
    last_notified_event_marker: &Option<LastNotifiedEventMarker>,
    now: DateTime<FixedOffset>,
) -> Option<Event> {
    match last_notified_event_marker {
        Some(marker) => match events.iter().position(|event| marker.matches(event)) {
            Some(idx) => {
                debug!("Matched last notified event marker, picking next in schedule as next to announce");
                events.get(idx + 1).cloned()
            }
            None => {
                debug!("Last notified event marker matched no events (something's fucky...), picking next chronological event from last announced as next to announce");
                events
                    .iter()
                    .find(|event| event.start > marker.start)
                    .cloned()
            }
        },
        None => {
            debug!(
                    "No last notified event marker, picking next in schedule chronologically from now as next to announce"
                );
            events
                .iter()
                .find(|event| event.start + event_offset >= now)
                .cloned()
        }
    }
}
