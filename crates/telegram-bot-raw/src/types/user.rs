//! Telegram [`User`] type — ported from `python-telegram-bot/src/telegram/_user.py`.
//!
//! Only data fields are included. No Bot reference, no API shortcuts.

use serde::{Deserialize, Serialize};

/// This object represents a Telegram user or bot.
///
/// Corresponds to the Bot API [`User`](https://core.telegram.org/bots/api#user) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl User {
    /// Create a new `User` with the required fields.
    ///
    /// All optional fields default to `None`.
    pub fn new(id: i64, is_bot: bool, first_name: impl Into<String>) -> Self {
        Self {
            id,
            is_bot,
            first_name: first_name.into(),
            last_name: None,
            username: None,
            language_code: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_inline_queries: None,
            is_premium: None,
            added_to_attachment_menu: None,
            can_connect_to_business: None,
            has_main_web_app: None,
            has_topics_enabled: None,
            allows_users_to_create_topics: None,
            can_manage_bots: None,
        }
    }

    /// Returns the user's full name (first + optional last name).
    pub fn full_name(&self) -> String {
        match &self.last_name {
            Some(ln) => format!("{} {}", self.first_name, ln),
            None => self.first_name.clone(),
        }
    }

    /// Returns the deep link URL to this user's profile.
    pub fn link(&self) -> String {
        format!("tg://user?id={}", self.id)
    }

    /// Returns an HTML `<a>` tag that mentions this user.
    ///
    /// If `name` is `None`, the user's first name is used as the display text.
    pub fn mention_html(&self, name: Option<&str>) -> String {
        let display = name.unwrap_or(&self.first_name);
        format!("<a href=\"tg://user?id={}\">{}</a>", self.id, display)
    }

    /// Returns a Markdown link that mentions this user.
    ///
    /// If `name` is `None`, the user's first name is used as the display text.
    pub fn mention_markdown(&self, name: Option<&str>) -> String {
        let display = name.unwrap_or(&self.first_name);
        format!("[{}](tg://user?id={})", display, self.id)
    }

    /// Returns a MarkdownV2-safe link that mentions this user.
    ///
    /// If `name` is `None`, the user's first name is used as the display text.
    /// Note: The caller is responsible for escaping `name` for MarkdownV2.
    pub fn mention_markdown_v2(&self, name: Option<&str>) -> String {
        let display = name.unwrap_or(&self.first_name);
        format!("[{}](tg://user?id={})", display, self.id)
    }
}
