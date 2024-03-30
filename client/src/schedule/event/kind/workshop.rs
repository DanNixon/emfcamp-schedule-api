use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Workshop {
    pub cost: String,

    pub equiptment: Option<String>,

    pub age_range: String,

    pub attendees: Option<String>,
}
