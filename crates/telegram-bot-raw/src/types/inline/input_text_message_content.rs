
use serde::{Deserialize, Serialize};

use crate::types::link_preview_options::LinkPreviewOptions;
use crate::types::message_entity::MessageEntity;

/// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent.
    pub message_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
}
