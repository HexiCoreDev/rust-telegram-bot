use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Service message about a change in the price of direct messages sent to a channel chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectMessagePriceChanged {
    /// True if direct messages are enabled for the channel chat.
    pub are_direct_messages_enabled: bool,

    /// New number of Stars that users must pay per direct message; absent when disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_message_star_count: Option<i64>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
