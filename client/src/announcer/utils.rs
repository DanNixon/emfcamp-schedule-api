use crate::{
    schedule::{
        event::Event,
        mutation::{Mutators, SortedByStartTime},
        Schedule,
    },
    Client,
};
use chrono::{DateTime, Duration as ChronoDuration, FixedOffset};
use tokio::time::{Duration as TokioDuration, Instant};
use tracing::warn;

pub(super) async fn get_sorted_schedule(client: &Client) -> crate::Result<(Schedule, Instant)> {
    let mut schedule = client.get_schedule().await?;
    schedule.mutate(&Mutators::new_single(Box::new(SortedByStartTime {})));

    let now = Instant::now();

    Ok((schedule, now))
}

pub(super) fn get_duration_before_event_notification(
    timepoint: DateTime<FixedOffset>,
    start_offset: ChronoDuration,
    event: &Event,
) -> TokioDuration {
    let delta = (event.start + start_offset) - timepoint;

    delta.to_std().unwrap_or_else(|e| {
        warn!("Negative time before event, something may be fucky... ({e})");
        std::time::Duration::ZERO
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn duration_before_event_notification_zero_offset() {
        let now: DateTime<FixedOffset> = Utc::now().into();
        let time_until_event = ChronoDuration::try_hours(2).unwrap();

        let event = Event::dummy(0, now + time_until_event);
        let time_until_notification =
            get_duration_before_event_notification(now, ChronoDuration::zero(), &event);

        assert_eq!(time_until_notification, time_until_event.to_std().unwrap());
    }

    #[test]
    fn duration_before_event_notification_negative_offset() {
        let now: DateTime<FixedOffset> = Utc::now().into();
        let time_until_event = ChronoDuration::try_hours(2).unwrap();

        let event = Event::dummy(0, now + time_until_event);
        let offset = ChronoDuration::try_minutes(-2).unwrap();
        let time_until_notification = get_duration_before_event_notification(now, offset, &event);

        let expected = time_until_event.to_std().unwrap() - TokioDuration::from_secs(60 * 2);
        assert_eq!(time_until_notification, expected);
    }

    #[test]
    fn duration_before_event_notification_positive_offset() {
        let now: DateTime<FixedOffset> = Utc::now().into();
        let time_until_event = ChronoDuration::try_hours(2).unwrap();

        let event = Event::dummy(0, now + time_until_event);
        let offset = ChronoDuration::try_minutes(2).unwrap();
        let time_until_notification = get_duration_before_event_notification(now, offset, &event);

        let expected = time_until_event.to_std().unwrap() + TokioDuration::from_secs(60 * 2);
        assert_eq!(time_until_notification, expected);
    }

    #[test]
    fn duration_before_event_notification_zero_offset_long_duration() {
        let now: DateTime<FixedOffset> = Utc::now().into();
        let time_until_event = ChronoDuration::try_days(30).unwrap();

        let event = Event::dummy(0, now + time_until_event);
        let time_until_notification =
            get_duration_before_event_notification(now, ChronoDuration::zero(), &event);

        assert_eq!(time_until_notification, time_until_event.to_std().unwrap());
    }

    #[test]
    fn duration_before_event_notification_zero_duration() {
        let now: DateTime<FixedOffset> = Utc::now().into();
        let time_until_event = ChronoDuration::try_minutes(1).unwrap();

        let event = Event::dummy(0, now + time_until_event);
        let offset = ChronoDuration::try_minutes(-2).unwrap();
        let time_until_notification = get_duration_before_event_notification(now, offset, &event);

        assert_eq!(time_until_notification, TokioDuration::ZERO);
    }
}
