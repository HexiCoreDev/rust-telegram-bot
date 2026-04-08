use serde::{Deserialize, Serialize};

/// Scope covering the default commands visible to all users.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeDefault {}

/// Scope covering all private chats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllPrivateChats {}

/// Scope covering all group and supergroup chats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllGroupChats {}

/// Scope covering all group and supergroup chat administrators.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {}

/// Scope covering a specific chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatData {
    /// Unique identifier for the target chat or username of the target supergroup.
    pub chat_id: ChatId,
}

/// Scope covering all administrators of a specific chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatAdministratorsData {
    /// Unique identifier for the target chat or username of the target supergroup.
    pub chat_id: ChatId,
}

/// Scope covering a specific member of a group or supergroup chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatMemberData {
    /// Unique identifier for the target chat or username of the target supergroup.
    pub chat_id: ChatId,

    /// Unique identifier of the target user.
    pub user_id: i64,
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

impl From<i64> for ChatId {
    fn from(id: i64) -> Self {
        Self::Id(id)
    }
}

impl From<&str> for ChatId {
    fn from(username: &str) -> Self {
        Self::Username(username.to_owned())
    }
}

impl From<String> for ChatId {
    fn from(username: String) -> Self {
        Self::Username(username)
    }
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

impl BotCommandScope {
    /// Default scope — commands visible to all users in all chats.
    pub fn default_scope() -> Self {
        Self::Default(BotCommandScopeDefault {})
    }

    /// Commands visible to all users in all private chats.
    pub fn all_private_chats() -> Self {
        Self::AllPrivateChats(BotCommandScopeAllPrivateChats {})
    }

    /// Commands visible to all users in all group/supergroup chats.
    pub fn all_group_chats() -> Self {
        Self::AllGroupChats(BotCommandScopeAllGroupChats {})
    }

    /// Commands visible to all administrators in all group/supergroup chats.
    pub fn all_chat_administrators() -> Self {
        Self::AllChatAdministrators(BotCommandScopeAllChatAdministrators {})
    }

    /// Commands visible to all users in a specific chat.
    pub fn chat(chat_id: impl Into<ChatId>) -> Self {
        Self::Chat(BotCommandScopeChatData {
            chat_id: chat_id.into(),
        })
    }

    /// Commands visible to all administrators of a specific chat.
    pub fn chat_administrators(chat_id: impl Into<ChatId>) -> Self {
        Self::ChatAdministrators(BotCommandScopeChatAdministratorsData {
            chat_id: chat_id.into(),
        })
    }

    /// Commands visible to a specific member of a specific chat.
    pub fn chat_member(chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self::ChatMember(BotCommandScopeChatMemberData {
            chat_id: chat_id.into(),
            user_id,
        })
    }
}
