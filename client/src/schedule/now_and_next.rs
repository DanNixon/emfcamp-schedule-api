use super::event::{Event, RelativeTime};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct NowAndNext {
    pub now: DateTime<FixedOffset>,
    pub guide: HashMap<String, VenueNowAndNext>,
}

impl NowAndNext {
    pub(super) fn new(events: &[Event], now: DateTime<FixedOffset>) -> Self {
        let mut result = Self {
            now,
            guide: Default::default(),
        };

        for venue in super::get_unique_venues_from_events(events) {
            let events_now = events
                .iter()
                .filter_map(|e| {
                    if e.venue == venue && e.relative_to(now) == RelativeTime::Now {
                        Some(e.clone())
                    } else {
                        None
                    }
                })
                .collect();

            // Find the next event to happen in the future.
            // This will determine the start time of the next event(s).
            let events_next = match events
                .iter()
                .find(|e| e.venue == venue && e.relative_to(now) == RelativeTime::Future)
            {
                // Then find all events that have the same start time.
                // This set of events are the next to happen at the venue in question.
                Some(first_future_event) => events
                    .iter()
                    .filter_map(|e| {
                        if e.venue == venue && e.start == first_future_event.start {
                            Some(e.clone())
                        } else {
                            None
                        }
                    })
                    .collect(),
                None => Vec::default(),
            };

            result.guide.insert(
                venue.clone(),
                VenueNowAndNext {
                    now: events_now,
                    next: events_next,
                },
            );
        }

        result
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VenueNowAndNext {
    /// An event is "now" if the query time is between its start and end timestamps.
    pub now: Vec<Event>,

    /// An event is "next" if is the first event in chronological order to have a start timestamp
    /// that is later than the query timestamp.
    pub next: Vec<Event>,
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn basic_1() {
        let events = vec![
            {
                let mut e = Event::dummy(
                    0,
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    1,
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 2".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    2,
                    DateTime::parse_from_rfc3339("2024-03-12T21:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
        ];

        let t = DateTime::parse_from_rfc3339("2024-03-12T20:30:00+00:00").unwrap();
        let now_and_next = NowAndNext::new(&events, t);

        assert_eq!(now_and_next.now, t);

        assert_eq!(now_and_next.guide.len(), 2);

        assert_eq!(now_and_next.guide["venue 1"].now, vec![events[0].clone()]);
        assert_eq!(now_and_next.guide["venue 1"].next, vec![events[2].clone()]);

        assert_eq!(now_and_next.guide["venue 2"].now, vec![events[1].clone()]);
        assert_eq!(now_and_next.guide["venue 2"].next, Vec::default());
    }

    #[test]
    fn basic_2() {
        let events = vec![
            {
                let mut e = Event::dummy(
                    0,
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    1,
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 2".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    2,
                    DateTime::parse_from_rfc3339("2024-03-12T21:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
        ];

        let t = DateTime::parse_from_rfc3339("2024-03-12T19:59:59+00:00").unwrap();
        let now_and_next = NowAndNext::new(&events, t);

        assert_eq!(now_and_next.now, t);

        assert_eq!(now_and_next.guide.len(), 2);

        assert_eq!(now_and_next.guide["venue 1"].now, Vec::default());
        assert_eq!(now_and_next.guide["venue 1"].next, vec![events[0].clone()]);

        assert_eq!(now_and_next.guide["venue 2"].now, Vec::default());
        assert_eq!(now_and_next.guide["venue 2"].next, vec![events[1].clone()]);
    }

    #[test]
    fn concurrent_now() {
        let events = vec![
            {
                let mut e = Event::dummy(
                    0,
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    1,
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 2".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    2,
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    3,
                    DateTime::parse_from_rfc3339("2024-03-12T21:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
        ];

        let t = DateTime::parse_from_rfc3339("2024-03-12T20:30:00+00:00").unwrap();
        let now_and_next = NowAndNext::new(&events, t);

        assert_eq!(now_and_next.now, t);

        assert_eq!(now_and_next.guide.len(), 2);

        assert_eq!(
            now_and_next.guide["venue 1"].now,
            vec![events[0].clone(), events[2].clone()]
        );
        assert_eq!(now_and_next.guide["venue 1"].next, vec![events[3].clone()]);
    }

    #[test]
    fn concurrent_next() {
        let events = vec![
            {
                let mut e = Event::dummy(
                    0,
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    1,
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 2".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    2,
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    3,
                    DateTime::parse_from_rfc3339("2024-03-12T21:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
        ];

        let t = DateTime::parse_from_rfc3339("2024-03-12T19:59:59+00:00").unwrap();
        let now_and_next = NowAndNext::new(&events, t);

        assert_eq!(now_and_next.now, t);

        assert_eq!(now_and_next.guide.len(), 2);

        assert_eq!(now_and_next.guide["venue 1"].now, Vec::default(),);
        assert_eq!(
            now_and_next.guide["venue 1"].next,
            vec![events[0].clone(), events[2].clone()]
        );
    }
}
