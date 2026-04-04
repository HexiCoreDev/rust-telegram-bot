use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::files::input_file::InputFile;

/// Payload for a static profile photo — a single `.JPG` image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputProfilePhotoStaticData {
    /// The static profile photo file.
    pub photo: InputFile,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Payload for an animated profile photo — an MPEG4 video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputProfilePhotoAnimatedData {
    /// The animated profile photo file.
    pub animation: InputFile,

    /// Timestamp (seconds) of the frame used as the static profile photo. Defaults to `0.0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_frame_timestamp: Option<f64>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A profile photo to set — either a static image or an animated video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputProfilePhoto {
    /// A static `.JPG` profile photo.
    Static(InputProfilePhotoStaticData),

    /// An animated MPEG4 profile photo.
    Animated(InputProfilePhotoAnimatedData),
}
