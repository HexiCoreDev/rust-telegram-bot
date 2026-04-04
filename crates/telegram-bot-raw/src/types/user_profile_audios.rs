use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::files::audio::Audio;

/// Audios displayed on a user's profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfileAudios {
    /// Total number of profile audios for the target user.
    pub total_count: i64,

    /// Requested profile audios.
    pub audios: Vec<Audio>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
