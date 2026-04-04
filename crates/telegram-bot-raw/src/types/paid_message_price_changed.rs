use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Service message about a change in the price of paid messages within a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaidMessagePriceChanged {
    /// New number of Telegram Stars that non-administrator users must pay per sent message.
    pub paid_message_star_count: i64,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
