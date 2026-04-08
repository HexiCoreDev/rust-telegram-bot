use super::{push_opt_str, Bot, ChatId, Result};
use crate::request::request_parameter::RequestParameter;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Verification
    // ======================================================================

    pub async fn verify_chat(
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

    pub async fn verify_user(
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

    pub async fn remove_chat_verification(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("removeChatVerification", params).await
    }

    pub async fn remove_user_verification(&self, user_id: i64) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        self.do_post("removeUserVerification", params).await
    }
}
