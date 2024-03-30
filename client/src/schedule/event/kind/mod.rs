mod workshop;

use serde::{Deserialize, Serialize};

pub use self::workshop::Workshop;

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
