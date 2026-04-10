use super::{push_opt_str, Bot, ChatId, Result};
use crate::request::request_parameter::RequestParameter;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Verification
    // ======================================================================

    /// Use this method to verify a chat on behalf of the organization that the bot represents.
    ///
    /// Calls the Telegram `verifyChat` API method.
    pub async fn verify_chat_raw(
        &self,
        chat_id: ChatId,
        custom_description: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt_str(&mut params, "custom_description", custom_description);
        self.do_post("verifyChat", params).await
    }

    /// Use this method to verify a user on behalf of the organization that the bot represents.
    ///
    /// Calls the Telegram `verifyUser` API method.
    pub async fn verify_user_raw(
        &self,
        user_id: i64,
        custom_description: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt_str(&mut params, "custom_description", custom_description);
        self.do_post("verifyUser", params).await
    }

    /// Use this method to remove verification from a chat.
    ///
    /// Calls the Telegram `removeChatVerification` API method.
    pub async fn remove_chat_verification_raw(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("removeChatVerification", params).await
    }

    /// Use this method to remove verification from a user.
    ///
    /// Calls the Telegram `removeUserVerification` API method.
    pub async fn remove_user_verification_raw(&self, user_id: i64) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        self.do_post("removeUserVerification", params).await
    }
}
