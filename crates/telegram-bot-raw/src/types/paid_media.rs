use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::files::photo_size::PhotoSize;
use super::files::video::Video;
use super::user::User;

// ---------------------------------------------------------------------------
// PaidMediaPreview
// ---------------------------------------------------------------------------

/// Paid media that is not yet available before payment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaidMediaPreview {
    /// Media width as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// Media height as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// Duration of the media in seconds as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// PaidMediaPhoto
// ---------------------------------------------------------------------------

/// The paid media is a photo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaidMediaPhoto {
    /// The photo.
    pub photo: Vec<PhotoSize>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// PaidMediaVideo
// ---------------------------------------------------------------------------

/// The paid media is a video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaidMediaVideo {
    /// The video.
    pub video: Video,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// PaidMedia — polymorphic, tagged on "type"
// ---------------------------------------------------------------------------

/// Paid media added to a message.
///
/// Discriminated by the `"type"` JSON field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaidMedia {
    /// Media not yet available before payment.
    Preview(PaidMediaPreview),

    /// A photo.
    Photo(PaidMediaPhoto),

    /// A video.
    Video(PaidMediaVideo),
}

// ---------------------------------------------------------------------------
// PaidMediaInfo
// ---------------------------------------------------------------------------

/// Paid media added to a message, together with the required Star count.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaidMediaInfo {
    /// Number of Telegram Stars that must be paid to buy access to the media.
    pub star_count: i64,

    /// Information about the paid media.
    pub paid_media: Vec<PaidMedia>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// PaidMediaPurchased
// ---------------------------------------------------------------------------

/// Information about a paid media purchase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaidMediaPurchased {
    /// User who purchased the media.
    ///
    /// JSON field name is `"from"` (reserved word in Rust).
    #[serde(rename = "from")]
    pub from_user: User,

    /// Bot-specified paid media payload.
    pub paid_media_payload: String,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
