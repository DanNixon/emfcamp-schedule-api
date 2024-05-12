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
use tokio::time::{Duration, Instant};

fn set_and_patch_dummy_events(server: &mut DummyScheduleServer, events: Vec<Event>) -> Vec<Event> {
    // Load the events into the dummy/test server.
    server.set_events(events.clone());

    // Return the patched events for comparisons.
    events
}
