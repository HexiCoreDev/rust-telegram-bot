use serde::{Deserialize, Serialize};

use crate::types::files::location::Location;
use crate::types::user::User;

/// An incoming inline query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query.
    pub id: String,

    /// Sender.
    #[serde(rename = "from")]
    pub from_user: User,

    /// Text of the query (up to 256 characters).
    pub query: String,

    /// Offset of the results to be returned, can be controlled by the bot.
    pub offset: String,

    /// Type of the chat from which the inline query was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,

    /// Sender location, only for bots that request user location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}
