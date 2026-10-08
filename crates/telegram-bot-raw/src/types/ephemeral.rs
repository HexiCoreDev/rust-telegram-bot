use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// EphemeralMessageParameters
// ---------------------------------------------------------------------------

/// Parameters of an ephemeral message to be sent.
///
/// Ephemeral messages are visible only to a specific user and the bot, and
/// delivery is not guaranteed. Used by the `send*` methods through the
/// `ephemeral_message_parameters` parameter.
///
/// Corresponds to the Bot API [`EphemeralMessageParameters`](https://core.telegram.org/bots/api#ephemeralmessageparameters) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct EphemeralMessageParameters {
    /// Identifier of the user who will receive the message.
    pub receiver_user_id: i64,

    /// Identifier of the callback query which triggered the message, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query_id: Option<String>,

    /// Pass `true` to show the ephemeral message in place of the original message.
    /// Must be `false` for callback queries from ephemeral messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_callback_query_message: Option<bool>,
}

impl EphemeralMessageParameters {
    /// Creates ephemeral message parameters targeting the given user.
    pub fn new(receiver_user_id: i64) -> Self {
        Self {
            receiver_user_id,
            callback_query_id: None,
            replace_callback_query_message: None,
        }
    }

    /// Sets the `callback_query_id` parameter.
    pub fn callback_query_id(mut self, val: impl Into<String>) -> Self {
        self.callback_query_id = Some(val.into());
        self
    }

    /// Shows the ephemeral message in place of the original message.
    pub fn replace_original(mut self) -> Self {
        self.replace_callback_query_message = Some(true);
        self
    }
}
