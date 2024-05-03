use super::{NowCommonOptions, TableWidthCommonOptions, VenueFilterCommonOptions};
use ascii_table::AsciiTable;
use clap::Parser;
use emfcamp_schedule_api::schedule::{
    event::Event,
    mutation::{self, Mutators, SortedByStartTime},
    Schedule,
};

#[derive(Debug, Parser)]
pub(crate) struct NowNextOptions {
    #[clap(flatten)]
    width: TableWidthCommonOptions,

    #[clap(flatten)]
    venues: VenueFilterCommonOptions,

    #[clap(flatten)]
    now: NowCommonOptions,
}

pub(crate) fn run(args: NowNextOptions, mut schedule: Schedule) {
    let var_name = Mutators::default();
    let mut mutations = var_name;

    // Filter by venue if requested
    if let Some(venues) = args.venues.venues {
        mutations.push(Box::new(mutation::AtVenues::new(venues)));
    }

    // Sort by start time
    mutations.push(Box::<SortedByStartTime>::default());

    schedule.mutate(&mutations);

    let now_next = schedule.now_and_next(args.now.now());

    println!("Now: {}", now_next.now);

    let mut table = AsciiTable::default();
    table.set_max_width(args.width.max_width);

    table.column(0).set_header("Venue");
    table.column(1).set_header("Now");
    table.column(2).set_header("Next");

    let table_data: Vec<_> = now_next
        .guide
        .iter()
        .map(|(venue, guide)| {
            vec![
                venue.to_string(),
                format_event_now(&guide.now),
                format_event_next(&guide.next),
            ]
        })
        .collect();

    table.print(table_data);
}

fn format_event_now(events: &[Event]) -> String {
    match events.first() {
        Some(event) => format!(
            ">{} [{}] {}",
            event.end.format("%H:%M"),
            event.id,
            event.title
        ),
        None => "".to_string(),
    }
}

fn format_event_next(events: &[Event]) -> String {
    match events.first() {
        Some(event) => format!(
            "@{} [{}] {}",
            event.start.format("%H:%M"),
            event.id,
            event.title
        ),
        None => "".to_string(),
    }
}
