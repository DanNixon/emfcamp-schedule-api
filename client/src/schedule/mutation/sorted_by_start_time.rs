use super::{Event, Mutator};

#[derive(Default)]
pub struct SortedByStartTime {}

impl Mutator for SortedByStartTime {
    fn mutate(&self, events: &mut Vec<Event>) {
        events.sort();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn basic() {
        let events = vec![
            Event::dummy(DateTime::parse_from_rfc3339("2024-03-12T20:30:00+00:00").unwrap()),
            Event::dummy(DateTime::parse_from_rfc3339("2024-03-12T21:00:00+00:00").unwrap()),
            Event::dummy(DateTime::parse_from_rfc3339("2024-03-12T20:00:00+00:00").unwrap()),
        ];

        let mutator = SortedByStartTime::default();

        let mut mutated = events.clone();
        mutator.mutate(&mut mutated);

        assert_eq!(mutated.len(), 3);

        assert_eq!(mutated[0], events[2]);
        assert_eq!(mutated[1], events[0]);
        assert_eq!(mutated[2], events[1]);
    }
}
