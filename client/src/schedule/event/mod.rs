mod kind;
mod timestamp;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};
use std::cmp::Ordering;
use url::Url;

pub use self::kind::{Kind, Workshop};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Event {
    pub id: u32,

    pub slug: String,

    #[serde(rename = "start_date", deserialize_with = "timestamp::deserialize")]
    pub start: DateTime<FixedOffset>,

    #[serde(rename = "end_date", deserialize_with = "timestamp::deserialize")]
    pub end: DateTime<FixedOffset>,

    pub venue: String,

    #[serde_as(as = "NoneAsEmptyString")]
    pub map_link: Option<String>,

    pub title: String,

    pub speaker: String,

    #[serde_as(as = "NoneAsEmptyString")]
    pub pronouns: Option<String>,

    pub description: String,

    #[serde(flatten)]
    pub kind: Kind,

    pub may_record: Option<bool>,

    pub is_family_friendly: Option<bool>,

    pub link: Url,
    // TODO: other fields
}

impl Event {
    #[cfg(test)]
    pub fn dummy(start: DateTime<FixedOffset>) -> Self {
        use chrono::Duration;

        let duration = Duration::try_hours(1).unwrap();

        Self {
            id: 0,
            slug: "".to_owned(),
            start,
            end: start + duration,
            venue: "".to_owned(),
            map_link: None,
            title: "".to_owned(),
            speaker: "".to_owned(),
            pronouns: None,
            description: "".to_owned(),
            kind: Kind::Talk,
            may_record: None,
            is_family_friendly: None,
            link: Url::parse("http://example.com").unwrap(),
        }
    }

    pub fn relative_to(&self, timestamp: DateTime<FixedOffset>) -> RelativeTime {
        if self.end < timestamp {
            RelativeTime::Past
        } else if self.start <= timestamp && self.end >= timestamp {
            RelativeTime::Now
        } else if self.start > timestamp {
            RelativeTime::Future
        } else {
            panic!("event start and end timestamps are quite obviously wrong");
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Equal => self.venue.cmp(&other.venue),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum RelativeTime {
    Past,
    Now,
    Future,
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Duration;

    #[test]
    fn relative_time_past() {
        let event =
            Event::dummy(DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap());
        let t = event.end + Duration::try_seconds(1).unwrap();
        assert_eq!(event.relative_to(t), RelativeTime::Past);
    }

    #[test]
    fn relative_time_future() {
        let event =
            Event::dummy(DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap());
        let t = event.start - Duration::try_seconds(1).unwrap();
        assert_eq!(event.relative_to(t), RelativeTime::Future);
    }

    #[test]
    fn relative_time_now_1() {
        let event =
            Event::dummy(DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap());
        assert_eq!(event.relative_to(event.start), RelativeTime::Now);
    }

    #[test]
    fn relative_time_now_2() {
        let event =
            Event::dummy(DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap());
        assert_eq!(event.relative_to(event.end), RelativeTime::Now);
    }

    #[test]
    #[should_panic]
    fn relative_time_now_panic() {
        let mut event =
            Event::dummy(DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap());
        event.end = event.start - Duration::try_minutes(5).unwrap();
        assert_eq!(event.relative_to(event.start), RelativeTime::Now);
    }
}
