use super::{Event, Mutator};
use chrono::{DateTime, FixedOffset};

/// Offsets event timestamps based on the difference between the start time of the first event and a given time.
/// Effectively making EMF start at a time of your choosing.
/// Useful for development only.
/// Note that the events must be sorted by start timestamp before using this mutator.
pub struct FakeStartEpoch {
    epoch: DateTime<FixedOffset>,
}

impl FakeStartEpoch {
    pub fn new(epoch: DateTime<FixedOffset>) -> Self {
        Self { epoch }
    }
}

impl Mutator for FakeStartEpoch {
    fn mutate(&self, events: &mut Vec<Event>) {
        if let Some(first_event) = events.first() {
            let first_event_start_time = first_event.start;

            for event in events.iter_mut() {
                let offset = event.start - first_event_start_time;
                event.start = self.epoch + offset;

                let offset = event.end - first_event_start_time;
                event.end = self.epoch + offset;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn basic() {
        let events = vec![
            Event::dummy(DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap()),
            Event::dummy(DateTime::parse_from_rfc3339("2024-03-12T21:00:00+00:00").unwrap()),
            Event::dummy(DateTime::parse_from_rfc3339("2024-03-12T22:00:00+00:00").unwrap()),
        ];

        let mutator =
            FakeStartEpoch::new(DateTime::parse_from_rfc3339("2024-02-01T12:00:00+00:00").unwrap());

        let mut mutated = events.clone();
        mutator.mutate(&mut mutated);

        assert_eq!(mutated.len(), 3);

        assert_eq!(
            mutated[0].start,
            DateTime::parse_from_rfc3339("2024-02-01T12:00:00+00:00").unwrap()
        );
        assert_eq!(
            mutated[0].end,
            DateTime::parse_from_rfc3339("2024-02-01T13:00:00+00:00").unwrap()
        );
        assert_eq!(
            mutated[1].start,
            DateTime::parse_from_rfc3339("2024-02-01T13:00:00+00:00").unwrap()
        );
        assert_eq!(
            mutated[1].end,
            DateTime::parse_from_rfc3339("2024-02-01T14:00:00+00:00").unwrap()
        );
        assert_eq!(
            mutated[2].start,
            DateTime::parse_from_rfc3339("2024-02-01T14:00:00+00:00").unwrap()
        );
        assert_eq!(
            mutated[2].end,
            DateTime::parse_from_rfc3339("2024-02-01T15:00:00+00:00").unwrap()
        );
    }
}
