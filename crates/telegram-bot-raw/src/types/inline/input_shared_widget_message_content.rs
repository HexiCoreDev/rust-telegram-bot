use serde::{Deserialize, Serialize};

use crate::types::message_entity::MessageEntity;

/// Represents the content of a message with a shared widget to be sent as the result of an
/// inline query.
///
/// Currently, widgets can't be sent in response to inline queries in the production environment.
///
/// Corresponds to the Bot API
/// [`InputSharedWidgetMessageContent`](https://core.telegram.org/bots/api#inputsharedwidgetmessagecontent) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputSharedWidgetMessageContent {
    /// Unique identifier of the shared widget created by the bot.
    pub widget_id: String,

    /// Data of the widget, which is specific to the message, 0-1024 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_data: Option<String>,

    /// Widget caption, 0-1024 characters after entities parsing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Mode for parsing entities in the widget caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// List of special entities that appear in the caption, which can be specified instead of
    /// `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Pass `true` if the caption must be shown above the widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}

impl_new!(InputSharedWidgetMessageContent { widget_id: String });

impl InputSharedWidgetMessageContent {
    /// Sets the `chat_data` parameter.
    pub fn chat_data(mut self, val: impl Into<String>) -> Self {
        self.chat_data = Some(val.into());
        self
    }

    /// Sets the `caption` parameter.
    pub fn caption(mut self, val: impl Into<String>) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    /// Sets the `caption_entities` parameter.
    pub fn caption_entities(mut self, val: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }

    /// Sets the `show_caption_above_media` parameter.
    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = Some(val);
        self
    }
}
