use serde::{Deserialize, Serialize};

use super::chat::Chat;
use super::chat_invite_link::ChatInviteLink;
use super::user::User;

/// Represents a join request sent to a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent.
    pub chat: Chat,
    /// User that sent the join request.
    ///
    /// Renamed from the API field `"from"` which is a reserved keyword in Rust.
    #[serde(rename = "from")]
    pub from_user: User,
    /// Unix timestamp of when the request was sent.
    pub date: i64,
    /// Identifier of a private chat with the user who sent the join request.
    pub user_chat_id: i64,

    /// Bio of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}
