//! Telegram [`User`] type — ported from `python-telegram-bot/src/telegram/_user.py`.
//!
//! Only data fields are included. No Bot reference, no API shortcuts.

use serde::{Deserialize, Serialize};

/// This object represents a Telegram user or bot.
///
/// Corresponds to the Bot API [`User`](https://core.telegram.org/bots/api#user) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: i64,

    /// `true` if this user is a bot.
    pub is_bot: bool,

    /// User's or bot's first name.
    pub first_name: String,

    /// User's or bot's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// User's or bot's username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// IETF language tag of the user's language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// `true` if the bot can be invited to groups.
    /// Returned only in `getMe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,

    /// `true` if privacy mode is disabled for the bot.
    /// Returned only in `getMe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,

    /// `true` if the bot supports inline queries.
    /// Returned only in `getMe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,

    /// `true` if this user is a Telegram Premium user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,

    /// `true` if this user added the bot to the attachment menu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,

    /// `true` if the bot can be connected to a Telegram Business account.
    /// Returned only in `getMe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_connect_to_business: Option<bool>,

    /// `true` if the bot has the main Web App.
    /// Returned only in `getMe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_main_web_app: Option<bool>,

    /// `true` if the bot has forum topic mode enabled in private chats.
    /// Returned only in `getMe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_topics_enabled: Option<bool>,

    /// `true` if the bot allows users to create and delete topics in private chats.
    /// Returned only in `getMe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_users_to_create_topics: Option<bool>,

    /// `true` if the bot can manage other bots.
    /// Returned only in `getMe`.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_bots: Option<bool>,
}
