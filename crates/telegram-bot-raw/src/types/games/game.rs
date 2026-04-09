use serde::{Deserialize, Serialize};

use crate::types::files::animation::Animation;
use crate::types::files::photo_size::PhotoSize;
use crate::types::message_entity::MessageEntity;

/// Represents a Telegram game.
///
/// Use [BotFather](https://t.me/BotFather) to create and edit games; their short names act as
/// unique identifiers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Game {
    /// Title of the game.
    pub title: String,

    /// Description of the game.
    pub description: String,

    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,

    /// Brief description of the game or high scores included in the game message.
    /// Can be automatically edited to include current high scores when the bot calls
    /// `setGameScore`, or manually edited using `editMessageText`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Special entities that appear in `text`, such as usernames, URLs, or bot commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,

    /// Animation that will be displayed in the game message in chats. Upload via BotFather.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}
