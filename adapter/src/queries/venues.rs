use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use tracing::info;

#[axum::debug_handler]
pub(crate) async fn venues(State(state): State<crate::State>) -> Response {
    info!("Query: venues");

    let schedule = state.client.get_schedule().await;

    let venues = schedule.venues();

    Json(venues).into_response()
}
