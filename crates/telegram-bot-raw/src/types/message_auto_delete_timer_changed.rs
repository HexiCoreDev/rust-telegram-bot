use serde::{Deserialize, Serialize};

/// Service message about a change in auto-delete timer settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat, in seconds.
    pub message_auto_delete_time: i64,
}
