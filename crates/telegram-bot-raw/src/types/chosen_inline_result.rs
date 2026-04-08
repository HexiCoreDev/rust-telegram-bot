//! Telegram [`ChosenInlineResult`] type — ported from
//! `python-telegram-bot/src/telegram/_choseninlineresult.py`.
//!
//! Only data fields are included. No Bot reference, no API shortcuts.

use serde::{Deserialize, Serialize};

use super::files::location::Location;
use super::user::User;

/// Represents a result of an inline query that was chosen by the user and sent to their chat
/// partner.
///
/// Corresponds to the Bot API
/// [`ChosenInlineResult`](https://core.telegram.org/bots/api#choseninlineresult) object.
///
/// Note: It is necessary to enable inline feedback via `@Botfather` in order to receive these
/// objects in updates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen.
    pub result_id: String,

    /// The user that chose the result.
    ///
    /// Renamed from Python's `from` (reserved keyword).
    #[serde(rename = "from")]
    pub from_user: User,

    /// The query that was used to obtain the result.
    pub query: String,

    /// Sender location, only for bots that require user location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// Identifier of the sent inline message. Available only if there is an inline keyboard
    /// attached to the message. Will be also received in callback queries and can be used to
    /// edit the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
