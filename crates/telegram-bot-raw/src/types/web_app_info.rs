
use serde::{Deserialize, Serialize};

/// Information about a Telegram Web App.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppInfo {
    /// HTTPS URL of the Web App to be opened.
    pub url: String,
}

impl_new!(WebAppInfo { url: String });
