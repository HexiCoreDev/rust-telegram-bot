use serde::{Deserialize, Serialize};

use crate::types::chat::Chat;
use crate::types::user::User;

/// Information about the affiliate that received a commission via a transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffiliateInfo {
    /// The bot or user that received an affiliate commission, if received by a bot or user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_user: Option<User>,

    /// The chat that received an affiliate commission, if received by a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_chat: Option<Chat>,

    /// Telegram Stars received by the affiliate for each 1000 Stars received by the bot from referred users.
    pub commission_per_mille: i64,

    /// Integer amount of Telegram Stars received by the affiliate from the transaction; can be negative for refunds.
    pub amount: i64,

    /// Fractional nanostar shares of Telegram Stars received by the affiliate; can be negative for refunds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,
}
