use serde::{Deserialize, Serialize};

/// A unique message identifier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageId {
    /// Unique message identifier. May be `0` when the server schedules the message instead of
    /// sending it immediately.
    pub message_id: i64,
}
