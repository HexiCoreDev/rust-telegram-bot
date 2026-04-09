//! Telegram [`Chat`] type — ported from `python-telegram-bot/src/telegram/_chat.py`.
//!
//! `Chat` is the lightweight identifier object used throughout API responses.
//! Full chat details live in [`ChatFullInfo`](super::chat_full_info::ChatFullInfo).
//! Only data fields are included. No Bot reference, no API shortcuts.

use serde::{Deserialize, Serialize};

/// This object represents a chat.
///
/// Corresponds to the Bot API [`Chat`](https://core.telegram.org/bots/api#chat) object.
///
/// As of Bot API 7.3 most extended fields have moved to
/// [`ChatFullInfo`](super::chat_full_info::ChatFullInfo).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
}

impl Chat {
    /// Returns `true` if this is a private (one-on-one) chat.
    pub fn is_private(&self) -> bool {
        self.chat_type == "private"
    }

    /// Returns `true` if this is a group chat.
    pub fn is_group(&self) -> bool {
        self.chat_type == "group"
    }

    /// Returns `true` if this is a supergroup chat.
    pub fn is_supergroup(&self) -> bool {
        self.chat_type == "supergroup"
    }

    /// Returns `true` if this is a channel.
    pub fn is_channel(&self) -> bool {
        self.chat_type == "channel"
    }

    /// Returns the chat's effective display name.
    ///
    /// For groups/supergroups/channels this is the title; for private chats it is the
    /// other party's first name. Returns `None` if neither is available.
    pub fn effective_name(&self) -> Option<&str> {
        self.title.as_deref().or(self.first_name.as_deref())
    }

    /// Returns the full name for private chats (first + optional last name),
    /// or the title for groups/channels.
    pub fn full_name(&self) -> Option<String> {
        if let Some(ref title) = self.title {
            return Some(title.clone());
        }
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => Some(format!("{first} {last}")),
            (Some(first), None) => Some(first.clone()),
            _ => None,
        }
    }
}
