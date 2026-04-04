use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::files::base_medium::BaseMedium;

/// A video file of a specific quality, associated with a `Video` message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoQuality {
    /// Telegram file identifier.
    pub file_id: String,

    /// Stable unique identifier — same across time and different bots.
    pub file_unique_id: String,

    /// Video width in pixels.
    pub width: i64,

    /// Video height in pixels.
    pub height: i64,

    /// Codec used to encode the video (e.g. `"h264"`, `"h265"`, `"av01"`).
    pub codec: String,

    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl BaseMedium for VideoQuality {
    fn file_id(&self) -> &str {
        &self.file_id
    }

    fn file_unique_id(&self) -> &str {
        &self.file_unique_id
    }
}
