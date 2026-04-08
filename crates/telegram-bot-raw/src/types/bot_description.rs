
use serde::{Deserialize, Serialize};

/// The bot's full description.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotDescription {
    /// The bot's description.
    pub description: String,
}

/// The bot's short description (shown on the profile page).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotShortDescription {
    /// The bot's short description.
    pub short_description: String,
}
