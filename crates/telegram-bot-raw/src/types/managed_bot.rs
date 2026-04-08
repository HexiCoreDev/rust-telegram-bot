
use serde::{Deserialize, Serialize};

use super::user::User;

// ---------------------------------------------------------------------------
// ManagedBotCreated
// ---------------------------------------------------------------------------

/// Service message: user created a bot that will be managed by the current bot.
///
/// Corresponds to the Bot API [`ManagedBotCreated`](https://core.telegram.org/bots/api#managedbotcreated) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
pub struct ManagedBotUpdated {
    /// User that created the bot.
    pub user: User,

    /// Information about the bot. Token of the bot can be fetched using `getManagedBotToken`.
    pub bot: User,
}
