use serde::{Deserialize, Serialize};

use super::chat::Chat;
use super::chat_invite_link::ChatInviteLink;
use super::chat_member::ChatMember;
use super::user::User;

/// Represents changes in the status of a chat member.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to.
    pub chat: Chat,
    /// Performer of the action which resulted in the change.
    ///
    /// Renamed from the API field `"from"` which is a reserved keyword in Rust.
    #[serde(rename = "from")]
    pub from_user: User,
    /// Unix timestamp of when the change was done.
    pub date: i64,
    /// Previous information about the chat member.
    pub old_chat_member: ChatMember,
    /// New information about the chat member.
    pub new_chat_member: ChatMember,

    /// Chat invite link which was used by the user to join the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
    /// True if the user joined the chat via a chat folder invite link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
    /// True if the user joined the chat after sending a direct join request approved by an admin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<bool>,
}
