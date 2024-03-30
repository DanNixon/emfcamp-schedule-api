use super::{Event, Mutator};

pub struct AtVenues {
    venues: Vec<String>,
}

impl AtVenues {
    pub fn new(venues: Vec<String>) -> Self {
        Self { venues }
    }
}

impl Mutator for AtVenues {
    fn mutate(&self, events: &mut Vec<Event>) {
        events.retain(|event| self.venues.contains(&event.venue));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn basic() {
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

        let mutator = AtVenues::new(vec!["venue 1".to_owned()]);

        let mut mutated = events.clone();
        mutator.mutate(&mut mutated);

        assert_eq!(mutated.len(), 2);

        assert_eq!(mutated[0], events[0]);
        assert_eq!(mutated[1], events[2]);
    }
}
