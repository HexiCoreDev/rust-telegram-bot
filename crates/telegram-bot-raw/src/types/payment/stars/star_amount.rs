use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An amount of Telegram Stars.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarAmount {
    /// Integer amount of Telegram Stars, rounded to 0; can be negative.
    pub amount: i64,

    /// Fractional nanostar shares; from 0 to 999999999. Can be negative if `amount` is non-positive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
