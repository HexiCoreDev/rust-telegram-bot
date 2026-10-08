use serde::{Deserialize, Serialize};

use crate::types::input_rich_message::InputRichMessage;

/// Represents the content of a rich message to be sent as the result of an
/// inline query. Only previously uploaded files may be used in the message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichMessageContent {
    /// The message to be sent.
    pub rich_message: InputRichMessage,
}

impl InputRichMessageContent {
    /// Creates rich message content.
    pub fn new(rich_message: InputRichMessage) -> Self {
        Self { rich_message }
    }
}
