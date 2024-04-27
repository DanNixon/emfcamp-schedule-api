mod workshop;

pub use self::workshop::Workshop;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Kind {
    // TODO
    Talk,

    Workshop(Workshop),

    // TODO
    YouthWorkshop,

    // TODO
    Performance,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Kind::Talk => write!(f, "Talk"),
            Kind::Workshop(_) => write!(f, "Workshop"),
            Kind::YouthWorkshop => write!(f, "Youth Workshop"),
            Kind::Performance => write!(f, "Performance"),
        }
    }
}
