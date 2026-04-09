use serde::{Deserialize, Serialize};

/// A reaction with a normal emoji.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReactionTypeEmojiData {
    /// Reaction emoji.
    pub emoji: String,
}

/// A reaction with a custom emoji.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReactionTypeCustomEmojiData {
    /// Custom emoji identifier.
    pub custom_emoji_id: String,
}

/// A paid reaction (no additional fields beyond the type tag).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReactionTypePaidData {}

/// Polymorphic reaction type, selected by the `"type"` field in the JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ReactionType {
    /// A reaction using a standard emoji.
    Emoji(ReactionTypeEmojiData),

    /// A reaction using a custom emoji.
    CustomEmoji(ReactionTypeCustomEmojiData),

    /// A paid reaction.
    Paid(ReactionTypePaidData),
}

impl ReactionType {
    /// Create a standard emoji reaction.
    pub fn emoji(emoji: impl Into<String>) -> Self {
        Self::Emoji(ReactionTypeEmojiData {
            emoji: emoji.into(),
        })
    }

    /// Create a custom emoji reaction.
    pub fn custom_emoji(custom_emoji_id: impl Into<String>) -> Self {
        Self::CustomEmoji(ReactionTypeCustomEmojiData {
            custom_emoji_id: custom_emoji_id.into(),
        })
    }

    /// Create a paid reaction.
    pub fn paid() -> Self {
        Self::Paid(ReactionTypePaidData {})
    }
}

/// A reaction added to a message along with the number of times it was added.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReactionCount {
    /// Type of the reaction.
    #[serde(rename = "type")]
    pub reaction_type: ReactionType,

    /// Number of times the reaction was added.
    pub total_count: i64,
}
