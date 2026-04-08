
use serde::{Deserialize, Serialize};

use crate::types::files::base_medium::BaseMedium;
use crate::types::files::base_thumbed_medium::BaseThumbedMedium;
use crate::types::files::photo_size::PhotoSize;

/// An audio file to be treated as music by Telegram clients.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Audio {
    /// Telegram file identifier.
    pub file_id: String,

    /// Stable unique identifier — same across time and different bots.
    pub file_unique_id: String,

    /// Duration of the audio in seconds.
    pub duration: i64,

    /// Performer of the audio as defined by the sender or by audio tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    /// Title of the audio as defined by the sender or by audio tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Original filename as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    /// MIME type of the file as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,

    /// Thumbnail of the album cover to which the music file belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

impl BaseMedium for Audio {
    fn file_id(&self) -> &str {
        &self.file_id
    }

    fn file_unique_id(&self) -> &str {
        &self.file_unique_id
    }
}

impl BaseThumbedMedium for Audio {
    fn thumbnail(&self) -> Option<&PhotoSize> {
        self.thumbnail.as_ref()
    }
}
