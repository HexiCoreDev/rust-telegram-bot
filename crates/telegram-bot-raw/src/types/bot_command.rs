use serde::{Deserialize, Serialize};

/// Represents a bot command (text + description pair).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct BotCommand {
    /// Text of the command; 1-32 characters, lowercase letters, digits and underscores.
    pub command: String,

    /// Description of the command; 1-256 characters.
    pub description: String,
}

impl_new!(BotCommand {
    command: String,
    description: String
});
