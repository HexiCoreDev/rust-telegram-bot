use serde::{Deserialize, Serialize};

use super::chat::Chat;
use super::files::sticker::Sticker;
use super::message_entity::MessageEntity;

// ---------------------------------------------------------------------------
// GiftBackground
// ---------------------------------------------------------------------------

/// Background of a gift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GiftBackground {
    /// Center color of the background in RGB format.
    pub center_color: i64,

    /// Edge color of the background in RGB format.
    pub edge_color: i64,

    /// Text color of the background in RGB format.
    pub text_color: i64,
}

// ---------------------------------------------------------------------------
// Gift
// ---------------------------------------------------------------------------

/// A gift that can be sent by the bot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Gift {
    /// Unique identifier of the gift.
    pub id: String,

    /// The sticker that represents the gift.
    pub sticker: Sticker,

    /// Number of Telegram Stars that must be paid to send the sticker.
    pub star_count: i64,

    /// Total number of gifts of this type that can be sent by all users; limited gifts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,

    /// Number of remaining gifts of this type that can be sent; limited gifts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<i64>,

    /// Stars required to upgrade the gift to a unique one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_star_count: Option<i64>,

    /// Chat that published the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Chat>,

    /// Total number of gifts of this type the bot can send; limited gifts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_total_count: Option<i64>,

    /// Remaining gifts of this type the bot can send; limited gifts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_remaining_count: Option<i64>,

    /// Background of the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<GiftBackground>,

    /// True if the gift can only be purchased by Telegram Premium subscribers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,

    /// True if the gift can be used to customize a user's appearance after upgrading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_colors: Option<bool>,

    /// Total number of different unique gifts obtainable by upgrading this gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_variant_count: Option<i64>,
}

// ---------------------------------------------------------------------------
// Gifts
// ---------------------------------------------------------------------------

/// A list of gifts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Gifts {
    /// The sequence of gifts.
    pub gifts: Vec<Gift>,
}

// ---------------------------------------------------------------------------
// GiftInfo
// ---------------------------------------------------------------------------

/// Service message about a regular gift that was sent or received.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GiftInfo {
    /// Information about the gift.
    pub gift: Gift,

    /// Unique identifier of the received gift for the bot; business accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,

    /// Stars claimable by converting the gift; absent if conversion is impossible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_star_count: Option<i64>,

    /// Stars prepaid for the ability to upgrade the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_upgrade_star_count: Option<i64>,

    /// True if the gift can be upgraded to a unique gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_upgraded: Option<bool>,

    /// Text of the message added to the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Special entities in the gift text.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,

    /// True if the sender and gift text are shown only to the gift receiver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,

    /// True if the gift's upgrade was purchased after the gift was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_upgrade_separate: Option<bool>,

    /// Unique number reserved for this gift when upgraded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_number: Option<i64>,
}

// ---------------------------------------------------------------------------
// AcceptedGiftTypes
// ---------------------------------------------------------------------------

/// Types of gifts that can be gifted to a user or a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AcceptedGiftTypes {
    /// True if unlimited regular gifts are accepted.
    pub unlimited_gifts: bool,

    /// True if limited regular gifts are accepted.
    pub limited_gifts: bool,

    /// True if unique gifts or free-upgrade gifts are accepted.
    pub unique_gifts: bool,

    /// True if a Telegram Premium subscription is accepted.
    pub premium_subscription: bool,

    /// True if transfers of unique gifts from channels are accepted.
    pub gifts_from_channels: bool,
}
