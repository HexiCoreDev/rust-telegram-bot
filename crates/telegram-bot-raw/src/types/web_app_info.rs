use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Information about a Telegram Web App.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebAppInfo {
    /// HTTPS URL of the Web App to be opened.
    pub url: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
