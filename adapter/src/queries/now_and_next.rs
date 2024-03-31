use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::Query;
use chrono::{DateTime, FixedOffset, Local};
use emfcamp_schedule_api::schedule::mutation;
use serde::{Deserialize, Serialize};

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
        let mut mutators = Self::default();

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
    let mut schedule = state.client.get_schedule().await;

    let now = query.now.unwrap_or_else(|| Local::now().into());

    let mutators = query.into();
    schedule.mutate(&mutators);

    let epg = schedule.now_and_next(now);

    Json(epg).into_response()
}
