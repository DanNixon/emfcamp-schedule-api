use crate::schedule::event::Event;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{net::TcpListener, task::JoinHandle};
use url::Url;

type Events = Arc<Mutex<Vec<Event>>>;

pub(crate) struct DummyScheduleServer {
    events: Events,

    port: u16,
    handle: Option<JoinHandle<()>>,
}

impl DummyScheduleServer {
    pub(crate) async fn new(port: u16) -> Self {
        let events = Arc::new(Mutex::default());

        let app = Router::new()
            .route("/schedule", get(schedule))
            .with_state(events.clone());

        let address = SocketAddr::new("127.0.0.1".parse().unwrap(), port);

        let listener = TcpListener::bind(address).await.unwrap();

        let handle = Some(tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap()
        }));

        Self {
            events,
            port,
            handle,
        }
    }

    pub(crate) fn url(&self) -> Url {
        Url::parse(&format!("http://localhost:{}/schedule", self.port)).unwrap()
    }

    pub(crate) fn set_events(&self, events: Vec<Event>) {
        *self.events.lock().unwrap() = events;
    }

    pub(crate) fn event(&self, idx: usize) -> Event {
        self.events.lock().unwrap()[idx].clone()
    }

    pub(crate) async fn stop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
            let _ = handle.await;
        }
    }
}

async fn schedule(State(state): State<Events>) -> Response {
    let events = state.lock().unwrap().clone();
    Json(events).into_response()
}

#[macro_export]
macro_rules! assert_future_in {
    ($future:expr, $expected_at:expr, $expected_value:expr) => {
        let result = tokio::time::timeout(Duration::from_secs(120), $future)
            .await
            .expect("future should not timeout");

        let finish = tokio::time::Instant::now();

        let tolerance = Duration::from_millis(500);
        let late = finish.checked_duration_since($expected_at);
        let early = $expected_at.checked_duration_since(finish);

        if let Some(late) = late {
            if late > tolerance {
                panic!("Future exited {:?} late with: {:?}", late, result);
            }
        } else if let Some(early) = early {
            if early > tolerance {
                panic!("Future exited {:?} early with: {:?}", early, result);
            }
        }

        let result = result.expect("a value from the future");
        assert_eq!(result, $expected_value);
    };
}
