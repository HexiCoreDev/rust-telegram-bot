use serde::{Deserialize, Serialize};

use crate::types::files::base_medium::BaseMedium;
use crate::types::files::base_thumbed_medium::BaseThumbedMedium;
use crate::types::files::photo_size::PhotoSize;

/// A video message (available in Telegram apps as of v4.0).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoNote {
    /// Telegram file identifier.
    pub file_id: String,

    /// Stable unique identifier — same across time and different bots.
    pub file_unique_id: String,

    /// Video width and height (diameter of the video message) as defined by the sender.
    pub length: i64,

    /// Duration of the video in seconds.
    pub duration: i64,

    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,

    /// Video thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

impl BaseMedium for VideoNote {
    fn file_id(&self) -> &str {
        &self.file_id
    }

    fn file_unique_id(&self) -> &str {
        &self.file_unique_id
    }
}

impl BaseThumbedMedium for VideoNote {
    fn thumbnail(&self) -> Option<&PhotoSize> {
        self.thumbnail.as_ref()
    }
}
