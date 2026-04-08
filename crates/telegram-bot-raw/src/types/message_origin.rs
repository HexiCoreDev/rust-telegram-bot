
use serde::{Deserialize, Serialize};

use super::chat::Chat;
use super::user::User;

/// Payload for `MessageOriginUser`: the message was originally sent by a known user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageOriginUserData {
    /// Date the message was sent originally, as a Unix timestamp.
    pub date: i64,

    /// User that sent the message originally.
    pub sender_user: User,
}

/// Payload for `MessageOriginHiddenUser`: the message was originally sent by an unknown user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageOriginHiddenUserData {
    /// Date the message was sent originally, as a Unix timestamp.
    pub date: i64,

    /// Name of the user that sent the message originally.
    pub sender_user_name: String,
}

/// Payload for `MessageOriginChat`: the message was originally sent on behalf of a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageOriginChatData {
    /// Date the message was sent originally, as a Unix timestamp.
    pub date: i64,

    /// Chat that sent the message originally.
    pub sender_chat: Chat,

    /// For messages originally sent by an anonymous chat administrator, the author signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}

/// Payload for `MessageOriginChannel`: the message was originally sent to a channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageOriginChannelData {
    /// Date the message was sent originally, as a Unix timestamp.
    pub date: i64,

    /// Channel chat to which the message was originally sent.
    pub chat: Chat,

    /// Unique message identifier inside the chat.
    pub message_id: i64,

    /// Signature of the original post author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}

/// Origin of a message, selected by the `"type"` field in the JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageOrigin {
    /// Sent by a known user.
    User(MessageOriginUserData),

    /// Sent by an unknown user.
    HiddenUser(MessageOriginHiddenUserData),

    /// Sent on behalf of a chat.
    Chat(MessageOriginChatData),

    /// Originally sent to a channel.
    Channel(MessageOriginChannelData),
}
