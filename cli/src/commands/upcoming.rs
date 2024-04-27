use super::{ListingTableCommonOptions, NowCommonOptions, VenueFilterCommonOptions};
use crate::formatting::event_listing;
use clap::Parser;
use emfcamp_schedule_api::schedule::{
    mutation::{self, Mutators, SortedByStartTime},
    Schedule,
};

#[derive(Debug, Parser)]
pub(crate) struct UpcomingOptions {
    #[clap(flatten)]
    table: ListingTableCommonOptions,

    #[clap(flatten)]
    venues: VenueFilterCommonOptions,

    #[clap(flatten)]
    now: NowCommonOptions,
}

pub(crate) fn run(args: UpcomingOptions, mut schedule: Schedule) {
    let mut mutations = Mutators::default();

    // Filter by venue if requested
    if let Some(venues) = args.venues.venues {
        mutations.push(Box::new(mutation::AtVenues::new(venues)));
    }

    // Filter by events that have already finished
    mutations.push(Box::new(mutation::EndsAfter::new(args.now.now())));

    // Sort by start time
    mutations.push(Box::<SortedByStartTime>::default());

    schedule.mutate(&mutations);
    let events = schedule.events;

    event_listing::print_table(args.table.width.max_width, &args.table.columns, &events);
}
