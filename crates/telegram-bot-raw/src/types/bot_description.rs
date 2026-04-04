use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The bot's full description.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotDescription {
    /// The bot's description.
    pub description: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The bot's short description (shown on the profile page).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotShortDescription {
    /// The bot's short description.
    pub short_description: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
