use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A unique message identifier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageId {
    /// Unique message identifier. May be `0` when the server schedules the message instead of
    /// sending it immediately.
    pub message_id: i64,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
