use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Community
// ---------------------------------------------------------------------------

/// A community: several supergroups, channels and bots linked around a shared
/// topic or audience.
///
/// Corresponds to the Bot API [`Community`](https://core.telegram.org/bots/api#community) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Community {
    /// Unique identifier for this community.
    pub id: i64,

    /// Name of the community.
    pub name: String,
}

impl Community {
    /// Creates a community with the given id and name.
    pub fn new(id: i64, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// CommunityChatAdded
// ---------------------------------------------------------------------------

/// Service message: a chat or bot was added to a community.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommunityChatAdded {
    /// The new community to which the chat or bot belongs.
    pub community: Community,
}

// ---------------------------------------------------------------------------
// CommunityChatRemoved
// ---------------------------------------------------------------------------

/// Service message: a chat or bot was removed from a community.
/// Carries no additional data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommunityChatRemoved {}

// ---------------------------------------------------------------------------
// CommunityChatJoined
// ---------------------------------------------------------------------------

/// Service message: a chat was joined by a user from a community.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommunityChatJoined {
    /// The community from which the chat was joined.
    pub community: Community,
}
