
use serde::{Deserialize, Serialize};

use super::chat::Chat;
use super::message::Message;
use super::user::User;

// ---------------------------------------------------------------------------
// Giveaway
// ---------------------------------------------------------------------------

/// A scheduled giveaway.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Giveaway {
    /// Chats the user must join to participate.
    pub chats: Vec<Chat>,

    /// Unix timestamp when the winner will be selected.
    pub winners_selection_date: i64,

    /// Number of users to be selected as winners.
    pub winner_count: i64,

    /// True if only users who join the chats after the giveaway started are eligible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,

    /// True if the list of winners will be visible to everyone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_public_winners: Option<bool>,

    /// Description of an additional giveaway prize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,

    /// Two-letter ISO 3166-1 alpha-2 country codes for eligible users.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country_codes: Vec<String>,

    /// Stars split between giveaway winners; Star giveaways only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,

    /// Months the Telegram Premium subscription will be active; Premium giveaways only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i64>,
}

// ---------------------------------------------------------------------------
// GiveawayCreated
// ---------------------------------------------------------------------------

/// Service message about the creation of a scheduled giveaway.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GiveawayCreated {
    /// Stars split between giveaway winners; Star giveaways only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
}

// ---------------------------------------------------------------------------
// GiveawayWinners
// ---------------------------------------------------------------------------

/// Message about the completion of a giveaway with public winners.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GiveawayWinners {
    /// Chat that created the giveaway.
    pub chat: Chat,

    /// Identifier of the message with the giveaway.
    pub giveaway_message_id: i64,

    /// Unix timestamp when winners were selected.
    pub winners_selection_date: i64,

    /// Total number of winners in the giveaway.
    pub winner_count: i64,

    /// List of giveaway winners.
    pub winners: Vec<User>,

    /// Number of other chats the user had to join.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<i64>,

    /// Months the Telegram Premium subscription will be active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i64>,

    /// Number of undistributed prizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,

    /// True if only users who joined after the giveaway started were eligible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,

    /// True if the giveaway was cancelled because the payment was refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,

    /// Description of an additional giveaway prize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,

    /// Stars split between giveaway winners; Star giveaways only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
}

// ---------------------------------------------------------------------------
// GiveawayCompleted
// ---------------------------------------------------------------------------

/// Service message about the completion of a giveaway without public winners.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway.
    pub winner_count: i64,

    /// Number of undistributed prizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,

    /// Message with the completed giveaway, if it was not deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_message: Option<Box<Message>>,

    /// True if this is a Telegram Star giveaway rather than a Premium giveaway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_star_giveaway: Option<bool>,
}
