
use serde::{Deserialize, Serialize};

use super::user::User;

/// Represents an invite link for a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatInviteLink {
    /// The invite link.
    pub invite_link: String,
    /// Creator of the link.
    pub creator: User,
    /// True if users joining the chat via the link need to be approved by chat administrators.
    pub creates_join_request: bool,
    /// True if the link is primary.
    pub is_primary: bool,
    /// True if the link is revoked.
    pub is_revoked: bool,

    /// Unix timestamp when the link will expire or has expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// Maximum number of users that can be members of the chat simultaneously after joining via
    /// this invite link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    /// Invite link name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Number of pending join requests created using this link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<i64>,
    /// Number of seconds the subscription will be active before the next payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,
    /// Amount of Telegram Stars a user must pay to be a member of the chat using the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_price: Option<i64>,
}
