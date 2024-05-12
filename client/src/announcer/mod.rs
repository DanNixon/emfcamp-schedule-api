#[cfg(test)]
mod test;
mod utils;

use crate::{
    schedule::{event::Event, Schedule},
    Client,
};
use chrono::{DateTime, Duration as ChronoDuration, FixedOffset, Utc};
use derive_builder::Builder;
use tokio::time::{Duration as TokioDuration, Interval};
use tracing::{debug, info, warn};

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
    NoMoreEvents,
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
            AnnouncerScheduleChanges::NoChanges
        } else {
            info!("New schedule is different from previously loaded");
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
            debug!("Time to wait before next event: {:?}", event_wait_time);

            // Wait for one of several things to happen...
            tokio::select! {
                // 1. The schedule is refreshed at the requested interval
                _ = self.schedule_update_interval.tick() => {
                    match self.update_schedule().await {
                        Ok(changes) => {
                            return Ok(AnnouncerPollResult::ScheduleRefreshed(changes))
                        },
                        Err(e) => {
                            warn!("Failed to update schedule: {e}")
                        },
                    }
                }
                // 2. The next event to be announced needs to be announced
                _ = tokio::time::sleep(event_wait_time) => {
                    if let Some(event) = next_event {
                        self.update_event_marker(&event);
                        return Ok(AnnouncerPollResult::Event(event))
                    }
                }
            }
        }
    }

    fn get_next_event_to_announce(&self) -> Option<Event> {
        match &self.last_notified_event_marker {
            Some(marker) => {
                match self
                    .schedule
                    .events
                    .iter()
                    .position(|event| marker.matches(event))
                {
                    Some(idx) => {
                        debug!("Matched last notified event marker, picking next in schedule as next to announce");
                        self.schedule.events.get(idx + 1).cloned()
                    }
                    None => {
                        debug!("Last notified event marker matched no events (something's fucky...), picking next chronological event as next to announce");
                        self.schedule
                            .events
                            .iter()
                            .find(|event| event.start > marker.start)
                            .cloned()
                    }
                }
            }
            None => {
                debug!(
                    "No last notified event marker, picking first in schedule as next to announce"
                );
                self.schedule.events.first().cloned()
            }
        }
    }

    fn update_event_marker(&mut self, event: &Event) {
        self.last_notified_event_marker = Some(event.into());
    }
}
