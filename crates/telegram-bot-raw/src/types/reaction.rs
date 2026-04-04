use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A reaction with a normal emoji.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactionTypeEmojiData {
    /// Reaction emoji.
    pub emoji: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A reaction with a custom emoji.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactionTypeCustomEmojiData {
    /// Custom emoji identifier.
    pub custom_emoji_id: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A paid reaction (no additional fields beyond the type tag).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactionTypePaidData {
    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Polymorphic reaction type, selected by the `"type"` field in the JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReactionType {
    /// A reaction using a standard emoji.
    Emoji(ReactionTypeEmojiData),

    /// A reaction using a custom emoji.
    CustomEmoji(ReactionTypeCustomEmojiData),

    /// A paid reaction.
    Paid(ReactionTypePaidData),
}

/// A reaction added to a message along with the number of times it was added.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactionCount {
    /// Type of the reaction.
    #[serde(rename = "type")]
    pub reaction_type: ReactionType,

    /// Number of times the reaction was added.
    pub total_count: i64,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
