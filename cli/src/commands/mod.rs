pub(super) mod details;
pub(super) mod full;
pub(super) mod now_next;
pub(super) mod upcoming;
pub(super) mod venues;

use crate::formatting::event_listing;
use chrono::{DateTime, FixedOffset};
use clap::Parser;

#[derive(Debug, Parser)]
pub(super) struct TableWidthCommonOptions {
    /// Maximum width of the table in characters
    #[clap(short = 'w', long, default_value = "120")]
    max_width: usize,
}

#[derive(Debug, Parser)]
pub(super) struct ListingTableCommonOptions {
    #[clap(flatten)]
    width: TableWidthCommonOptions,

    /// Only show specific columns
    #[clap(short, long, value_enum, default_values_t = event_listing::default_columns())]
    columns: Vec<event_listing::Column>,
}

#[derive(Debug, Parser)]
pub(super) struct VenueFilterCommonOptions {
    /// Only show specific venues
    #[clap(short, long = "venue", value_name = "VENUE")]
    venues: Option<Vec<String>>,
}

#[derive(Debug, Parser)]
pub(super) struct NowCommonOptions {
    /// Manually specify the time to consider "now" (mainly used for debugging)
    #[clap(long, value_name = "TIMESTAMP")]
    now: Option<DateTime<FixedOffset>>,
}

impl NowCommonOptions {
    fn now(&self) -> DateTime<FixedOffset> {
        match self.now {
            Some(now) => now,
            None => chrono::Local::now().into(),
        }
    }
}
