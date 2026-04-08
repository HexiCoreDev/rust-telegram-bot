use serde::{Deserialize, Serialize};

use crate::types::files::input_file::InputFile;

/// Payload for a static profile photo -- a single `.JPG` image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputProfilePhotoStaticData {
    /// The static profile photo file.
    pub photo: InputFile,
}

/// Payload for an animated profile photo -- an MPEG4 video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputProfilePhotoAnimatedData {
    /// The animated profile photo file.
    pub animation: InputFile,

    /// Timestamp (seconds) of the frame used as the static profile photo. Defaults to `0.0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_frame_timestamp: Option<f64>,
}

/// A profile photo to set -- either a static image or an animated video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputProfilePhoto {
    /// A static `.JPG` profile photo.
    Static(InputProfilePhotoStaticData),

    /// An animated MPEG4 profile photo.
    Animated(InputProfilePhotoAnimatedData),
}

impl InputProfilePhoto {
    /// Create a static profile photo from a file.
    pub fn static_photo(photo: InputFile) -> Self {
        Self::Static(InputProfilePhotoStaticData { photo })
    }

    /// Create an animated profile photo from a video file.
    pub fn animated(animation: InputFile) -> Self {
        Self::Animated(InputProfilePhotoAnimatedData {
            animation,
            main_frame_timestamp: None,
        })
    }
}
