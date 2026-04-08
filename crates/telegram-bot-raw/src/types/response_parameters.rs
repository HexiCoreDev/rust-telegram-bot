use serde::{Deserialize, Serialize};

/// Describes why a request was unsuccessful.
///
/// Corresponds to the Bot API
/// [`ResponseParameters`](https://core.telegram.org/bots/api#responseparameters) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    /// In case of exceeding flood control, the number of seconds left to wait
    /// before the request can be repeated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<i64>,
}
