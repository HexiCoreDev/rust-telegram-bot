use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Information about an inline message sent by a Web App on behalf of a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message.  Present only when an inline keyboard is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
