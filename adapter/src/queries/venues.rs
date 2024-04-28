use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use metrics::counter;
use tracing::info;

#[axum::debug_handler]
pub(crate) async fn venues(State(state): State<crate::State>) -> Response {
    info!("Query: venues");
    counter!(crate::metrics::REQUESTS, crate::metrics::ENDPOINT_LABEL => "venues").increment(1);

    let schedule = state.client.get_schedule().await;

    let venues = schedule.venues();

    Json(venues).into_response()
}
