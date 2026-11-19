use serde::{Deserialize, Serialize};

use super::unique_gift::{UniqueGift, UniqueGiftBackdropColors};

// ---------------------------------------------------------------------------
// ChatShareTheme
// ---------------------------------------------------------------------------

/// The theme with emoji placed on a background.
///
/// Corresponds to the Bot API
/// [`ChatShareThemeEmoji`](https://core.telegram.org/bots/api#chatsharethemeemoji) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct ChatShareThemeEmoji {
    /// The emoji; 0-6 regular or custom emoji.
    pub emojis: Vec<EmojiType>,

    /// Identifier of the backdrop used as the background; omitted if the default background is
    /// used.
    ///
    /// Use [`Bot::get_chat_share_theme_backdrops`](crate::bot::Bot::get_chat_share_theme_backdrops)
    /// to get details about the backdrop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_backdrop_id: Option<i64>,
}

/// The theme based on a unique gift owned by the user or the chat.
///
/// Corresponds to the Bot API
/// [`ChatShareThemeUniqueGift`](https://core.telegram.org/bots/api#chatsharethemeuniquegift) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatShareThemeUniqueGift {
    /// The gift.
    ///
    /// Boxed: a `UniqueGift` is over 1 KiB and the emoji theme variant is far smaller, so
    /// indirection keeps the `ChatShareTheme` enum compact (clippy::large_enum_variant).
    pub gift: Box<UniqueGift>,
}

/// The theme of the screen for sharing a link to a user or a chat.
///
/// Serialized with a `"type"` tag that selects the variant.
///
/// Corresponds to the Bot API
/// [`ChatShareTheme`](https://core.telegram.org/bots/api#chatsharetheme) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChatShareTheme {
    /// The theme with emoji placed on a background.
    Emoji(ChatShareThemeEmoji),

    /// The theme based on a unique gift owned by the user or the chat.
    UniqueGift(ChatShareThemeUniqueGift),
}

impl ChatShareTheme {
    /// Creates a theme with the given emoji.
    pub fn emoji(emojis: Vec<EmojiType>) -> Self {
        Self::Emoji(ChatShareThemeEmoji {
            emojis,
            background_backdrop_id: None,
        })
    }

    /// Creates a theme based on a unique gift.
    pub fn unique_gift(gift: UniqueGift) -> Self {
        Self::UniqueGift(ChatShareThemeUniqueGift {
            gift: Box::new(gift),
        })
    }
}

// ---------------------------------------------------------------------------
// EmojiType
// ---------------------------------------------------------------------------

/// A regular emoji.
///
/// Corresponds to the Bot API
/// [`EmojiTypeEmoji`](https://core.telegram.org/bots/api#emojitypeemoji) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct EmojiTypeEmoji {
    /// The emoji.
    pub emoji: String,
}

/// A custom emoji.
///
/// Corresponds to the Bot API
/// [`EmojiTypeCustomEmoji`](https://core.telegram.org/bots/api#emojitypecustomemoji) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct EmojiTypeCustomEmoji {
    /// Identifier of the custom emoji. Use `getCustomEmojiStickers` to get full information
    /// about the sticker.
    pub custom_emoji_id: String,
}

/// Describes an emoji.
///
/// Serialized with a `"type"` tag that selects the variant.
///
/// Corresponds to the Bot API [`EmojiType`](https://core.telegram.org/bots/api#emojitype) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum EmojiType {
    /// A regular emoji.
    Emoji(EmojiTypeEmoji),

    /// A custom emoji.
    CustomEmoji(EmojiTypeCustomEmoji),
}

impl EmojiType {
    /// Creates a regular emoji.
    pub fn emoji(emoji: impl Into<String>) -> Self {
        Self::Emoji(EmojiTypeEmoji {
            emoji: emoji.into(),
        })
    }

    /// Creates a custom emoji.
    pub fn custom_emoji(custom_emoji_id: impl Into<String>) -> Self {
        Self::CustomEmoji(EmojiTypeCustomEmoji {
            custom_emoji_id: custom_emoji_id.into(),
        })
    }
}

// ---------------------------------------------------------------------------
// ChatShareThemeBackdrop(s)
// ---------------------------------------------------------------------------

/// A backdrop, which can be used as the background of a theme of the screen for sharing a link
/// to a user or a chat.
///
/// Corresponds to the Bot API
/// [`ChatShareThemeBackdrop`](https://core.telegram.org/bots/api#chatsharethemebackdrop) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatShareThemeBackdrop {
    /// Unique identifier of the backdrop.
    pub id: i64,

    /// Name of the backdrop.
    pub name: String,

    /// Colors of the backdrop.
    pub colors: UniqueGiftBackdropColors,
}

/// Contains a list of backdrops, which can be used as the background of a theme of the screen
/// for sharing a link to a user or a chat.
///
/// Corresponds to the Bot API
/// [`ChatShareThemeBackdrops`](https://core.telegram.org/bots/api#chatsharethemebackdrops) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct ChatShareThemeBackdrops {
    /// The list of backdrops.
    pub backdrops: Vec<ChatShareThemeBackdrop>,
}

impl_new!(ChatShareThemeBackdrops { backdrops: Vec<ChatShareThemeBackdrop> });
