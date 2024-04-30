use crate::{
    schedule::{
        event::Event,
        mutation::{Mutators, SortedByStartTime},
        Schedule,
    },
    Client,
};
use chrono::{DateTime, Duration as ChronoDuration, FixedOffset};
use tokio::time::{Duration as TokioDuration, Instant};
use tracing::warn;

pub(super) async fn get_sorted_schedule(client: &Client) -> crate::Result<(Schedule, Instant)> {
    let mut schedule = client.get_schedule().await?;
    schedule.mutate(&Mutators::new_single(Box::new(SortedByStartTime {})));

    let now = Instant::now();

    Ok((schedule, now))
}

pub(super) fn get_duration_before_event(
    timepoint: DateTime<FixedOffset>,
    start_offset: ChronoDuration,
    event: &Event,
) -> TokioDuration {
    let delta = (event.start + start_offset) - timepoint;

    delta.to_std().unwrap_or_else(|e| {
        warn!("Negative time before event, something may be fucky... ({e})");
        std::time::Duration::ZERO
    })
}
