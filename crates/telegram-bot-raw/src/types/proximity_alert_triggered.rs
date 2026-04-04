use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::user::User;

/// Service message sent when a user triggers a proximity alert set by another user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert.
    pub traveler: User,

    /// User that set the alert.
    pub watcher: User,

    /// Distance between the users in metres.
    pub distance: i64,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
