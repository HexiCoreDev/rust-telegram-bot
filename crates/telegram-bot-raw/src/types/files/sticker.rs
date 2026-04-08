use serde::{Deserialize, Serialize};

use crate::types::files::base_medium::BaseMedium;
use crate::types::files::base_thumbed_medium::BaseThumbedMedium;
use crate::types::files::file::File;
use crate::types::files::input_file::InputFile;
use crate::types::files::photo_size::PhotoSize;

/// Position on a face where a mask sticker should be placed by default.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask is placed.
    /// One of `"forehead"`, `"eyes"`, `"mouth"`, or `"chin"`.
    pub point: String,

    /// Shift by X-axis measured in widths of the mask scaled to the face size.
    pub x_shift: f64,

    /// Shift by Y-axis measured in heights of the mask scaled to the face size.
    pub y_shift: f64,

    /// Mask scaling coefficient (e.g. `2.0` means double size).
    pub scale: f64,
}

impl MaskPosition {
    /// Creates a new `MaskPosition`.
    pub fn new(point: impl Into<String>, x_shift: f64, y_shift: f64, scale: f64) -> Self {
        Self {
            point: point.into(),
            x_shift,
            y_shift,
            scale,
        }
    }
}

/// A sticker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sticker {
    /// Telegram file identifier.
    pub file_id: String,

    /// Stable unique identifier — same across time and different bots.
    pub file_unique_id: String,

    /// Sticker width in pixels.
    pub width: i64,

    /// Sticker height in pixels.
    pub height: i64,

    /// `true` if the sticker is animated.
    pub is_animated: bool,

    /// `true` if the sticker is a video sticker.
    pub is_video: bool,

    /// Type of the sticker: `"regular"`, `"mask"`, or `"custom_emoji"`.
    #[serde(rename = "type")]
    pub sticker_type: String,

    /// Emoji associated with the sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,

    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,

    /// Name of the sticker set to which this sticker belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,

    /// Position where the mask should be placed. For mask stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,

    /// Premium animation file for premium regular stickers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<File>,

    /// Unique identifier of the custom emoji. For custom emoji stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,

    /// Sticker thumbnail in `.WEBP` or `.JPG` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,

    /// `true` if the sticker must be repainted to a text color in messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
}

impl BaseMedium for Sticker {
    fn file_id(&self) -> &str {
        &self.file_id
    }

    fn file_unique_id(&self) -> &str {
        &self.file_unique_id
    }
}

impl BaseThumbedMedium for Sticker {
    fn thumbnail(&self) -> Option<&PhotoSize> {
        self.thumbnail.as_ref()
    }
}

/// A sticker set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickerSet {
    /// Sticker set name.
    pub name: String,

    /// Sticker set title.
    pub title: String,

    /// Type of stickers in the set: `"regular"`, `"mask"`, or `"custom_emoji"`.
    pub sticker_type: String,

    /// All stickers that belong to this set.
    pub stickers: Vec<Sticker>,

    /// Set thumbnail in `.WEBP`, `.TGS`, or `.WEBM` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

/// A sticker to be added to a sticker set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputSticker {
    /// The sticker file to add. Animated and video stickers cannot be uploaded via HTTP URL.
    pub sticker: InputFile,

    /// List of 1–20 emoji associated with the sticker.
    pub emoji_list: Vec<String>,

    /// Format of the sticker: `"static"`, `"animated"`, or `"video"`.
    pub format: String,

    /// Where the mask should be placed on faces. For mask stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,

    /// 0–20 search keywords for the sticker (max total length 64 chars).
    /// For regular and custom emoji stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

impl InputSticker {
    /// Creates a new `InputSticker`.
    pub fn new(sticker: InputFile, emoji_list: Vec<String>, format: impl Into<String>) -> Self {
        Self {
            sticker,
            emoji_list,
            format: format.into(),
            mask_position: None,
            keywords: None,
        }
    }
}
