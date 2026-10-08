use serde::{Deserialize, Serialize};

use super::files::photo_size::PhotoSize;

/// A live photo (a photo with a short video).
///
/// Corresponds to the Bot API [`LivePhoto`](https://core.telegram.org/bots/api#livephoto) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct LivePhoto {
    /// Available sizes of the corresponding static photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,

    /// Identifier for the video file, which can be used to download or reuse the file.
    pub file_id: String,

    /// Unique identifier for the video file, supposed to be the same over time and for
    /// different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,

    /// Video width as defined by the sender.
    pub width: i64,

    /// Video height as defined by the sender.
    pub height: i64,

    /// Duration of the video in seconds as defined by the sender.
    pub duration: i64,

    /// MIME type of the file as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

impl_new!(LivePhoto {
    file_id: String,
    file_unique_id: String,
    width: i64,
    height: i64,
    duration: i64
});
