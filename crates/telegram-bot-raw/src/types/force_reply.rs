
use serde::{Deserialize, Serialize};

/// Upon receiving a message with this object, Telegram clients will display a reply interface to
/// the user (act as if the user has selected the bot's message and tapped 'Reply').
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot's message and
    /// tapped 'Reply'. Always `true`.
    pub force_reply: bool,

    /// If `true`, force reply only from specific @mentioned users or the sender of the message
    /// being replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,

    /// Placeholder shown in the input field when the reply is active; 1-64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
}
