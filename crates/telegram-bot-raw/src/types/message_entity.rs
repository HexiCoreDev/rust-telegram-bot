use serde::{Deserialize, Serialize};

use super::user::User;

/// One special entity in a text message (hashtag, username, URL, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
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
}

impl MessageEntity {
    /// Creates a new `MessageEntity` with the given type, offset, and length.
    pub fn new(entity_type: impl Into<String>, offset: i64, length: i64) -> Self {
        Self {
            entity_type: entity_type.into(),
            offset,
            length,
            ..Default::default()
        }
    }

    // ── Convenience constructors for common entity types ──

    /// Creates a **bold** entity.
    pub fn bold(offset: i64, length: i64) -> Self {
        Self::new("bold", offset, length)
    }

    /// Creates an *italic* entity.
    pub fn italic(offset: i64, length: i64) -> Self {
        Self::new("italic", offset, length)
    }

    /// Creates an underline entity.
    pub fn underline(offset: i64, length: i64) -> Self {
        Self::new("underline", offset, length)
    }

    /// Creates a ~~strikethrough~~ entity.
    pub fn strikethrough(offset: i64, length: i64) -> Self {
        Self::new("strikethrough", offset, length)
    }

    /// Creates a spoiler entity.
    pub fn spoiler(offset: i64, length: i64) -> Self {
        Self::new("spoiler", offset, length)
    }

    /// Creates a `code` (inline monospace) entity.
    pub fn code(offset: i64, length: i64) -> Self {
        Self::new("code", offset, length)
    }

    /// Creates a `pre` (code block) entity with an optional language.
    pub fn pre(offset: i64, length: i64, language: Option<impl Into<String>>) -> Self {
        Self {
            language: language.map(Into::into),
            ..Self::new("pre", offset, length)
        }
    }

    /// Creates a `text_link` entity with a URL.
    pub fn text_link(offset: i64, length: i64, url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            ..Self::new("text_link", offset, length)
        }
    }

    /// Creates a `text_mention` entity for a user without a username.
    pub fn text_mention(offset: i64, length: i64, user: User) -> Self {
        Self {
            user: Some(user),
            ..Self::new("text_mention", offset, length)
        }
    }

    /// Creates a `custom_emoji` entity.
    pub fn custom_emoji(offset: i64, length: i64, custom_emoji_id: impl Into<String>) -> Self {
        Self {
            custom_emoji_id: Some(custom_emoji_id.into()),
            ..Self::new("custom_emoji", offset, length)
        }
    }

    /// Creates a `blockquote` entity.
    pub fn blockquote(offset: i64, length: i64) -> Self {
        Self::new("blockquote", offset, length)
    }

    /// Creates an `expandable_blockquote` entity.
    pub fn expandable_blockquote(offset: i64, length: i64) -> Self {
        Self::new("expandable_blockquote", offset, length)
    }

    /// Creates a `mention` entity (@username).
    pub fn mention(offset: i64, length: i64) -> Self {
        Self::new("mention", offset, length)
    }

    /// Creates a `hashtag` entity.
    pub fn hashtag(offset: i64, length: i64) -> Self {
        Self::new("hashtag", offset, length)
    }

    /// Creates a `cashtag` entity.
    pub fn cashtag(offset: i64, length: i64) -> Self {
        Self::new("cashtag", offset, length)
    }

    /// Creates a `bot_command` entity.
    pub fn bot_command(offset: i64, length: i64) -> Self {
        Self::new("bot_command", offset, length)
    }

    /// Creates a `url` entity.
    pub fn url(offset: i64, length: i64) -> Self {
        Self::new("url", offset, length)
    }

    /// Creates an `email` entity.
    pub fn email(offset: i64, length: i64) -> Self {
        Self::new("email", offset, length)
    }

    /// Creates a `phone_number` entity.
    pub fn phone_number(offset: i64, length: i64) -> Self {
        Self::new("phone_number", offset, length)
    }
}
