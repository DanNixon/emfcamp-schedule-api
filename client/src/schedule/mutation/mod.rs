mod at_venues;
mod ends_after;
mod fake_start_epoch;
mod starts_after;

pub use self::{
    at_venues::AtVenues, ends_after::EndsAfter, fake_start_epoch::FakeStartEpoch,
    starts_after::StartsAfter,
};
use super::event::Event;

#[derive(Default)]
pub struct Mutators {
    mutators: Vec<Box<dyn Mutator>>,
}

impl Mutators {
    pub fn push(&mut self, mutator: Box<dyn Mutator>) {
        self.mutators.push(mutator);
    }

    pub(crate) fn mutate(&self, events: &mut Vec<Event>) {
        for a in &self.mutators {
            a.mutate(events);
        }
    }
}

pub trait Mutator {
    fn mutate(&self, events: &mut Vec<Event>);
}
