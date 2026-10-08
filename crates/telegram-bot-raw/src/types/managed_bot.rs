use serde::{Deserialize, Serialize};

use super::user::User;

// ---------------------------------------------------------------------------
// ManagedBotCreated
// ---------------------------------------------------------------------------

/// Service message: user created a bot that will be managed by the current bot.
///
/// Corresponds to the Bot API [`ManagedBotCreated`](https://core.telegram.org/bots/api#managedbotcreated) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ManagedBotCreated {
    /// Information about the bot. The bot's token can be fetched using `getManagedBotToken`.
    pub bot: User,
}

// ---------------------------------------------------------------------------
// ManagedBotUpdated
// ---------------------------------------------------------------------------

/// Information about the creation or token update of a bot managed by the current bot.
///
/// Corresponds to the Bot API [`ManagedBotUpdated`](https://core.telegram.org/bots/api#managedbotupdated) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ManagedBotUpdated {
    /// User that created the bot.
    pub user: User,

    /// Information about the bot. Token of the bot can be fetched using `getManagedBotToken`.
    pub bot: User,
}

// ---------------------------------------------------------------------------
// BotAccessSettings
// ---------------------------------------------------------------------------

/// Access settings of a managed bot.
///
/// Returned by `getManagedBotAccessSettings`. Corresponds to the Bot API
/// [`BotAccessSettings`](https://core.telegram.org/bots/api#botaccesssettings) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BotAccessSettings {
    /// `true` if only selected users can access the bot. The bot's owner can always access it.
    pub is_access_restricted: bool,

    /// Users who have access to the bot when access is restricted, besides its owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_users: Option<Vec<User>>,
}

impl BotAccessSettings {
    /// Creates access settings with the given restriction flag.
    pub fn new(is_access_restricted: bool) -> Self {
        Self {
            is_access_restricted,
            added_users: None,
        }
    }
}
