use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Options used for link preview generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    /// `true` if the link preview is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,

    /// URL to use for the link preview. If empty, the first URL found in the message text is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// `true` if the media in the link preview is supposed to be shrunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,

    /// `true` if the media in the link preview is supposed to be enlarged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,

    /// `true` if the link preview must be shown above the message text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
