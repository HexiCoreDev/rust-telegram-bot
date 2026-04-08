
use serde::{Deserialize, Serialize};

/// A placeholder type for a callback game. Currently holds no information.
///
/// Use BotFather to set up your game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallbackGame {
}
