use serde::{Deserialize, Serialize};

use super::user::User;

/// A topic for direct messages in a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DirectMessagesTopic {
    /// Unique identifier of the topic.
    pub topic_id: i64,

    /// Information about the user that created the topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
