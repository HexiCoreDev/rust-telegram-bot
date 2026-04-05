//! [`ChatBoostHandler`] -- handles chat boost updates.
//!
//! Ported from `python-telegram-bot`'s `ChatBoostHandler`. Supports filtering
//! by boost type (`chat_boost`, `removed_chat_boost`, or both) and by
//! chat ID / username.

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;

use telegram_bot_raw::types::update::Update;

use super::base::{Handler, HandlerCallback, HandlerResult, MatchResult};

/// Which kind of chat boost update to handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatBoostType {
    /// Only `chat_boost`.
    ChatBoost,
    /// Only `removed_chat_boost`.
    RemovedChatBoost,
    /// Both `chat_boost` and `removed_chat_boost`.
    Any,
}

/// Handler for `Update.chat_boost` and `Update.removed_chat_boost`.
pub struct ChatBoostHandler {
    callback: HandlerCallback,
    boost_type: ChatBoostType,
    chat_ids: HashSet<i64>,
    chat_usernames: HashSet<String>,
    block: bool,
}

impl ChatBoostHandler {
    /// Create a new `ChatBoostHandler`.
    pub fn new(
        callback: HandlerCallback,
        boost_type: ChatBoostType,
        chat_ids: HashSet<i64>,
        chat_usernames: HashSet<String>,
        block: bool,
    ) -> Self {
        let chat_usernames = chat_usernames
            .into_iter()
            .map(|u| u.trim_start_matches('@').to_lowercase())
            .collect();
        Self {
            callback,
            boost_type,
            chat_ids,
            chat_usernames,
            block,
        }
    }
}

impl Handler for ChatBoostHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        let has_boost = update.chat_boost.is_some();
        let has_removed = update.removed_chat_boost.is_some();

        if !has_boost && !has_removed {
            return None;
        }

        match self.boost_type {
            ChatBoostType::ChatBoost => {
                if !has_boost {
                    return None;
                }
            }
            ChatBoostType::RemovedChatBoost => {
                if !has_removed {
                    return None;
                }
            }
            ChatBoostType::Any => {}
        }

        // If no ID/username filters, accept.
        if self.chat_ids.is_empty() && self.chat_usernames.is_empty() {
            return Some(MatchResult::Empty);
        }

        // Extract chat info from the effective chat.
        if let Some(chat) = update.effective_chat() {
            if self.chat_ids.contains(&chat.id) {
                return Some(MatchResult::Empty);
            }
            if let Some(ref uname) = chat.username {
                if self.chat_usernames.contains(&uname.to_lowercase()) {
                    return Some(MatchResult::Empty);
                }
            }
        }

        None
    }

    fn handle_update(
        &self,
        update: Update,
        match_result: MatchResult,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        (self.callback)(update, match_result)
    }

    fn block(&self) -> bool {
        self.block
    }
}
