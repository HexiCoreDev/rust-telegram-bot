use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::chat::Chat;
use super::message::Message;
use super::message_entity::MessageEntity;
use super::user::User;

// ---------------------------------------------------------------------------
// ChecklistTask
// ---------------------------------------------------------------------------

/// A task in a checklist.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecklistTask {
    /// Unique identifier of the task.
    pub id: i64,

    /// Text of the task.
    pub text: String,

    /// Special entities in the task text.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_entities: Vec<MessageEntity>,

    /// User that completed the task; absent if the task was not completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by_user: Option<User>,

    /// Chat that completed the task; absent if the task was not completed by a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by_chat: Option<Chat>,

    /// Unix timestamp when the task was completed; absent if not completed.
    ///
    /// A value of `0` means the task was not completed (mirrors `ZERO_DATE` in Python).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<i64>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// Checklist
// ---------------------------------------------------------------------------

/// A checklist.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Checklist {
    /// Title of the checklist.
    pub title: String,

    /// Special entities in the checklist title.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub title_entities: Vec<MessageEntity>,

    /// Tasks in the checklist.
    pub tasks: Vec<ChecklistTask>,

    /// True if users other than the creator can add tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_add_tasks: Option<bool>,

    /// True if users other than the creator can mark tasks as done or not done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_mark_tasks_as_done: Option<bool>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// ChecklistTasksDone
// ---------------------------------------------------------------------------

/// Service message about checklist tasks marked as done or not done.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecklistTasksDone {
    /// Message containing the checklist whose tasks were updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_message: Option<Box<Message>>,

    /// Identifiers of the tasks that were marked as done.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub marked_as_done_task_ids: Vec<i64>,

    /// Identifiers of the tasks that were marked as not done.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub marked_as_not_done_task_ids: Vec<i64>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// ChecklistTasksAdded
// ---------------------------------------------------------------------------

/// Service message about tasks added to a checklist.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecklistTasksAdded {
    /// Message containing the checklist to which tasks were added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_message: Option<Box<Message>>,

    /// Tasks added to the checklist.
    pub tasks: Vec<ChecklistTask>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
