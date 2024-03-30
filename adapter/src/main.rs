mod queries;

use crate::queries::now_and_next::now_and_next;
use crate::queries::schedule::schedule;
use crate::queries::venues::venues;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Clone)]
struct State {
    client: emfcamp_schedule_api::Client,
}

#[tokio::main]
async fn main() {
    let client = emfcamp_schedule_api::Client::new(
        url::Url::parse("https://www.emfcamp.org/schedule/2022.json").unwrap(),
    );

    let state = State { client };

    let app = Router::new()
        .route("/schedule", get(schedule))
        .route("/now-and-next", get(now_and_next))
        .route("/venues", get(venues))
        .with_state(state);

    let addr: SocketAddr = ([127, 0, 0, 1], 8000).into();
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
