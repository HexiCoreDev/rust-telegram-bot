use serde::{Deserialize, Serialize};

/// Describes an [HTML widget](https://core.telegram.org/bots/widgets) shown in a message.
///
/// Corresponds to the Bot API [`Widget`](https://core.telegram.org/bots/api#widget) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct Widget {
    /// Unique identifier of the shared widget, which can be used to send the widget again
    /// with `sendSharedWidget`; omitted if the widget isn't shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Width of the widget in logical pixels; omitted if the widget must occupy the whole width of the message bubble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// Height of the widget in logical pixels; omitted if the height of the widget isn't fixed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// HTML code of the widget.
    pub html: String,

    /// Public data of the widget, available wherever the widget is shown. All messages with a shared widget have the same public data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,

    /// Data of the widget, which is specific to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_data: Option<String>,
}

impl_new!(Widget { html: String });
