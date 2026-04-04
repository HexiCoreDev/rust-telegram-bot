use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::chat::Chat;

/// A Telegram story.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Story {
    /// Chat that posted the story.
    pub chat: Chat,

    /// Unique identifier for the story in the chat.
    pub id: i64,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
