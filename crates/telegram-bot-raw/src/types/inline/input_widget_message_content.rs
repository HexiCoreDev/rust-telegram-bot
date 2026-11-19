use serde::{Deserialize, Serialize};

use crate::types::message_entity::MessageEntity;

/// Represents the content of a message with an [HTML widget](https://core.telegram.org/bots/widgets)
/// to be sent as the result of an inline query.
///
/// Currently, widgets can't be sent in response to inline queries in the production environment.
///
/// Corresponds to the Bot API
/// [`InputWidgetMessageContent`](https://core.telegram.org/bots/api#inputwidgetmessagecontent) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputWidgetMessageContent {
    /// HTML code of the widget, 1-32768 bytes. The code must not exceed 4096 bytes when
    /// compressed by zlib with the maximum compression level.
    pub html: String,

    /// Width of the widget in logical pixels; omit if the widget must occupy the whole width of
    /// the message bubble. The width is clamped by the app to the range from 200 to the maximum
    /// width of the message bubble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// Height of the widget in logical pixels; omit if the height of the widget isn't fixed. The
    /// height is clamped by the app to the range 50-600.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// Public data of the widget, 0-1024 bytes, which will be available wherever the widget is
    /// shown. If the widget is shared, then all messages with it will have the same public data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,

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

    /// Pass `true` to allow the widget to send queries to the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_queries: Option<bool>,

    /// Pass `true` to create a shared widget. Each message sent from the result creates a
    /// separate shared widget, whose identifier is received by the bot only in the field
    /// `widget_id` of queries from the widget. To send an existing shared widget in response to
    /// an inline query, use `InputSharedWidgetMessageContent` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
}

impl_new!(InputWidgetMessageContent { html: String });

impl InputWidgetMessageContent {
    /// Sets the `width` parameter.
    pub fn width(mut self, val: i64) -> Self {
        self.width = Some(val);
        self
    }

    /// Sets the `height` parameter.
    pub fn height(mut self, val: i64) -> Self {
        self.height = Some(val);
        self
    }

    /// Sets the `public_data` parameter.
    pub fn public_data(mut self, val: impl Into<String>) -> Self {
        self.public_data = Some(val.into());
        self
    }

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

    /// Sets the `allow_queries` parameter.
    pub fn allow_queries(mut self, val: bool) -> Self {
        self.allow_queries = Some(val);
        self
    }

    /// Sets the `is_shared` parameter.
    pub fn is_shared(mut self, val: bool) -> Self {
        self.is_shared = Some(val);
        self
    }
}
