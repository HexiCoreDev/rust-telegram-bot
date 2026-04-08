use serde::{Deserialize, Serialize};

/// The bot's display name.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotName {
    /// The bot's name; up to 64 characters.
    pub name: String,
}
