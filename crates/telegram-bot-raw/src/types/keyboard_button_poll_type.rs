use serde::{Deserialize, Serialize};

/// The type of poll a `KeyboardButton` may request the user to create.
///
/// When `poll_type` is `Some("quiz")` only quiz polls are allowed; `Some("regular")` allows
/// only regular polls; `None` allows any poll type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct KeyboardButtonPollType {
    /// `"quiz"`, `"regular"`, or absent to allow any type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub poll_type: Option<String>,
}

impl_new!(KeyboardButtonPollType {});
