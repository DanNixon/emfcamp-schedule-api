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

fn fixup_events_for_test_comparison(events: &mut [Event]) {
    for event in events {
        event.extra = HashMap::from([("type".to_string(), Value::String("talk".to_string()))]);
    }
}
