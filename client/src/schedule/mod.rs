pub mod event;
pub mod mutation;
pub mod now_and_next;

use self::mutation::Mutators;
use chrono::{DateTime, FixedOffset};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Schedule {
    pub events: Vec<event::Event>,
}

fn get_unique_venues_from_events(events: &[event::Event]) -> Vec<String> {
    let venues: HashSet<String> = events.iter().map(|e| e.venue.clone()).collect();
    venues.into_iter().collect()
}

impl Schedule {
    pub fn mutate(&mut self, mutators: &Mutators) {
        mutators.mutate(&mut self.events);
    }

    pub fn venues(&self) -> Vec<String> {
        let mut venues = get_unique_venues_from_events(&self.events);
        venues.sort();
        venues
    }

    pub fn now_and_next(&self, now: DateTime<FixedOffset>) -> now_and_next::NowAndNext {
        now_and_next::NowAndNext::new(&self.events, now)
    }
}
