use super::*;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Managed bot methods (Bot API 9.6)
    // ======================================================================

    /// Use this method to get the bot token of a business bot that is managed
    /// by the current bot.
    ///
    /// Requires the *can_manage_bots* business-bot right.
    /// Returns the bot token as a [`String`] on success.
    pub(crate) async fn get_managed_bot_token_raw(&self, bot_user_id: i64) -> Result<String> {
        let params = vec![RequestParameter::new(
            "bot_user_id",
            serde_json::to_value(bot_user_id)?,
        )];
        self.do_post("getManagedBotToken", params).await
    }

    /// Use this method to replace the bot token of a business bot that is
    /// managed by the current bot.  The old token stops working immediately.
    ///
    /// Requires the *can_manage_bots* business-bot right.
    /// Returns the new bot token as a [`String`] on success.
    pub(crate) async fn replace_managed_bot_token_raw(&self, bot_user_id: i64) -> Result<String> {
        let params = vec![RequestParameter::new(
            "bot_user_id",
            serde_json::to_value(bot_user_id)?,
        )];
        self.do_post("replaceManagedBotToken", params).await
    }
}
