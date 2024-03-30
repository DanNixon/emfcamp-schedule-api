use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};

#[axum::debug_handler]
pub(crate) async fn venues(State(state): State<crate::State>) -> Response {
    let schedule = state.client.get_schedule().await;
    let venues = schedule.venues();
    Json(venues).into_response()
}
