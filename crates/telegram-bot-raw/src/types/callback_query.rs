//! Telegram [`CallbackQuery`] type — ported from
//! `python-telegram-bot/src/telegram/_callbackquery.py`.
//!
//! Only data fields are included. No Bot reference, no API shortcuts.

use serde::{Deserialize, Serialize};

use super::message::MaybeInaccessibleMessage;
use super::user::User;

/// This object represents an incoming callback query from a callback button in an inline keyboard.
///
/// Corresponds to the Bot API
/// [`CallbackQuery`](https://core.telegram.org/bots/api#callbackquery) object.
///
/// If the button that originated the query was attached to a message sent by the bot, the field
/// `message` will be present. If the button was attached to a message sent via the bot (in inline
/// mode), the field `inline_message_id` will be present.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CallbackQuery {
    /// Unique identifier for this query.
    pub id: String,

    /// Sender of the callback query.
    ///
    /// Renamed from Python's `from` (reserved keyword).
    #[serde(rename = "from")]
    pub from_user: User,

    /// Global identifier, uniquely corresponding to the chat to which the message with the
    /// callback button was sent. Useful for high scores in games.
    pub chat_instance: String,

    /// Message sent by the bot with the callback button that originated the query.
    /// Note: message content and message date will not be available if the message is too old.
    ///
    /// Can be a full [`Message`](super::message::Message) or an
    /// [`InaccessibleMessage`](super::message::InaccessibleMessage).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<MaybeInaccessibleMessage>>,

    /// Data associated with the callback button. Be aware that the message originated the query
    /// can contain no callback buttons with this data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Identifier of the message sent via the bot in inline mode, that originated the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    /// Short name of a Game to be returned, serves as the unique identifier for the game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

impl CallbackQuery {
    /// Create a new `CallbackQuery` with the required fields.
    ///
    /// All optional fields default to `None`.
    pub fn new(
        id: impl Into<String>,
        from_user: User,
        chat_instance: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            from_user,
            chat_instance: chat_instance.into(),
            message: None,
            data: None,
            inline_message_id: None,
            game_short_name: None,
        }
    }
}
