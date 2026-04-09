use serde::{Deserialize, Serialize};

use crate::types::files::input_file::InputFile;
use crate::types::message_entity::MessageEntity;

// ─── InputMedia variants ──────────────────────────────────────────────────────

/// A photo to be sent as part of an album or media group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputMediaPhoto {
    /// The photo file to send.
    pub media: InputFile,

    /// Caption (0–1024 characters after entity parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Parse mode for the caption (`"Markdown"`, `"MarkdownV2"`, or `"HTML"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// Special entities in the caption (bold, italic, links, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Pass `true` to cover the photo with a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,

    /// Pass `true` to show the caption above the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}

impl InputMediaPhoto {
    /// Creates a new `InputMediaPhoto` with the given media file.
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            ..Default::default()
        }
    }

    /// Set the caption.
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set the parse mode.
    pub fn parse_mode(mut self, mode: impl Into<String>) -> Self {
        self.parse_mode = Some(mode.into());
        self
    }

    /// Cover the photo with a spoiler animation.
    pub fn spoiler(mut self) -> Self {
        self.has_spoiler = Some(true);
        self
    }
}

/// A video to be sent as part of an album or media group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputMediaVideo {
    /// The video file to send.
    pub media: InputFile,

    /// Caption (0–1024 characters after entity parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// Special entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Video width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// Video height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// Duration of the video in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    /// Pass `true` if the uploaded video is suitable for streaming.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,

    /// Pass `true` to cover the video with a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,

    /// Thumbnail for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,

    /// Pass `true` to show the caption above the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    /// Cover image for the video in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<InputFile>,

    /// Start timestamp for video playback in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
}

impl InputMediaVideo {
    /// Creates a new `InputMediaVideo` with the given media file.
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            ..Default::default()
        }
    }

    /// Set the caption.
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set the parse mode.
    pub fn parse_mode(mut self, mode: impl Into<String>) -> Self {
        self.parse_mode = Some(mode.into());
        self
    }

    /// Set the video dimensions.
    pub fn dimensions(mut self, width: i64, height: i64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Set the duration in seconds.
    pub fn duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Cover the video with a spoiler animation.
    pub fn spoiler(mut self) -> Self {
        self.has_spoiler = Some(true);
        self
    }
}

/// An animation (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputMediaAnimation {
    /// The animation file to send.
    pub media: InputFile,

    /// Caption (0–1024 characters after entity parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// Special entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Animation width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// Animation height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// Duration of the animation in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    /// Pass `true` to cover the animation with a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,

    /// Thumbnail for the animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,

    /// Pass `true` to show the caption above the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}

impl InputMediaAnimation {
    /// Creates a new `InputMediaAnimation` with the given media file.
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            ..Default::default()
        }
    }

    /// Set the caption.
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set the parse mode.
    pub fn parse_mode(mut self, mode: impl Into<String>) -> Self {
        self.parse_mode = Some(mode.into());
        self
    }

    /// Cover the animation with a spoiler animation.
    pub fn spoiler(mut self) -> Self {
        self.has_spoiler = Some(true);
        self
    }
}

/// An audio file to be treated as music to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputMediaAudio {
    /// The audio file to send.
    pub media: InputFile,

    /// Caption (0–1024 characters after entity parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// Special entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Duration of the audio in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    /// Performer of the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    /// Title of the audio track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Thumbnail for the audio album cover.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
}

impl InputMediaAudio {
    /// Creates a new `InputMediaAudio` with the given media file.
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            ..Default::default()
        }
    }

    /// Set the caption.
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set the parse mode.
    pub fn parse_mode(mut self, mode: impl Into<String>) -> Self {
        self.parse_mode = Some(mode.into());
        self
    }

    /// Set the audio title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the performer.
    pub fn performer(mut self, performer: impl Into<String>) -> Self {
        self.performer = Some(performer.into());
        self
    }
}

/// A general file (document) to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputMediaDocument {
    /// The document file to send.
    pub media: InputFile,

    /// Caption (0–1024 characters after entity parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// Special entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Thumbnail for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,

    /// Pass `true` to disable automatic server-side content type detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}

impl InputMediaDocument {
    /// Creates a new `InputMediaDocument` with the given media file.
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            ..Default::default()
        }
    }

    /// Set the caption.
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set the parse mode.
    pub fn parse_mode(mut self, mode: impl Into<String>) -> Self {
        self.parse_mode = Some(mode.into());
        self
    }
}

/// A tagged union of all `InputMedia*` variants, serialized with a `"type"` discriminant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputMedia {
    /// A photo.
    Photo(InputMediaPhoto),
    /// A video.
    Video(InputMediaVideo),
    /// An animation (GIF / silent video).
    Animation(InputMediaAnimation),
    /// An audio file.
    Audio(InputMediaAudio),
    /// A general document file.
    Document(InputMediaDocument),
}

// ─── InputPaidMedia variants ──────────────────────────────────────────────────

/// A paid photo media item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputPaidMediaPhoto {
    /// The photo file to send.
    pub media: InputFile,
}

impl InputPaidMediaPhoto {
    /// Creates a new `InputPaidMediaPhoto` with the given media file.
    pub fn new(media: InputFile) -> Self {
        Self { media }
    }
}

/// A paid video media item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputPaidMediaVideo {
    /// The video file to send.
    pub media: InputFile,

    /// Thumbnail for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,

    /// Cover image for the video in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<InputFile>,

    /// Start timestamp for video playback in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,

    /// Video width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// Video height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// Duration of the video in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    /// Pass `true` if the uploaded video is suitable for streaming.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}

impl InputPaidMediaVideo {
    /// Creates a new `InputPaidMediaVideo` with the given media file.
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            ..Default::default()
        }
    }
}

/// A tagged union of paid media variants, serialized with a `"type"` discriminant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputPaidMedia {
    /// A paid photo.
    Photo(InputPaidMediaPhoto),
    /// A paid video.
    Video(InputPaidMediaVideo),
}
