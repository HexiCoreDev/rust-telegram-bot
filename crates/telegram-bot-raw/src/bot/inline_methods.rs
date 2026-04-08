use super::*;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Callback & inline queries
    // ======================================================================

    pub(crate) async fn answer_callback_query_raw(
        &self,
        callback_query_id: &str,
        text: Option<&str>,
        show_alert: Option<bool>,
        url: Option<&str>,
        cache_time: Option<i64>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "callback_query_id",
            serde_json::Value::String(callback_query_id.to_owned()),
        )];
        push_opt_str(&mut params, "text", text);
        push_opt(&mut params, "show_alert", &show_alert)?;
        push_opt_str(&mut params, "url", url);
        push_opt(&mut params, "cache_time", &cache_time)?;
        self.do_post("answerCallbackQuery", params).await
    }

    pub(crate) async fn answer_inline_query_raw(
        &self,
        inline_query_id: &str,
        results: Vec<serde_json::Value>,
        cache_time: Option<i64>,
        is_personal: Option<bool>,
        next_offset: Option<&str>,
        button: Option<serde_json::Value>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "inline_query_id",
                serde_json::Value::String(inline_query_id.to_owned()),
            ),
            RequestParameter::new("results", serde_json::to_value(&results)?),
        ];
        push_opt(&mut params, "cache_time", &cache_time)?;
        push_opt(&mut params, "is_personal", &is_personal)?;
        push_opt_str(&mut params, "next_offset", next_offset);
        push_opt(&mut params, "button", &button)?;
        self.do_post("answerInlineQuery", params).await
    }

    pub async fn save_prepared_inline_message(
        &self,
        user_id: i64,
        result: serde_json::Value,
        allow_user_chats: Option<bool>,
        allow_bot_chats: Option<bool>,
        allow_group_chats: Option<bool>,
        allow_channel_chats: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("result", result),
        ];
        push_opt(&mut params, "allow_user_chats", &allow_user_chats)?;
        push_opt(&mut params, "allow_bot_chats", &allow_bot_chats)?;
        push_opt(&mut params, "allow_group_chats", &allow_group_chats)?;
        push_opt(&mut params, "allow_channel_chats", &allow_channel_chats)?;
        self.do_post("savePreparedInlineMessage", params).await
    }

    pub async fn answer_web_app_query(
        &self,
        web_app_query_id: &str,
        result: serde_json::Value,
    ) -> Result<sent_web_app_message::SentWebAppMessage> {
        let params = vec![
            RequestParameter::new(
                "web_app_query_id",
                serde_json::Value::String(web_app_query_id.to_owned()),
            ),
            RequestParameter::new("result", result),
        ];
        self.do_post("answerWebAppQuery", params).await
    }
}
