use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// ChatOwnerChanged
// ---------------------------------------------------------------------------

/// Service message: the chat owner was changed.
///
/// This is a marker type — Telegram does not expose additional fields beyond
/// the presence of the message itself. Extra unknown fields are captured in
/// `extra` for forward-compatibility.
///
/// Corresponds to the Bot API `chat_owner_changed` service message field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatOwnerChanged {}

// ---------------------------------------------------------------------------
// ChatOwnerLeft
// ---------------------------------------------------------------------------

/// Service message: the chat owner has left the chat.
///
/// This is a marker type — Telegram does not expose additional fields beyond
/// the presence of the message itself. Extra unknown fields are captured in
/// `extra` for forward-compatibility.
///
/// Corresponds to the Bot API `chat_owner_left` service message field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatOwnerLeft {}
