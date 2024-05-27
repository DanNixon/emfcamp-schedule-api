use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::Query;
use chrono::{DateTime, FixedOffset};
use emfcamp_schedule_api::schedule::mutation;
use metrics::counter;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ScheduleQueryParams {
    /// Offset the timestamps of all events, using this as the starting time of the earliest event.
    /// For development use.
    fake_epoch: Option<DateTime<FixedOffset>>,

    /// Include only events that start after this time.
    starting_after: Option<DateTime<FixedOffset>>,

    /// Include only events that start before this time.
    starting_before: Option<DateTime<FixedOffset>>,

    /// Include only events that end after this time.
    ending_after: Option<DateTime<FixedOffset>>,

    /// Include only events that take place at these venues.
    #[serde(rename = "venue")]
    venues: Option<Vec<String>>,
}

impl From<ScheduleQueryParams> for mutation::Mutators {
    fn from(params: ScheduleQueryParams) -> Self {
        let mut mutators = Self::new_single(Box::<mutation::SortedByStartTime>::default());

        if let Some(epoch) = params.fake_epoch {
            mutators.push(Box::new(mutation::FakeStartEpoch::new(epoch)));
        }

        if let Some(starting_after) = params.starting_after {
            mutators.push(Box::new(mutation::StartsAfter::new(starting_after)));
        }

        if let Some(starting_before) = params.starting_before {
            mutators.push(Box::new(mutation::StartsBefore::new(starting_before)));
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
    info!("Query: schedule: {:?}", query);
    counter!(crate::metrics::REQUESTS, crate::metrics::ENDPOINT_LABEL => "schedule").increment(1);

    match state.client.get_schedule().await {
        Ok(mut schedule) => {
            let mutators = query.into();
            schedule.mutate(&mutators);

            let events = &mut schedule.events;
            Json(events).into_response()
        }
        Err(err) => {
            error!("{err}");
            counter!(crate::metrics::UPSTREAM_API_FAILURES).increment(1);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
