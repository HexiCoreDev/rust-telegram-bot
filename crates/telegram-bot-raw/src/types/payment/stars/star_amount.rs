use serde::{Deserialize, Serialize};

/// An amount of Telegram Stars.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StarAmount {
    /// Integer amount of Telegram Stars, rounded to 0; can be negative.
    pub amount: i64,

    /// Fractional nanostar shares; from 0 to 999999999. Can be negative if `amount` is non-positive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,
}
