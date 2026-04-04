use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a file uploaded to Telegram Passport.
///
/// Currently all Telegram Passport files are in JPEG format when decrypted and do not exceed 10 MB.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassportFile {
    /// Identifier for this file, which can be used to download or reuse it.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over time and for
    /// different bots. Cannot be used to download or reuse the file.
    pub file_unique_id: String,

    /// File size in bytes.
    pub file_size: u64,

    /// Unix timestamp when the file was uploaded.
    pub file_date: i64,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
