use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::Query;
use chrono::{DateTime, FixedOffset};
use emfcamp_schedule_api::schedule::mutation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ScheduleQueryParams {
    /// Offset the timestamps of all events, using this as the starting time of the earliest event.
    /// For development use.
    fake_epoch: Option<DateTime<FixedOffset>>,

    /// Include only events that start after this time.
    starting_after: Option<DateTime<FixedOffset>>,

    /// Include only events that end after this time.
    ending_after: Option<DateTime<FixedOffset>>,

    /// Include only events that take place at these venues.
    #[serde(rename = "venue")]
    venues: Option<Vec<String>>,
}

impl From<ScheduleQueryParams> for mutation::Mutators {
    fn from(params: ScheduleQueryParams) -> Self {
        let mut mutators = Self::default();

        if let Some(epoch) = params.fake_epoch {
            mutators.push(Box::new(mutation::FakeStartEpoch::new(epoch)));
        }

        if let Some(starting_after) = params.starting_after {
            mutators.push(Box::new(mutation::StartsAfter::new(starting_after)));
        }

        if let Some(ending_after) = params.ending_after {
            mutators.push(Box::new(mutation::EndsAfter::new(ending_after)));
        }

        if let Some(venues) = params.venues {
            mutators.push(Box::new(mutation::AtVenues::new(venues)));
        }

        mutators
    }
}

#[axum::debug_handler]
pub(crate) async fn schedule(
    State(state): State<crate::State>,
    Query(query): Query<ScheduleQueryParams>,
) -> Response {
    let mut schedule = state.client.get_schedule().await;

    let mutators = query.into();
    schedule.mutate(&mutators);

    let events = &mut schedule.events;

    Json(events).into_response()
}
