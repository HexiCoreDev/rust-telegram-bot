use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An animated emoji with a random value, as returned by the Telegram Bot API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dice {
    /// Value of the dice.
    pub value: i64,

    /// Emoji on which the dice throw animation is based.
    pub emoji: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
