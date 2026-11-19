use serde::{Deserialize, Serialize};

use super::user::User;

// ---------------------------------------------------------------------------
// ChatOwnerChanged
// ---------------------------------------------------------------------------

/// Describes a service message about an ownership change in the chat.
///
/// Corresponds to the Bot API
/// [`ChatOwnerChanged`](https://core.telegram.org/bots/api#chatownerchanged) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatOwnerChanged {
    /// The new owner of the chat.
    pub new_owner: User,
}

impl ChatOwnerChanged {
    /// Creates a new `ChatOwnerChanged` for the given owner.
    pub fn new(new_owner: User) -> Self {
        Self { new_owner }
    }
}

// ---------------------------------------------------------------------------
// ChatOwnerLeft
// ---------------------------------------------------------------------------

/// Describes a service message about the chat owner leaving the chat.
///
/// Corresponds to the Bot API
/// [`ChatOwnerLeft`](https://core.telegram.org/bots/api#chatownerleft) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct ChatOwnerLeft {
    /// The user who will become the new owner of the chat if the previous owner does not return
    /// to the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_owner: Option<User>,
}
