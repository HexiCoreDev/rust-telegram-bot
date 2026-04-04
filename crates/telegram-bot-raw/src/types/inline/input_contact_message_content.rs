use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InputContactMessageContent {
    /// Contact's phone number.
    pub phone_number: String,

    /// Contact's first name.
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
