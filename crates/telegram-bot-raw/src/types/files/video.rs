
use serde::{Deserialize, Serialize};

use crate::types::files::base_medium::BaseMedium;
use crate::types::files::base_thumbed_medium::BaseThumbedMedium;
use crate::types::files::photo_size::PhotoSize;
use crate::types::files::video_quality::VideoQuality;

/// A video file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
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

    /// Original filename as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    /// MIME type of a file as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,

    /// Video thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,

    /// Available sizes of the cover image for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<Vec<PhotoSize>>,

    /// Timestamp in seconds from which the video will play in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,

    /// List of available quality variants for this video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualities: Option<Vec<VideoQuality>>,
}

impl BaseMedium for Video {
    fn file_id(&self) -> &str {
        &self.file_id
    }

    fn file_unique_id(&self) -> &str {
        &self.file_unique_id
    }
}

impl BaseThumbedMedium for Video {
    fn thumbnail(&self) -> Option<&PhotoSize> {
        self.thumbnail.as_ref()
    }
}
