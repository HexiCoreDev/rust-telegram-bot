use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Scope covering the default commands visible to all users.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeDefault {
    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Scope covering all private chats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllPrivateChats {
    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Scope covering all group and supergroup chats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllGroupChats {
    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Scope covering all group and supergroup chat administrators.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {
    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Scope covering a specific chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatData {
    /// Unique identifier for the target chat or username of the target supergroup.
    pub chat_id: ChatId,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Scope covering all administrators of a specific chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatAdministratorsData {
    /// Unique identifier for the target chat or username of the target supergroup.
    pub chat_id: ChatId,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Scope covering a specific member of a group or supergroup chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatMemberData {
    /// Unique identifier for the target chat or username of the target supergroup.
    pub chat_id: ChatId,

    /// Unique identifier of the target user.
    pub user_id: i64,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Chat identifier: either a numeric ID or a `@username` string.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatId {
    /// Numeric chat ID.
    Id(i64),
    /// `@username` string.
    Username(String),
}

/// Polymorphic scope to which bot commands are applied.
///
/// Serialized with a `"type"` tag that selects the variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BotCommandScope {
    /// Default scope — commands visible to all users in all chats.
    Default(BotCommandScopeDefault),

    /// Commands visible to all users in all private chats.
    AllPrivateChats(BotCommandScopeAllPrivateChats),

    /// Commands visible to all users in all group/supergroup chats.
    AllGroupChats(BotCommandScopeAllGroupChats),

    /// Commands visible to all administrators in all group/supergroup chats.
    AllChatAdministrators(BotCommandScopeAllChatAdministrators),

    /// Commands visible to all users in a specific chat.
    Chat(BotCommandScopeChatData),

    /// Commands visible to all administrators of a specific chat.
    ChatAdministrators(BotCommandScopeChatAdministratorsData),

    /// Commands visible to a specific member of a specific chat.
    ChatMember(BotCommandScopeChatMemberData),
}
