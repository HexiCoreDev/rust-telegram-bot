//! Telegram [`ChatFullInfo`] type — ported from
//! `python-telegram-bot/src/telegram/_chatfullinfo.py`.
//!
//! `ChatFullInfo` is returned by `getChat` and carries the complete set of chat metadata.
//! Only data fields are included. No Bot reference, no API shortcuts.

use serde::{Deserialize, Serialize};

use super::birthdate::Birthdate;
use super::business::{BusinessIntro, BusinessLocation, BusinessOpeningHours};
use super::chat::Chat;
use super::chat_location::ChatLocation;
use super::chat_permissions::ChatPermissions;
use super::files::audio::Audio;
use super::files::chat_photo::ChatPhoto;
use super::gifts::AcceptedGiftTypes;
use super::message::Message;
use super::reaction::ReactionType;
use super::unique_gift::UniqueGiftColors;
use super::user_rating::UserRating;

/// This object contains full information about a chat.
///
/// Corresponds to the Bot API [`ChatFullInfo`](https://core.telegram.org/bots/api#chatfullinfo)
/// object (returned by `getChat`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatFullInfo {
    // ── Fields inherited from _ChatBase ──────────────────────────────────────
    /// Unique identifier for this chat.
    pub id: i64,

    /// Type of chat: `"private"`, `"group"`, `"supergroup"` or `"channel"`.
    #[serde(rename = "type")]
    pub chat_type: String,

    /// Title, for supergroups, channels and group chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Username, for private chats, supergroups and channels if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// First name of the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Last name of the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// `true` if the supergroup chat is a forum (has topics enabled).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,

    /// `true` if the chat is the direct messages chat of a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_direct_messages: Option<bool>,

    // ── Fields unique to ChatFullInfo ─────────────────────────────────────────
    /// The maximum number of reactions that can be set on a message in the chat.
    pub max_reaction_count: i32,

    /// Identifier of the accent color for the chat name and backgrounds of
    /// the chat photo, reply header, and link preview.
    pub accent_color_id: i32,

    /// Types of gifts accepted by the chat.
    pub accepted_gift_types: AcceptedGiftTypes,

    /// Chat photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,

    /// List of all active chat usernames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,

    /// For private chats, the date of birth of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<Birthdate>,

    /// For private chats with business accounts, the intro of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<BusinessIntro>,

    /// For private chats with business accounts, the location of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_location: Option<BusinessLocation>,

    /// For private chats with business accounts, the opening hours of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_opening_hours: Option<BusinessOpeningHours>,

    /// For private chats, the personal channel of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_chat: Option<Box<Chat>>,

    /// List of available reactions allowed in the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Vec<ReactionType>>,

    /// Custom emoji identifier of the emoji chosen by the chat for the reply header
    /// and link preview background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<String>,

    /// Identifier of the accent color for the chat's profile background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_accent_color_id: Option<i32>,

    /// Custom emoji identifier of the emoji chosen by the chat for its profile background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_background_custom_emoji_id: Option<String>,

    /// Custom emoji identifier of the emoji status of the chat or the other party in a
    /// private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,

    /// Expiration date of the emoji status, in Unix time, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,

    /// Bio of the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    /// `true` if privacy settings of the other party restrict `tg://user?id=<user_id>` links
    /// to chats with the user only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,

    /// `true` if the privacy settings of the other party restrict sending voice and video
    /// note messages in the private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,

    /// `true` if users need to join the supergroup before they can send messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,

    /// `true` if all users directly joining the supergroup need to be approved by
    /// supergroup administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,

    /// Description, for groups, supergroups and channel chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Primary invite link, for groups, supergroups and channel chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,

    /// The most recent pinned message (by sending date).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,

    /// Default chat member permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,

    /// Delay (in seconds) between consecutive messages sent by each unprivileged user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i64>,

    /// Minimum number of boosts a non-administrator user needs to add in order to ignore
    /// slow mode and chat permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<i32>,

    /// The time (in seconds) after which all messages sent to the chat will be automatically
    /// deleted; 0 if auto-delete is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i64>,

    /// `true` if aggressive anti-spam checks are enabled in the supergroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,

    /// `true` if non-administrators can only get the list of bots and administrators in
    /// the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,

    /// `true` if messages from the chat can't be forwarded to other chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,

    /// `true` if new chat members will have access to old messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,

    /// For supergroups, name of the group sticker set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,

    /// `true` if the bot can change the group sticker set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,

    /// For supergroups, the name of the group's custom emoji sticker set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<String>,

    /// Unique identifier for the linked chat (discussion group for a channel and vice versa).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,

    /// For supergroups, the location to which the supergroup is connected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,

    /// `true` if paid media messages can be sent or forwarded to the channel chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_paid_media: Option<bool>,

    /// The parent chat of a channel's direct messages chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_chat: Option<Box<Chat>>,

    /// Rating of the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<UserRating>,

    /// Colors of unique gifts accepted by the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_colors: Option<UniqueGiftColors>,

    /// The number of Telegram Stars that must be paid for each message sent to the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_message_star_count: Option<i32>,

    /// First audio in the profile of the chat or the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_profile_audio: Option<Audio>,
}
