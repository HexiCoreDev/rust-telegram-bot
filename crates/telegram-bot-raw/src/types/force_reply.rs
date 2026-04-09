use serde::{Deserialize, Serialize};

/// Upon receiving a message with this object, Telegram clients will display a reply interface to
/// the user (act as if the user has selected the bot's message and tapped 'Reply').
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
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

impl ForceReply {
    /// Creates a new `ForceReply` with `force_reply` set to `true`.
    pub fn new() -> Self {
        Self {
            force_reply: true,
            ..Default::default()
        }
    }

    /// Show the force reply to specific users only.
    pub fn selective(mut self) -> Self {
        self.selective = Some(true);
        self
    }

    /// Set a placeholder shown in the input field when the reply is active.
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.input_field_placeholder = Some(text.into());
        self
    }
}
