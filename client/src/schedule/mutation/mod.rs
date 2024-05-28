mod at_venues;
mod ends_after;
mod fake_start_epoch;
mod sorted_by_start_time;
mod starts_after;
mod starts_before;

pub use self::{
    at_venues::AtVenues, ends_after::EndsAfter, fake_start_epoch::FakeStartEpoch,
    sorted_by_start_time::SortedByStartTime, starts_after::StartsAfter,
    starts_before::StartsBefore,
};
use super::event::Event;

pub type BoxedMutator = Box<dyn Mutator + Send + Sync>;

#[derive(Default)]
pub struct Mutators {
    mutators: Vec<BoxedMutator>,
}

impl Mutators {
    pub fn new(mutators: Vec<BoxedMutator>) -> Self {
        Self { mutators }
    }

    pub fn new_single(mutator: BoxedMutator) -> Self {
        Self {
            mutators: vec![mutator],
        }
    }

    pub fn push(&mut self, mutator: BoxedMutator) {
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
