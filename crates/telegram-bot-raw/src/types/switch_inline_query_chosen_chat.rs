
use serde::{Deserialize, Serialize};

/// An inline button that switches the user to inline mode in a chosen chat.
///
/// At least one of the `allow_*` fields must be `true`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    /// Default inline query inserted into the input field; empty string means bot username only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// If `true`, private chats with users can be chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,

    /// If `true`, private chats with bots can be chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,

    /// If `true`, group and supergroup chats can be chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,

    /// If `true`, channel chats can be chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}
