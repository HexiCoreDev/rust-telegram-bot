use super::*;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Suggested posts
    // ======================================================================

    pub async fn approve_suggested_post(
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

    pub async fn decline_suggested_post(
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
