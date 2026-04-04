use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::files::base_medium::BaseMedium;

/// One size of a photo or a file/sticker thumbnail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhotoSize {
    /// Telegram file identifier.
    pub file_id: String,

    /// Stable unique identifier — same across time and different bots.
    pub file_unique_id: String,

    /// Photo width in pixels.
    pub width: i64,

    /// Photo height in pixels.
    pub height: i64,

    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl BaseMedium for PhotoSize {
    fn file_id(&self) -> &str {
        &self.file_id
    }

    fn file_unique_id(&self) -> &str {
        &self.file_unique_id
    }
}
