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

/// A live photo to be sent.
///
/// The video must be no longer than 10 seconds and must not exceed 10 MB.
/// Sending live photos by URL is currently unsupported.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputMediaLivePhoto {
    /// Video of the live photo to send.
    pub media: InputFile,

    /// The static photo to send.
    pub photo: InputFile,

    /// Caption (0–1024 characters after entity parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// Special entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Pass `true` to show the caption above the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    /// Pass `true` to cover the live photo with a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

impl InputMediaLivePhoto {
    /// Creates a new `InputMediaLivePhoto` with the given video and static photo.
    pub fn new(media: InputFile, photo: InputFile) -> Self {
        Self {
            media,
            photo,
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

    /// Cover the live photo with a spoiler animation.
    pub fn spoiler(mut self) -> Self {
        self.has_spoiler = Some(true);
        self
    }
}

/// A location to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputMediaLocation {
    /// Latitude of the location.
    pub latitude: f64,

    /// Longitude of the location.
    pub longitude: f64,

    /// Radius of uncertainty for the location, in meters (0–1500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
}

impl InputMediaLocation {
    /// Creates a new `InputMediaLocation` with the given latitude and longitude.
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            ..Default::default()
        }
    }
}

/// A venue to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputMediaVenue {
    /// Latitude of the venue.
    pub latitude: f64,

    /// Longitude of the venue.
    pub longitude: f64,

    /// Name of the venue.
    pub title: String,

    /// Address of the venue.
    pub address: String,

    /// Foursquare identifier of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,

    /// Foursquare type of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,

    /// Google Places identifier of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,

    /// Google Places type of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

impl InputMediaVenue {
    /// Creates a new `InputMediaVenue` with the given coordinates, name and address.
    pub fn new(
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            ..Default::default()
        }
    }
}

/// A sticker file to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputMediaSticker {
    /// File to send: file_id, HTTP URL for .WEBP stickers, or `attach://` upload
    /// of a .WEBP, .TGS or .WEBM sticker.
    pub media: InputFile,

    /// Emoji associated with the sticker; only for just uploaded stickers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
}

impl InputMediaSticker {
    /// Creates a new `InputMediaSticker` with the given media file.
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            ..Default::default()
        }
    }
}

/// An HTTP link to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputMediaLink {
    /// HTTP URL of the link.
    pub url: String,
}

impl InputMediaLink {
    /// Creates a new `InputMediaLink` with the given URL.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

/// A voice message file to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputMediaVoiceNote {
    /// File to send: file_id, HTTP URL, or `attach://` upload.
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

    /// Duration of the voice message in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

impl InputMediaVoiceNote {
    /// Creates a new `InputMediaVoiceNote` with the given media file.
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
    /// A live photo.
    LivePhoto(InputMediaLivePhoto),
    /// A location.
    Location(InputMediaLocation),
    /// A venue.
    Venue(InputMediaVenue),
    /// A sticker.
    Sticker(InputMediaSticker),
}

impl InputMedia {
    /// Attach names and files that must be uploaded as multipart parts.
    ///
    /// Each name matches the `attach://` URI produced by [`InputFile`]'s
    /// `Serialize` impl for the same file, so the JSON payload and the
    /// multipart body agree. [`InputFile::FileId`] and [`InputFile::Url`]
    /// inputs need no upload and are skipped.
    pub fn upload_files(&self) -> Vec<(String, &InputFile)> {
        fn push<'a>(out: &mut Vec<(String, &'a InputFile)>, file: &'a InputFile) {
            match file {
                InputFile::Bytes { filename, .. } => out.push((filename.clone(), file)),
                InputFile::Path(path) => out.push((path.display().to_string(), file)),
                InputFile::FileId(_) | InputFile::Url(_) => {}
            }
        }
        fn push_opt<'a>(out: &mut Vec<(String, &'a InputFile)>, file: &'a Option<InputFile>) {
            if let Some(file) = file {
                push(out, file);
            }
        }
        let mut out = Vec::new();
        match self {
            InputMedia::Photo(m) => push(&mut out, &m.media),
            InputMedia::Video(m) => {
                push(&mut out, &m.media);
                push_opt(&mut out, &m.thumbnail);
                push_opt(&mut out, &m.cover);
            }
            InputMedia::Animation(m) => {
                push(&mut out, &m.media);
                push_opt(&mut out, &m.thumbnail);
            }
            InputMedia::Audio(m) => {
                push(&mut out, &m.media);
                push_opt(&mut out, &m.thumbnail);
            }
            InputMedia::Document(m) => {
                push(&mut out, &m.media);
                push_opt(&mut out, &m.thumbnail);
            }
            InputMedia::LivePhoto(m) => {
                push(&mut out, &m.media);
                push(&mut out, &m.photo);
            }
            InputMedia::Sticker(m) => push(&mut out, &m.media),
            // Locations and venues carry no files.
            InputMedia::Location(_) | InputMedia::Venue(_) => {}
        }
        out
    }
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

/// A paid live photo media item.
///
/// Sending live photos by URL is currently unsupported.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputPaidMediaLivePhoto {
    /// Video of the live photo to send.
    pub media: InputFile,

    /// The static photo to send.
    pub photo: InputFile,
}

impl InputPaidMediaLivePhoto {
    /// Creates a new `InputPaidMediaLivePhoto` with the given video and static photo.
    pub fn new(media: InputFile, photo: InputFile) -> Self {
        Self { media, photo }
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
    /// A paid live photo.
    LivePhoto(InputPaidMediaLivePhoto),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upload_files_skips_file_id_and_url() {
        let photo = InputMedia::Photo(InputMediaPhoto::new(InputFile::file_id("abc")));
        assert!(photo.upload_files().is_empty());
        let photo = InputMedia::Photo(InputMediaPhoto::new(InputFile::url(
            "https://example.com/x.jpg",
        )));
        assert!(photo.upload_files().is_empty());
    }

    #[test]
    fn upload_files_collects_bytes_with_filename_attach_name() {
        let media = InputMedia::Photo(InputMediaPhoto::new(InputFile::bytes(
            "pic.jpg",
            vec![1, 2, 3],
        )));
        let files = media.upload_files();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].0, "pic.jpg");
        // Attach name must match the serialized URI.
        let value = serde_json::to_value(&media).unwrap();
        assert_eq!(value["media"], serde_json::json!("attach://pic.jpg"));
    }

    #[test]
    fn upload_files_collects_thumbnail_and_cover() {
        let mut video = InputMediaVideo::new(InputFile::file_id("vid"));
        video.thumbnail = Some(InputFile::bytes("thumb.jpg", vec![9]));
        video.cover = Some(InputFile::bytes("cover.jpg", vec![8]));
        let media = InputMedia::Video(video);
        let files = media.upload_files();
        let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(names, vec!["thumb.jpg", "cover.jpg"]);
    }

    #[test]
    fn upload_files_collects_live_photo_pair() {
        let live = InputMedia::LivePhoto(InputMediaLivePhoto::new(
            InputFile::bytes("clip.mov", vec![1]),
            InputFile::bytes("still.jpg", vec![2]),
        ));
        let files = live.upload_files();
        let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(names, vec!["clip.mov", "still.jpg"]);
    }

    #[test]
    fn upload_files_empty_for_location_and_venue() {
        assert!(InputMedia::Location(InputMediaLocation {
            latitude: 1.0,
            longitude: 2.0,
            ..Default::default()
        })
        .upload_files()
        .is_empty());
    }
}
