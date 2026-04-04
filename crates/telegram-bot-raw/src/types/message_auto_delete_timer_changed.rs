use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Service message about a change in auto-delete timer settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat, in seconds.
    pub message_auto_delete_time: i64,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
