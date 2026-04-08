
use serde::{Deserialize, Serialize};

use super::user::User;

// ---------------------------------------------------------------------------
// ChatMember — tagged union on the "status" field (wire values)
// ---------------------------------------------------------------------------

/// Represents a chat member that owns the chat and has all administrator privileges.
///
/// Wire value: `"creator"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberOwner {
    /// Information about the user.
    pub user: User,
    /// True if the user's presence in the chat is hidden.
    pub is_anonymous: bool,

    /// Custom title for this user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

/// Represents a chat member that has some additional privileges.
///
/// Wire value: `"administrator"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberAdministrator {
    /// Information about the user.
    pub user: User,
    /// True if the bot is allowed to edit administrator privileges of that user.
    pub can_be_edited: bool,
    /// True if the user's presence in the chat is hidden.
    pub is_anonymous: bool,
    /// True if the administrator can access the chat event log and ignore slow mode.
    pub can_manage_chat: bool,
    /// True if the administrator can delete messages of other users.
    pub can_delete_messages: bool,
    /// True if the administrator can manage video chats.
    pub can_manage_video_chats: bool,
    /// True if the administrator can restrict, ban or unban chat members.
    pub can_restrict_members: bool,
    /// True if the administrator can add new administrators with a subset of their own privileges.
    pub can_promote_members: bool,
    /// True if the user can change the chat title, photo and other settings.
    pub can_change_info: bool,
    /// True if the user can invite new users to the chat.
    pub can_invite_users: bool,
    /// True if the administrator can post stories to the chat.
    pub can_post_stories: bool,
    /// True if the administrator can edit stories posted by other users.
    pub can_edit_stories: bool,
    /// True if the administrator can delete stories posted by other users.
    pub can_delete_stories: bool,

    /// True if the administrator can post messages in the channel; channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// True if the administrator can edit messages of other users; channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// True if the user is allowed to pin messages; groups and supergroups only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// True if the user is allowed to create, rename, close and reopen forum topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// Custom title for this user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
    /// True if the administrator can manage direct messages of the channel; channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_direct_messages: Option<bool>,
    /// True if the administrator can edit the tags of regular members; groups and supergroups only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_tags: Option<bool>,
}

/// Represents a chat member that has no additional privileges or restrictions.
///
/// Wire value: `"member"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberMember {
    /// Information about the user.
    pub user: User,

    /// Unix timestamp of when the user's subscription will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    /// Tag of the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Represents a chat member that is under certain restrictions in the chat (supergroups only).
///
/// Wire value: `"restricted"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberRestricted {
    /// Information about the user.
    pub user: User,
    /// True if the user is a member of the chat at the moment of the request.
    pub is_member: bool,
    /// True if the user can change the chat title, photo and other settings.
    pub can_change_info: bool,
    /// True if the user can invite new users to the chat.
    pub can_invite_users: bool,
    /// True if the user is allowed to pin messages.
    pub can_pin_messages: bool,
    /// True if the user is allowed to send text messages, contacts, invoices, locations and venues.
    pub can_send_messages: bool,
    /// True if the user is allowed to send polls.
    pub can_send_polls: bool,
    /// True if the user is allowed to send animations, games, stickers and use inline bots.
    pub can_send_other_messages: bool,
    /// True if the user is allowed to add web page previews to their messages.
    pub can_add_web_page_previews: bool,
    /// True if the user is allowed to create forum topics.
    pub can_manage_topics: bool,
    /// Unix timestamp of when restrictions will be lifted for this user.
    pub until_date: i64,
    /// True if the user is allowed to send audios.
    pub can_send_audios: bool,
    /// True if the user is allowed to send documents.
    pub can_send_documents: bool,
    /// True if the user is allowed to send photos.
    pub can_send_photos: bool,
    /// True if the user is allowed to send videos.
    pub can_send_videos: bool,
    /// True if the user is allowed to send video notes.
    pub can_send_video_notes: bool,
    /// True if the user is allowed to send voice notes.
    pub can_send_voice_notes: bool,
    /// True if the user is allowed to edit their own tag.
    pub can_edit_tag: bool,

    /// Tag of the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Represents a chat member that isn't currently a member of the chat but may join.
///
/// Wire value: `"left"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberLeft {
    /// Information about the user.
    pub user: User,
}

/// Represents a chat member that was banned and cannot return to or view the chat.
///
/// Wire value: `"kicked"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberBanned {
    /// Information about the user.
    pub user: User,
    /// Unix timestamp of when restrictions will be lifted for this user.
    pub until_date: i64,
}

/// Represents a member of a chat. Discriminated by the `"status"` field.
///
/// The Telegram API wire values are: `"creator"`, `"administrator"`, `"member"`,
/// `"restricted"`, `"left"`, `"kicked"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ChatMember {
    #[serde(rename = "creator")]
    Owner(ChatMemberOwner),
    #[serde(rename = "administrator")]
    Administrator(ChatMemberAdministrator),
    #[serde(rename = "member")]
    Member(ChatMemberMember),
    #[serde(rename = "restricted")]
    Restricted(ChatMemberRestricted),
    #[serde(rename = "left")]
    Left(ChatMemberLeft),
    #[serde(rename = "kicked")]
    Banned(ChatMemberBanned),
}

// ---------------------------------------------------------------------------
// ChatOwnerChanged / ChatOwnerLeft — from _chatowner.py
// These are service message types distinct from the ChatMember status system.
// ---------------------------------------------------------------------------

/// Service message about an ownership change in the chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatOwnerChanged {
    /// The new owner of the chat.
    pub new_owner: User,
}

/// Service message about the chat owner leaving the chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatOwnerLeft {
    /// The user which will be the new owner of the chat if the previous owner does not return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_owner: Option<User>,
}
