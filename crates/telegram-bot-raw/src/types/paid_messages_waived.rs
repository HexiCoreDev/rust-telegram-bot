use serde::{Deserialize, Serialize};

/// Describes a service message about the sender of the message using a link of the other user,
/// which allows sending messages to the user without paying Telegram Stars.
///
/// Currently holds no information.
///
/// Corresponds to the Bot API
/// [`PaidMessagesWaived`](https://core.telegram.org/bots/api#paidmessageswaived) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct PaidMessagesWaived {}

impl_new!(PaidMessagesWaived {});
