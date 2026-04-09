use serde::{Deserialize, Serialize};

use super::gifts::Gift;
use super::message_entity::MessageEntity;
use super::unique_gift::UniqueGift;
use super::user::User;

// ---------------------------------------------------------------------------
// OwnedGiftRegular
// ---------------------------------------------------------------------------

/// A regular gift owned by a user or a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OwnedGiftRegular {
    /// Information about the regular gift.
    pub gift: Gift,

    /// Unix timestamp when the gift was sent.
    pub send_date: i64,

    /// Unique identifier of the gift for the bot; business accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,

    /// Sender of the gift if they are a known user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_user: Option<User>,

    /// Text of the message added to the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Special entities in the gift text.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,

    /// True if the sender and gift text are shown only to the receiver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,

    /// True if the gift is displayed on the account's profile page; business accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,

    /// True if the gift can be upgraded to a unique gift; business accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_upgraded: Option<bool>,

    /// True if the gift was refunded and is no longer available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,

    /// Stars the receiver can claim by converting the gift; absent if not convertible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_star_count: Option<i64>,

    /// Stars prepaid for the ability to upgrade the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_upgrade_star_count: Option<i64>,

    /// True if the gift's upgrade was purchased after the gift was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_upgrade_separate: Option<bool>,

    /// Unique number reserved for this gift when upgraded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_number: Option<i64>,
}

// ---------------------------------------------------------------------------
// OwnedGiftUnique
// ---------------------------------------------------------------------------

/// A unique gift owned by a user or a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OwnedGiftUnique {
    /// Information about the unique gift.
    pub gift: UniqueGift,

    /// Unix timestamp when the gift was sent.
    pub send_date: i64,

    /// Unique identifier of the gift for the bot; business accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,

    /// Sender of the gift if they are a known user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_user: Option<User>,

    /// True if the gift is displayed on the account's profile page; business accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,

    /// True if the gift can be transferred to another owner; business accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_transferred: Option<bool>,

    /// Stars required to transfer the gift; absent if the bot cannot transfer it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_star_count: Option<i64>,

    /// Unix timestamp when the gift can be transferred next.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_transfer_date: Option<i64>,
}

// ---------------------------------------------------------------------------
// OwnedGift — polymorphic, tagged on "type"
// ---------------------------------------------------------------------------

/// A gift received and owned by a user or a chat.
///
/// Discriminated by the `"type"` JSON field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum OwnedGift {
    /// A regular gift.
    Regular(Box<OwnedGiftRegular>),

    /// A unique gift.
    Unique(Box<OwnedGiftUnique>),
}

// ---------------------------------------------------------------------------
// OwnedGifts
// ---------------------------------------------------------------------------

/// List of gifts received and owned by a user or a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OwnedGifts {
    /// Total number of gifts owned.
    pub total_count: i64,

    /// The list of gifts.
    pub gifts: Vec<OwnedGift>,

    /// Offset for the next request; absent when there are no more results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
}
