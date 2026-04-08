use serde::{Deserialize, Serialize};

use super::message::Message;

// ---------------------------------------------------------------------------
// StarAmount — defined here to avoid circular dependency with the payment
// module; mirrors telegram._payment.stars.staramount.StarAmount.
// ---------------------------------------------------------------------------

/// An amount of Telegram Stars.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarAmount {
    /// Integer amount of Telegram Stars, rounded to 0 decimal places.
    pub amount: i64,

    /// The number of 1/1000000000 shares of Telegram Stars by the amount value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,
}

// ---------------------------------------------------------------------------
// SuggestedPostPrice
// ---------------------------------------------------------------------------

/// Price of a suggested post.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostPrice {
    /// Currency: `"XTR"` for Telegram Stars or `"TON"` for toncoins.
    pub currency: String,

    /// Amount in the smallest units of the currency.
    pub amount: i64,
}

// ---------------------------------------------------------------------------
// SuggestedPostParameters
// ---------------------------------------------------------------------------

/// Parameters of a post being suggested by the bot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostParameters {
    /// Proposed price; absent if the post is unpaid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<SuggestedPostPrice>,

    /// Proposed Unix timestamp send date; absent if any time within 30 days is acceptable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}

// ---------------------------------------------------------------------------
// SuggestedPostInfo
// ---------------------------------------------------------------------------

/// Information about a suggested post.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostInfo {
    /// State: `"pending"`, `"approved"`, or `"declined"`.
    pub state: String,

    /// Proposed price; absent if the post is unpaid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<SuggestedPostPrice>,

    /// Proposed Unix timestamp send date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}

// ---------------------------------------------------------------------------
// SuggestedPostDeclined
// ---------------------------------------------------------------------------

/// Service message about the rejection of a suggested post.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostDeclined {
    /// Message containing the suggested post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,

    /// Comment with which the post was declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

// ---------------------------------------------------------------------------
// SuggestedPostPaid
// ---------------------------------------------------------------------------

/// Service message about a successful payment for a suggested post.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostPaid {
    /// Currency: `"XTR"` for Telegram Stars or `"TON"` for toncoins.
    pub currency: String,

    /// Message containing the suggested post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,

    /// Amount received in nanotoncoins; for TON payments only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Amount of Telegram Stars received; for XTR payments only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_amount: Option<StarAmount>,
}

// ---------------------------------------------------------------------------
// SuggestedPostRefunded
// ---------------------------------------------------------------------------

/// Service message about a payment refund for a suggested post.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostRefunded {
    /// Reason for the refund (`"post_deleted"` or `"payment_refunded"`).
    pub reason: String,

    /// Message containing the suggested post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,
}

// ---------------------------------------------------------------------------
// SuggestedPostApproved
// ---------------------------------------------------------------------------

/// Service message about the approval of a suggested post.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostApproved {
    /// Unix timestamp when the post will be published.
    pub send_date: i64,

    /// Message containing the suggested post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,

    /// Amount paid for the post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<SuggestedPostPrice>,
}

// ---------------------------------------------------------------------------
// SuggestedPostApprovalFailed
// ---------------------------------------------------------------------------

/// Service message about a failed approval of a suggested post.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostApprovalFailed {
    /// Expected price of the post.
    pub price: SuggestedPostPrice,

    /// Message containing the suggested post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,
}
