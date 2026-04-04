//! Telegram [`Chat`] type — ported from `python-telegram-bot/src/telegram/_chat.py`.
//!
//! `Chat` is the lightweight identifier object used throughout API responses.
//! Full chat details live in [`ChatFullInfo`](super::chat_full_info::ChatFullInfo).
//! Only data fields are included. No Bot reference, no API shortcuts.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// This object represents a chat.
///
/// Corresponds to the Bot API [`Chat`](https://core.telegram.org/bots/api#chat) object.
///
/// As of Bot API 7.3 most extended fields have moved to
/// [`ChatFullInfo`](super::chat_full_info::ChatFullInfo).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chat {
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

    /// Catch-all for any extra fields returned by the Bot API not yet modelled here.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
