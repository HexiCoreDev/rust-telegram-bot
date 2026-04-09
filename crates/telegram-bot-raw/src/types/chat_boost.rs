use serde::{Deserialize, Serialize};

use super::chat::Chat;
use super::user::User;

// ---------------------------------------------------------------------------
// ChatBoostAdded
// ---------------------------------------------------------------------------

/// Service message about a user boosting a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatBoostAdded {
    /// Number of boosts added by the user.
    pub boost_count: i64,
}

// ---------------------------------------------------------------------------
// ChatBoostSource -- tagged union on the "source" field
// ---------------------------------------------------------------------------

/// The boost was obtained by subscribing to Telegram Premium or gifting a subscription.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatBoostSourcePremium {
    /// User that boosted the chat.
    pub user: User,
}

/// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatBoostSourceGiftCode {
    /// User for which the gift code was created.
    pub user: User,
}

/// The boost was obtained by the creation of a Telegram Premium giveaway or a Telegram Star.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatBoostSourceGiveaway {
    /// Identifier of a message in the chat with the giveaway.
    pub giveaway_message_id: i64,

    /// User that won the prize in the giveaway; for Telegram Premium giveaways only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Number of Telegram Stars split between giveaway winners; for Telegram Star giveaways only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
    /// True if the giveaway was completed but there was no user to win the prize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unclaimed: Option<bool>,
}

/// Source of a chat boost. Discriminated by the `"source"` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChatBoostSource {
    /// The boost was obtained by subscribing to Telegram Premium.
    Premium(ChatBoostSourcePremium),
    /// The boost was obtained by creating Telegram Premium gift codes.
    GiftCode(ChatBoostSourceGiftCode),
    /// The boost was obtained through a giveaway.
    Giveaway(ChatBoostSourceGiveaway),
}

// ---------------------------------------------------------------------------
// ChatBoost
// ---------------------------------------------------------------------------

/// Contains information about a chat boost.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatBoost {
    /// Unique identifier of the boost.
    pub boost_id: String,
    /// Unix timestamp when the chat was boosted.
    pub add_date: i64,
    /// Unix timestamp when the boost will automatically expire.
    pub expiration_date: i64,
    /// Source of the added boost.
    pub source: ChatBoostSource,
}

// ---------------------------------------------------------------------------
// ChatBoostUpdated
// ---------------------------------------------------------------------------

/// A boost added to a chat or changed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatBoostUpdated {
    /// Chat which was boosted.
    pub chat: Chat,
    /// Information about the chat boost.
    pub boost: ChatBoost,
}

// ---------------------------------------------------------------------------
// ChatBoostRemoved
// ---------------------------------------------------------------------------

/// A boost removed from a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatBoostRemoved {
    /// Chat which was boosted.
    pub chat: Chat,
    /// Unique identifier of the boost.
    pub boost_id: String,
    /// Unix timestamp when the boost was removed.
    pub remove_date: i64,
    /// Source of the removed boost.
    pub source: ChatBoostSource,
}

// ---------------------------------------------------------------------------
// UserChatBoosts
// ---------------------------------------------------------------------------

/// A list of boosts added to a chat by a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserChatBoosts {
    /// List of boosts added to the chat by the user.
    pub boosts: Vec<ChatBoost>,
}
