
use serde::{Deserialize, Serialize};

/// Options used for link preview generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
}

impl_new!(LinkPreviewOptions {});

impl LinkPreviewOptions {
    /// Disable link preview generation entirely.
    pub fn disabled() -> Self {
        Self {
            is_disabled: Some(true),
            ..Default::default()
        }
    }

    /// Set the URL to use for the link preview.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Prefer small media in the link preview.
    pub fn prefer_small_media(mut self) -> Self {
        self.prefer_small_media = Some(true);
        self
    }

    /// Prefer large media in the link preview.
    pub fn prefer_large_media(mut self) -> Self {
        self.prefer_large_media = Some(true);
        self
    }

    /// Show the link preview above the message text.
    pub fn show_above_text(mut self) -> Self {
        self.show_above_text = Some(true);
        self
    }
}
