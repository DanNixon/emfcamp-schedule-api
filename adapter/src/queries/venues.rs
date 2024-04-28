use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use metrics::counter;
use tracing::{error, info};

#[axum::debug_handler]
pub(crate) async fn venues(State(state): State<crate::State>) -> Response {
    info!("Query: venues");
    counter!(crate::metrics::REQUESTS, crate::metrics::ENDPOINT_LABEL => "venues").increment(1);

    match state.client.get_schedule().await {
        Ok(schedule) => {
            let venues = schedule.venues();
            Json(venues).into_response()
        }
        Err(err) => {
            error!("{err}");
            counter!(crate::metrics::UPSTREAM_API_FAILURES).increment(1);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
