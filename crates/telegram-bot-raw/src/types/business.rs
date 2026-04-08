
use serde::{Deserialize, Serialize};

use super::chat::Chat;
use super::files::location::Location;
use super::files::sticker::Sticker;
use super::user::User;

/// Rights of a business bot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessBotRights {
    /// True if the bot can send and edit messages in private chats with incoming messages
    /// in the last 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_reply: Option<bool>,

    /// True if the bot can mark incoming private messages as read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_messages: Option<bool>,

    /// True if the bot can delete messages sent by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_sent_messages: Option<bool>,

    /// True if the bot can delete all private messages in managed chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_all_messages: Option<bool>,

    /// True if the bot can edit the first and last name of the business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_name: Option<bool>,

    /// True if the bot can edit the bio of the business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_bio: Option<bool>,

    /// True if the bot can edit the profile photo of the business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_profile_photo: Option<bool>,

    /// True if the bot can edit the username of the business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_username: Option<bool>,

    /// True if the bot can change gift privacy settings of the business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_gift_settings: Option<bool>,

    /// True if the bot can view gifts and the Stars balance of the business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view_gifts_and_stars: Option<bool>,

    /// True if the bot can convert regular gifts owned by the business account to Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_convert_gifts_to_stars: Option<bool>,

    /// True if the bot can transfer and upgrade gifts owned by the business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_transfer_and_upgrade_gifts: Option<bool>,

    /// True if the bot can transfer Stars received by the business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_transfer_stars: Option<bool>,

    /// True if the bot can post, edit, and delete stories on behalf of the business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_stories: Option<bool>,
}

/// Connection of a bot with a business account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessConnection {
    /// Unique identifier of the business connection.
    pub id: String,

    /// Business account user that created the connection.
    pub user: User,

    /// Identifier of a private chat with the user who created the connection.
    pub user_chat_id: i64,

    /// Unix timestamp when the connection was established.
    pub date: i64,

    /// True if the connection is active.
    pub is_enabled: bool,

    /// Rights of the business bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<BusinessBotRights>,
}

/// Service message received when messages are deleted from a connected business account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection.
    pub business_connection_id: String,

    /// Information about the chat in the business account.
    pub chat: Chat,

    /// Identifiers of the deleted messages in the chat.
    pub message_ids: Vec<i64>,
}

/// Start page settings of a Telegram Business account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessIntro {
    /// Title text of the business intro.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Message text of the business intro.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Sticker of the business intro.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
}

/// Location information of a Telegram Business account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessLocation {
    /// Address of the business.
    pub address: String,

    /// Location of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// A single time interval during which a business is open.
///
/// Minutes are counted from the start of the week (Monday = 0).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessOpeningHoursInterval {
    /// Minute of the week marking the opening time; 0 – 7×24×60.
    pub opening_minute: i64,

    /// Minute of the week marking the closing time; 0 – 8×24×60.
    pub closing_minute: i64,
}

/// Opening hours of a business.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessOpeningHours {
    /// IANA timezone name for which the opening hours are defined.
    pub time_zone_name: String,

    /// Time intervals describing business opening hours.
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
