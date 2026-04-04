use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a bot command (text + description pair).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommand {
    /// Text of the command; 1-32 characters, lowercase letters, digits and underscores.
    pub command: String,

    /// Description of the command; 1-256 characters.
    pub description: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
