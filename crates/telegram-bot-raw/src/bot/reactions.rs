use super::{push_opt, Bot, ChatId, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::chat_boost;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Reactions & boosts
    // ======================================================================

    /// Use this method to change the chosen reactions on a message.
    ///
    /// Calls the Telegram `setMessageReaction` API method.
    pub async fn set_message_reaction_raw(
        &self,
        chat_id: ChatId,
        message_id: i64,
        reaction: Option<Vec<serde_json::Value>>,
        is_big: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt(&mut params, "reaction", &reaction)?;
        push_opt(&mut params, "is_big", &is_big)?;
        self.do_post("setMessageReaction", params).await
    }

    /// Use this method to get the list of boosts added to a chat by a user.
    ///
    /// Calls the Telegram `getUserChatBoosts` API method.
    pub async fn get_user_chat_boosts_raw(
        &self,
        chat_id: ChatId,
        user_id: i64,
    ) -> Result<chat_boost::UserChatBoosts> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        self.do_post("getUserChatBoosts", params).await
    }
}
