
use serde::{Deserialize, Serialize};

use crate::types::files::input_file::InputFile;

/// Payload for a story photo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputStoryContentPhotoData {
    /// The photo to post as a story.
    /// Must be 1080×1920 px and no larger than 10 MB.
    pub photo: InputFile,
}

/// Payload for a story video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputStoryContentVideoData {
    /// The video to post as a story.
    /// Must be 720×1280 px, streamable, H.265-encoded MPEG4, and no larger than 30 MB.
    pub video: InputFile,

    /// Precise duration of the video in seconds (0–60).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// Timestamp (seconds) of the frame used as the static story cover. Defaults to `0.0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_frame_timestamp: Option<f64>,

    /// Pass `true` if the video has no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_animation: Option<bool>,
}

/// The content of a story to post — either a photo or a video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputStoryContent {
    /// A photo story.
    Photo(InputStoryContentPhotoData),

    /// A video story.
    Video(InputStoryContentVideoData),
}
