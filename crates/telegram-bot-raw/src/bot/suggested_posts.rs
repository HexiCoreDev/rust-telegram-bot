use super::{push_opt, push_opt_str, Bot, Result};
use crate::request::request_parameter::RequestParameter;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Suggested posts
    // ======================================================================

    /// Use this method to approve a suggested post in a channel managed by the bot.
    ///
    /// Calls the Telegram `approveSuggestedPost` API method.
    pub async fn approve_suggested_post_raw(
        &self,
        chat_id: i64,
        message_id: i64,
        send_date: Option<i64>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt(&mut params, "send_date", &send_date)?;
        self.do_post("approveSuggestedPost", params).await
    }

    /// Use this method to decline a suggested post in a channel managed by the bot.
    ///
    /// Calls the Telegram `declineSuggestedPost` API method.
    pub async fn decline_suggested_post_raw(
        &self,
        chat_id: i64,
        message_id: i64,
        comment: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt_str(&mut params, "comment", comment);
        self.do_post("declineSuggestedPost", params).await
    }
}
