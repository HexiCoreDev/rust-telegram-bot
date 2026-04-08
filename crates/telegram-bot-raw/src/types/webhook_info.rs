
use serde::{Deserialize, Serialize};

/// Current status of a webhook.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL; empty string if no webhook is set.
    pub url: String,

    /// `true` if a custom certificate was provided for webhook certificate checks.
    pub has_custom_certificate: bool,

    /// Number of updates awaiting delivery.
    pub pending_update_count: i64,

    /// Currently used webhook IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// Unix timestamp of the most recent delivery error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i64>,

    /// Human-readable error message for the most recent delivery failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,

    /// Maximum simultaneous HTTPS connections allowed for update delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,

    /// Update types the bot is subscribed to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,

    /// Unix timestamp of the most recent datacenter synchronization error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<i64>,
}
