use serde::{Deserialize, Serialize};

use crate::types::files::base_medium::BaseMedium;
use crate::types::files::base_thumbed_medium::BaseThumbedMedium;
use crate::types::files::photo_size::PhotoSize;

/// A general file (as opposed to photos, voice messages and audio files).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    /// Telegram file identifier.
    pub file_id: String,

    /// Stable unique identifier — same across time and different bots.
    pub file_unique_id: String,

    /// Original filename as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    /// MIME type of the file as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,

    /// Document thumbnail as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

impl BaseMedium for Document {
    fn file_id(&self) -> &str {
        &self.file_id
    }

    fn file_unique_id(&self) -> &str {
        &self.file_unique_id
    }
}

impl BaseThumbedMedium for Document {
    fn thumbnail(&self) -> Option<&PhotoSize> {
        self.thumbnail.as_ref()
    }
}
