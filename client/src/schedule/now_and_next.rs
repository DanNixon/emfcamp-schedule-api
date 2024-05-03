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
            result.guide.insert(
                venue.clone(),
                VenueNowAndNext {
                    now: match events
                        .iter()
                        .find(|e| e.venue == venue && e.relative_to(now) == RelativeTime::Now)
                    {
                        Some(e) => vec![e.clone()],
                        None => Vec::default(),
                    },
                    next: match events
                        .iter()
                        .find(|e| e.venue == venue && e.relative_to(now) == RelativeTime::Future)
                    {
                        Some(e) => vec![e.clone()],
                        None => Vec::default(),
                    },
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
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 2".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
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
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 1".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
                    DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
                );
                e.venue = "venue 2".to_owned();
                e
            },
            {
                let mut e = Event::dummy(
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
}
