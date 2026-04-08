use super::*;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Reactions & boosts
    // ======================================================================

    pub async fn set_message_reaction(
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

    pub async fn get_user_chat_boosts(
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
