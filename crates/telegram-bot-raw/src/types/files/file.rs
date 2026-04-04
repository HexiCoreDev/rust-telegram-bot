use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::files::base_medium::BaseMedium;

/// A file ready to be downloaded from the Telegram servers.
///
/// The download link is guaranteed valid for at least one hour.
/// When it expires, call `getFile` to obtain a fresh one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    /// Telegram file identifier that can be used to download or reuse the file.
    pub file_id: String,

    /// Stable unique identifier — same across time and different bots.
    pub file_unique_id: String,

    /// File size in bytes, if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,

    /// File path used to construct the download URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl BaseMedium for File {
    fn file_id(&self) -> &str {
        &self.file_id
    }

    fn file_unique_id(&self) -> &str {
        &self.file_unique_id
    }
}
