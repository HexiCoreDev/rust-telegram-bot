use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::files::photo_size::PhotoSize;

/// A user's profile pictures.
///
/// `photos` is a list of size sets — each inner `Vec` holds the same image in up to 4 sizes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has.
    pub total_count: i64,

    /// Requested profile pictures; each inner list holds the same image in up to 4 sizes.
    pub photos: Vec<Vec<PhotoSize>>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
