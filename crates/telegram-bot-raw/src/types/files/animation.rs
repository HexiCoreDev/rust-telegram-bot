use serde::{Deserialize, Serialize};

use crate::types::files::base_medium::BaseMedium;
use crate::types::files::base_thumbed_medium::BaseThumbedMedium;
use crate::types::files::photo_size::PhotoSize;

/// An animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    /// Telegram file identifier.
    pub file_id: String,

    /// Stable unique identifier — same across time and different bots.
    pub file_unique_id: String,

    /// Video width as defined by the sender.
    pub width: i64,

    /// Video height as defined by the sender.
    pub height: i64,

    /// Duration of the video in seconds.
    pub duration: i64,

    /// Original animation filename as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    /// MIME type of the file as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,

    /// Animation thumbnail as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

impl BaseMedium for Animation {
    fn file_id(&self) -> &str {
        &self.file_id
    }

    fn file_unique_id(&self) -> &str {
        &self.file_unique_id
    }
}

impl BaseThumbedMedium for Animation {
    fn thumbnail(&self) -> Option<&PhotoSize> {
        self.thumbnail.as_ref()
    }
}
