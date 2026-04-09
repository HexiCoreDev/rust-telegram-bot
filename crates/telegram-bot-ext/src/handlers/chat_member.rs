//! [`ChatMemberHandler`] -- handles chat member updates.
//!
//! Ported from `python-telegram-bot`'s `ChatMemberHandler`. Supports filtering
//! by member update type (`my_chat_member`, `chat_member`, or both) and by
//! chat ID.

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use rust_tg_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Which kind of chat member update to handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChatMemberType {
    /// Only `my_chat_member` (the bot's own status changed).
    MyChatMember,
    /// Only `chat_member` (another user's status changed).
    ChatMember,
    /// Both `my_chat_member` and `chat_member`.
    Any,
}

/// Handler for `Update.my_chat_member` and `Update.chat_member`.
pub struct ChatMemberHandler {
    callback: HandlerCallback,
    member_type: ChatMemberType,
    chat_ids: HashSet<i64>,
    block: bool,
}

impl ChatMemberHandler {
    /// Create a new `ChatMemberHandler`.
    ///
    /// `chat_ids` may be empty; when empty, all chats match.
    pub fn new(
        callback: HandlerCallback,
        member_type: ChatMemberType,
        chat_ids: HashSet<i64>,
        block: bool,
    ) -> Self {
        Self {
            callback,
            member_type,
            chat_ids,
            block,
        }
    }
}

impl Handler for ChatMemberHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let has_my = update.my_chat_member().is_some();
        let has_other = update.chat_member().is_some();

        if !has_my && !has_other {
            return None;
        }

        // Type filter
        match self.member_type {
            ChatMemberType::MyChatMember => {
                if !has_my {
                    return None;
                }
            }
            ChatMemberType::ChatMember => {
                if !has_other {
                    return None;
                }
            }
            ChatMemberType::Any => {}
        }

        // Chat ID filter
        if !self.chat_ids.is_empty() {
            let chat_id = update.effective_chat().map(|c| c.id)?;
            if !self.chat_ids.contains(&chat_id) {
                return None;
            }
        }

        Some(MatchResult::Empty)
    }

    fn handle_update(
        &self,
        update: Arc<Update>,
        match_result: MatchResult,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        (self.callback)(update, match_result)
    }

    fn block(&self) -> bool {
        self.block
    }
}
