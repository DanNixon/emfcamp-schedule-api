mod t02;
mod t03;
mod t04;
mod t05;
mod t06;
mod t07;
mod t08;
mod t09;

use super::*;
use crate::testing::DummyScheduleServer;
use serde_json::Value;
use std::collections::HashMap;
use tokio::time::{Duration, Instant};

fn set_and_patch_dummy_events(
    server: &mut DummyScheduleServer,
    mut events: Vec<Event>,
) -> Vec<Event> {
    // Load the events into the dummy/test server.
    server.set_events(events.clone());

    // Add "type" to extra fields to ensure equality checking works as expected.
    // In theory this should not be needed, I assume there is some funkyness going on with the fact
    // this field is renamed by serde.
    for event in events.iter_mut() {
        event.extra = HashMap::from([("type".to_string(), Value::String("talk".to_string()))]);
    }

    // Return the patched events for comparisons.
    events
}
