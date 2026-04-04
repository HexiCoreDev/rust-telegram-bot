use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A placeholder type for a callback game. Currently holds no information.
///
/// Use BotFather to set up your game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallbackGame {
    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
