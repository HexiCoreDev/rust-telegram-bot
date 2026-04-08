
use serde::{Deserialize, Serialize};

use super::chat::Chat;
use super::reaction::{ReactionCount, ReactionType};
use super::user::User;

/// Reaction changes on a message with anonymous reactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageReactionCountUpdated {
    /// The chat containing the message.
    pub chat: Chat,

    /// Unique message identifier inside the chat.
    pub message_id: i64,

    /// Date of the change as a Unix timestamp.
    pub date: i64,

    /// List of reactions that are present on the message.
    pub reactions: Vec<ReactionCount>,
}

/// A change of a reaction on a message performed by a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageReactionUpdated {
    /// The chat containing the message.
    pub chat: Chat,

    /// Unique message identifier inside the chat.
    pub message_id: i64,

    /// Date of the change as a Unix timestamp.
    pub date: i64,

    /// Previous list of reaction types set by the user.
    pub old_reaction: Vec<ReactionType>,

    /// New list of reaction types set by the user.
    pub new_reaction: Vec<ReactionType>,

    /// The user that changed the reaction, if the user is not anonymous.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// The chat on behalf of which the reaction was changed, if the user is anonymous.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Chat>,
}
