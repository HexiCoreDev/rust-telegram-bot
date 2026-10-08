use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// SentGuestMessage
// ---------------------------------------------------------------------------

/// An inline message sent by a guest bot.
///
/// Returned by [`answerGuestQuery`](https://core.telegram.org/bots/api#answerguestquery).
/// Corresponds to the Bot API [`SentGuestMessage`](https://core.telegram.org/bots/api#sentguestmessage) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SentGuestMessage {
    /// Identifier of the sent inline message.
    pub inline_message_id: String,
}

impl SentGuestMessage {
    /// Creates a new `SentGuestMessage` with the given inline message identifier.
    pub fn new(inline_message_id: impl Into<String>) -> Self {
        Self {
            inline_message_id: inline_message_id.into(),
        }
    }
}
