
use serde::{Deserialize, Serialize};

/// Describes actions that a non-administrator user is allowed to take in a chat.
///
/// All fields are optional — when absent the permission is unset (inherits from defaults).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatPermissions {
    /// True if the user is allowed to send text messages, contacts, locations and venues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    /// True if the user is allowed to send polls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    /// True if the user is allowed to send animations, games, stickers and use inline bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    /// True if the user is allowed to add web page previews to their messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    /// True if the user is allowed to change the chat title, photo and other settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// True if the user is allowed to invite new users to the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// True if the user is allowed to pin messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// True if the user is allowed to create forum topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// True if the user is allowed to send audios.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,
    /// True if the user is allowed to send documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,
    /// True if the user is allowed to send photos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,
    /// True if the user is allowed to send videos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,
    /// True if the user is allowed to send video notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,
    /// True if the user is allowed to send voice notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,
    /// True if the user is allowed to edit their own tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_tag: Option<bool>,
}
