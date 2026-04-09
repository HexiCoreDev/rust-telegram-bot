use serde::{Deserialize, Serialize};

use super::files::audio::Audio;

/// Audios displayed on a user's profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserProfileAudios {
    /// Total number of profile audios for the target user.
    pub total_count: i64,

    /// Requested profile audios.
    pub audios: Vec<Audio>,
}
