use serde::{Deserialize, Serialize};

/// Service message about a change in the price of paid messages within a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PaidMessagePriceChanged {
    /// New number of Telegram Stars that non-administrator users must pay per sent message.
    pub paid_message_star_count: i64,
}
