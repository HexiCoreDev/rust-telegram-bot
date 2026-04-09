use serde::{Deserialize, Serialize};

/// An animated emoji with a random value, as returned by the Telegram Bot API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Dice {
    /// Value of the dice.
    pub value: i64,

    /// Emoji on which the dice throw animation is based.
    pub emoji: String,
}
