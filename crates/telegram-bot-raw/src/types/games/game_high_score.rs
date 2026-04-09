use serde::{Deserialize, Serialize};

use crate::types::user::User;

/// Represents one row of the high scores table for a game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GameHighScore {
    /// Position in the high score table for the game.
    pub position: u32,

    /// The user who achieved this score.
    pub user: User,

    /// Score achieved by the user.
    pub score: i64,
}
