use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::user::User;

/// One special entity in a text message (hashtag, username, URL, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Type of the entity: `mention`, `hashtag`, `cashtag`, `bot_command`, `url`, `email`,
    /// `phone_number`, `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, `blockquote`,
    /// `expandable_blockquote`, `code`, `pre`, `text_link`, `text_mention`, `custom_emoji`,
    /// or `date_time`.
    #[serde(rename = "type")]
    pub entity_type: String,

    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: i64,

    /// Length of the entity in UTF-16 code units.
    pub length: i64,

    /// For `text_link` only: URL that will be opened after the user taps on the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// For `text_mention` only: the mentioned user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// For `pre` only: the programming language of the entity text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// For `custom_emoji` only: unique identifier of the custom emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,

    /// For `date_time` only: the string defining the formatting of the date and time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_format: Option<String>,

    /// For `date_time` only: Unix timestamp associated with the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unix_time: Option<i64>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
