
use serde::{Deserialize, Serialize};

/// A chat photo containing references to both the small and big variants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatPhoto {
    /// File identifier of the small (160×160) chat photo.
    /// Valid only for photo download and only as long as the photo is not changed.
    pub small_file_id: String,

    /// Stable unique identifier of the small chat photo — same across time and different bots.
    pub small_file_unique_id: String,

    /// File identifier of the big (640×640) chat photo.
    /// Valid only for photo download and only as long as the photo is not changed.
    pub big_file_id: String,

    /// Stable unique identifier of the big chat photo — same across time and different bots.
    pub big_file_unique_id: String,
}
