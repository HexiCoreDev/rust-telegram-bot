
use serde::{Deserialize, Serialize};

/// A forum topic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic (message thread ID).
    pub message_thread_id: i64,

    /// Name of the topic.
    pub name: String,

    /// Color of the topic icon in RGB format.
    pub icon_color: i64,

    /// Unique identifier of the custom emoji shown as the topic icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,

    /// `true` if the topic name was not set explicitly and may need updating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_name_implicit: Option<bool>,
}

/// Service message content for a newly created forum topic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicCreated {
    /// Name of the topic.
    pub name: String,

    /// Color of the topic icon in RGB format.
    pub icon_color: i64,

    /// Unique identifier of the custom emoji shown as the topic icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,

    /// `true` if the topic name was not set explicitly and may need updating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_name_implicit: Option<bool>,
}

/// Service message: a forum topic was closed.  Carries no additional data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicClosed {
}

/// Service message: a forum topic was reopened.  Carries no additional data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicReopened {
}

/// Service message content for an edited forum topic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicEdited {
    /// New name of the topic, if it was changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// New custom emoji identifier for the topic icon; empty string means icon was removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// Service message: the General forum topic was hidden.  Carries no additional data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralForumTopicHidden {
}

/// Service message: the General forum topic was unhidden.  Carries no additional data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralForumTopicUnhidden {
}
