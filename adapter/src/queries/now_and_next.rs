use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::Query;
use chrono::{DateTime, FixedOffset, Local};
use emfcamp_schedule_api::schedule::mutation;
use metrics::counter;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NowAndNextQueryParams {
    /// Offset the timestamps of all events, using this as the starting time of the earliest event.
    /// For development use.
    fake_epoch: Option<DateTime<FixedOffset>>,

    /// Use this time instead of the actual currrent time when evaluating events against time based filters.
    /// For development use.
    now: Option<DateTime<FixedOffset>>,

    /// Include only events that take place at these venues.
    #[serde(rename = "venue")]
    venues: Option<Vec<String>>,
}

impl From<NowAndNextQueryParams> for mutation::Mutators {
    fn from(params: NowAndNextQueryParams) -> Self {
        let mut mutators = Self::new_single(Box::<mutation::SortedByStartTime>::default());

        if let Some(epoch) = params.fake_epoch {
            mutators.push(Box::new(mutation::FakeStartEpoch::new(epoch)));
        }

        if let Some(venues) = params.venues {
            mutators.push(Box::new(mutation::AtVenues::new(venues)));
        }

        mutators
    }
}

#[axum::debug_handler]
pub(crate) async fn now_and_next(
    State(state): State<crate::State>,
    Query(query): Query<NowAndNextQueryParams>,
) -> Response {
    info!("Query: now and next: {:?}", query);
    counter!(crate::metrics::REQUESTS, crate::metrics::ENDPOINT_LABEL => "now_and_next")
        .increment(1);

    match state.client.get_schedule().await {
        Ok(mut schedule) => {
            let now = query.now.unwrap_or_else(|| Local::now().into());

            let mutators = query.into();
            schedule.mutate(&mutators);

            let epg = schedule.now_and_next(now);
            Json(epg).into_response()
        }
        Err(err) => {
            error!("{err}");
            counter!(crate::metrics::UPSTREAM_API_FAILURES).increment(1);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
