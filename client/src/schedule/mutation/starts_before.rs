use super::{Event, Mutator};
use chrono::{DateTime, FixedOffset};

pub struct StartsBefore {
    timestamp: DateTime<FixedOffset>,
}

impl StartsBefore {
    pub fn new(timestamp: DateTime<FixedOffset>) -> Self {
        Self { timestamp }
    }
}

impl Mutator for StartsBefore {
    fn mutate(&self, events: &mut Vec<Event>) {
        events.retain(|event| event.start < self.timestamp);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn basic() {
        let events = vec![
            Event::dummy(
                0,
                DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap(),
            ),
            Event::dummy(
                1,
                DateTime::parse_from_rfc3339("2024-03-12T21:00:00+00:00").unwrap(),
            ),
            Event::dummy(
                2,
                DateTime::parse_from_rfc3339("2024-03-12T22:00:00+00:00").unwrap(),
            ),
        ];

        let mutator =
            StartsBefore::new(DateTime::parse_from_rfc3339("2024-03-12T21:59:59+00:00").unwrap());

        let mut mutated = events.clone();
        mutator.mutate(&mut mutated);

        assert_eq!(mutated.len(), 2);

        assert_eq!(mutated[0], events[0]);
        assert_eq!(mutated[1], events[1]);
    }
}
