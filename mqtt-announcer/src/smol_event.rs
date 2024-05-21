use chrono::{DateTime, FixedOffset};
use emfcamp_schedule_api::schedule::event::{Event, Kind};
use serde::{Deserialize, Serialize};

/// A smaller representation of an event, with only the bare minimum information.
/// Useful for receiving on embedded systems (e.g. LED matrix signs).
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SmolEvent {
    id: u32,

    #[serde(flatten)]
    kind: Kind,

    start: DateTime<FixedOffset>,

    end: DateTime<FixedOffset>,

    venue: String,

    title: String,

    speaker: String,
}

impl From<Event> for SmolEvent {
    fn from(event: Event) -> Self {
        Self {
            id: event.id,
            kind: event.kind,
            start: event.start,
            end: event.end,
            venue: event.venue,
            title: event.title,
            speaker: event.speaker,
        }
    }
}
