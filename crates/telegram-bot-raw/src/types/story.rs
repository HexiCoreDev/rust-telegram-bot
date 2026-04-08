use serde::{Deserialize, Serialize};

use super::chat::Chat;

/// A Telegram story.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Story {
    /// Chat that posted the story.
    pub chat: Chat,

    /// Unique identifier for the story in the chat.
    pub id: i64,
}
