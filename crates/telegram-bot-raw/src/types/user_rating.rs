use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Rating of a user based on their Telegram Star spendings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserRating {
    /// Current level of the user. Higher means more trustworthy; negative is a concern.
    pub level: i64,

    /// Numerical value of the user's rating. Higher is better.
    pub rating: i64,

    /// Rating value required to reach the current level.
    pub current_level_rating: i64,

    /// Rating value required to reach the next level. Absent if the maximum level was reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_level_rating: Option<i64>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
