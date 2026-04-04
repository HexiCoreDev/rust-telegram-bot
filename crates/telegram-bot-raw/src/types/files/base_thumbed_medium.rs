use crate::types::files::base_medium::BaseMedium;
use crate::types::files::photo_size::PhotoSize;

/// Marker trait for Telegram media objects that may include a thumbnail.
///
/// Extends `BaseMedium` — any type implementing this also exposes
/// `file_id` and `file_unique_id`.
pub trait BaseThumbedMedium: BaseMedium {
    /// Optional thumbnail sent alongside the media.
    fn thumbnail(&self) -> Option<&PhotoSize>;
}
