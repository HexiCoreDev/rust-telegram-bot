use serde::{Deserialize, Serialize};

use super::message_entity::MessageEntity;

// ---------------------------------------------------------------------------
// InputChecklistTask
// ---------------------------------------------------------------------------

/// A task to add to a checklist.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputChecklistTask {
    /// Unique identifier of the task; must be positive and unique within the checklist.
    pub id: i64,

    /// Text of the task.
    pub text: String,

    /// Parse mode for the task text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// Special entities in the text, used instead of `parse_mode`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_entities: Vec<MessageEntity>,
}

impl InputChecklistTask {
    /// Creates a new `InputChecklistTask`.
    pub fn new(id: i64, text: impl Into<String>) -> Self {
        Self {
            id,
            text: text.into(),
            ..Default::default()
        }
    }
}

// ---------------------------------------------------------------------------
// InputChecklist
// ---------------------------------------------------------------------------

/// A checklist to create.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputChecklist {
    /// Title of the checklist.
    pub title: String,

    /// Tasks in the checklist.
    pub tasks: Vec<InputChecklistTask>,

    /// Parse mode for the checklist title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// Special entities in the title, used instead of `parse_mode`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub title_entities: Vec<MessageEntity>,

    /// True if other users can add tasks to the checklist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_add_tasks: Option<bool>,

    /// True if other users can mark tasks as done or not done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_mark_tasks_as_done: Option<bool>,
}

impl InputChecklist {
    /// Creates a new `InputChecklist` with the given title and tasks.
    pub fn new(title: impl Into<String>, tasks: Vec<InputChecklistTask>) -> Self {
        Self {
            title: title.into(),
            tasks,
            parse_mode: None,
            title_entities: Vec::new(),
            others_can_add_tasks: None,
            others_can_mark_tasks_as_done: None,
        }
    }
}
