use serde::{Deserialize, Serialize};

use crate::types::user::User;

// ---------------------------------------------------------------------------
// BotSubscriptionUpdated
// ---------------------------------------------------------------------------

/// Changes to a user payment subscription toward the current bot.
///
/// Corresponds to the Bot API [`BotSubscriptionUpdated`](https://core.telegram.org/bots/api#botsubscriptionupdated) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BotSubscriptionUpdated {
    /// User who subscribed for payments toward the bot.
    pub user: User,

    /// Bot-specified invoice payload.
    pub invoice_payload: String,

    /// New subscription state: `"canceled"`, `"active"` or `"failed"`.
    pub state: String,
}
