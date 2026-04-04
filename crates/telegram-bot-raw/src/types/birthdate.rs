use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Describes the birthdate of a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Birthdate {
    /// Day of the user's birth; 1-31.
    pub day: i64,

    /// Month of the user's birth; 1-12.
    pub month: i64,

    /// Year of the user's birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
