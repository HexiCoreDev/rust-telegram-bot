
use serde::{Deserialize, Serialize};

use crate::types::chat::Chat;
use crate::types::gifts::Gift;
use crate::types::paid_media::PaidMedia;
use crate::types::unique_gift::UniqueGift;
use crate::types::user::User;

use super::affiliate_info::AffiliateInfo;
use super::revenue_withdrawal_state::RevenueWithdrawalState;

/// Describes the affiliate program that issued the affiliate commission received via this transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerAffiliateProgram {
    /// Information about the bot that sponsored the affiliate program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsor_user: Option<User>,

    /// Telegram Stars received by the bot for each 1000 Stars received by the affiliate program sponsor from referred users.
    pub commission_per_mille: i64,
}

/// Describes a transaction with a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerChat {
    /// Information about the chat.
    pub chat: Chat,

    /// The gift sent to the chat by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<Gift>,
}

/// Describes a withdrawal transaction with Fragment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerFragment {
    /// State of the transaction if the transaction is outgoing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}

/// Describes a transaction with a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerUser {
    /// Type of the transaction (e.g. `invoice_payment`, `paid_media_payment`, `gift_purchase`).
    pub transaction_type: String,

    /// Information about the user.
    pub user: User,

    /// Information about the affiliate that received a commission via this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<AffiliateInfo>,

    /// Bot-specified invoice payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_payload: Option<String>,

    /// Duration of the paid subscription in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,

    /// Information about the paid media bought by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Vec<PaidMedia>>,

    /// Bot-specified paid media payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media_payload: Option<String>,

    /// The gift sent to the user by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<Gift>,

    /// Number of months the gifted Telegram Premium subscription will be active for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_duration: Option<i64>,

    /// The unique gift transferred to the bot by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift: Option<UniqueGift>,
}

/// Describes a transaction with an unknown partner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerOther {
}

/// Describes a withdrawal transaction to the Telegram Ads platform.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerTelegramAds {
}

/// Describes a transaction with payment for paid broadcasting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerTelegramApi {
    /// Number of successful requests that exceeded regular limits and were billed.
    pub request_count: i64,
}

/// Source or recipient of a transaction.
///
/// Discriminated by the `"type"` JSON field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransactionPartner {
    /// Affiliate program that issued the commission.
    AffiliateProgram(TransactionPartnerAffiliateProgram),

    /// Transaction with a chat.
    Chat(Box<TransactionPartnerChat>),

    /// Withdrawal transaction with Fragment.
    Fragment(TransactionPartnerFragment),

    /// Transaction with a user.
    User(Box<TransactionPartnerUser>),

    /// Transaction with an unknown partner.
    Other(TransactionPartnerOther),

    /// Withdrawal to the Telegram Ads platform.
    TelegramAds(TransactionPartnerTelegramAds),

    /// Payment for paid broadcasting.
    TelegramApi(TransactionPartnerTelegramApi),
}
