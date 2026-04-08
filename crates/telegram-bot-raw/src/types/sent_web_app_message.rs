use serde::{Deserialize, Serialize};

/// Information about an inline message sent by a Web App on behalf of a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message.  Present only when an inline keyboard is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
