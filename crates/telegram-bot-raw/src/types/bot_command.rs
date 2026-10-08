use serde::{Deserialize, Serialize};

/// Represents a bot command (text + description pair).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct BotCommand {
    /// Text of the command; 1-32 characters, lowercase letters, digits and underscores.
    pub command: String,

    /// Description of the command; 1-256 characters.
    pub description: String,

    /// `true` if the command sends an ephemeral message, visible only to the
    /// sender and the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ephemeral: Option<bool>,
}

impl_new!(BotCommand {
    command: String,
    description: String
});
