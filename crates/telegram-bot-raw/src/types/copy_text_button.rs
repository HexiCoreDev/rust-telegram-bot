use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Inline keyboard button that copies specified text to the clipboard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyTextButton {
    /// The text to be copied to the clipboard; 1-256 characters.
    pub text: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
