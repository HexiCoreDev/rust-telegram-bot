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
    ///
    /// Required unless the message is sent as an ephemeral reply to a callback query. If
    /// omitted, then the `chat_id` parameter of the method is ignored. If specified together
    /// with `callback_query_id`, then it must be the identifier of the user who pressed the
    /// button. Must be omitted for messages sent in chats where the bot isn't a member, unless
    /// the message is a reply to an ephemeral message. It is not guaranteed that the user will
    /// receive the message, especially if they are offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_user_id: Option<i64>,

    /// Identifier of the callback query which triggered the message, if any.
    ///
    /// If specified and `receiver_user_id` is omitted, then the message is sent as an ephemeral
    /// reply to the callback query and will be shown only to the user who pressed the button.
    /// Ephemeral replies can't be sent to callback queries from messages in channel direct
    /// messages chats. The bot must still answer the callback query using
    /// `answerCallbackQuery`.
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
            receiver_user_id: Some(receiver_user_id),
            callback_query_id: None,
            replace_callback_query_message: None,
        }
    }

    /// Creates ephemeral message parameters for an ephemeral reply to a callback query.
    ///
    /// The answer is shown only to the user who pressed the button. This is the only documented
    /// way to send an ephemeral message without a `receiver_user_id`.
    pub fn reply_to_callback_query(callback_query_id: impl Into<String>) -> Self {
        Self {
            receiver_user_id: None,
            callback_query_id: Some(callback_query_id.into()),
            replace_callback_query_message: None,
        }
    }

    /// Sets the `receiver_user_id` parameter.
    pub fn receiver_user_id(mut self, val: i64) -> Self {
        self.receiver_user_id = Some(val);
        self
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
