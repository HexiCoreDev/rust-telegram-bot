use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The bot's display name.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotName {
    /// The bot's name; up to 64 characters.
    pub name: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
