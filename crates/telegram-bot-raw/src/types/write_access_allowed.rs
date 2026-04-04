use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Service message: a user has allowed the bot to write messages.
///
/// This is sent when the user adds the bot to an attachment menu, launches a Web App
/// from a link, or accepts an explicit `requestWriteAccess` call from a Web App.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteAccessAllowed {
    /// Name of the Web App if access was granted by launching it from a link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<String>,

    /// `true` if the user accepted an explicit `requestWriteAccess` Web App call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_request: Option<bool>,

    /// `true` if access was granted when the bot was added to an attachment or side menu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attachment_menu: Option<bool>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
