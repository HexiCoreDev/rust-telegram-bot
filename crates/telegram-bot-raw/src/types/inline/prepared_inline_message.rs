use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Describes an inline message to be sent by a user of a Mini App.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PreparedInlineMessage {
    /// Unique identifier of the prepared message.
    pub id: String,

    /// Expiration date of the prepared message as a Unix timestamp.
    /// Expired prepared messages can no longer be used.
    pub expiration_date: i64,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
