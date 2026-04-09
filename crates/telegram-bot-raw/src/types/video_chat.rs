use serde::{Deserialize, Serialize};

use super::user::User;

/// Service message about a video chat started in the chat.
///
/// Currently holds no information beyond unknown extra fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VideoChatStarted {}

/// Service message about a video chat ended in the chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VideoChatEnded {
    /// Duration of the video chat in seconds.
    pub duration: i64,
}

/// Service message about a video chat scheduled in the chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VideoChatScheduled {
    /// Unix timestamp when the video chat is supposed to be started by a chat administrator.
    pub start_date: i64,
}

/// Service message about new members invited to a video chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat.
    pub users: Vec<User>,
}
