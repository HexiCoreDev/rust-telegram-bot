use super::{push_opt, push_opt_str, Bot, MessageOrBool, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{games, message, reply};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Games
    // ======================================================================

    /// Use this method to send a game.
    ///
    /// Calls the Telegram `sendGame` API method.
    pub async fn send_game_raw(
        &self,
        chat_id: i64,
        game_short_name: &str,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new(
                "game_short_name",
                serde_json::Value::String(game_short_name.to_owned()),
            ),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        self.do_post("sendGame", params).await
    }

    /// Use this method to set the score of the specified user in a game message.
    ///
    /// Calls the Telegram `setGameScore` API method.
    pub async fn set_game_score_raw(
        &self,
        user_id: i64,
        score: i64,
        force: Option<bool>,
        disable_edit_message: Option<bool>,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("score", serde_json::to_value(score)?),
        ];
        push_opt(&mut params, "force", &force)?;
        push_opt(&mut params, "disable_edit_message", &disable_edit_message)?;
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        self.do_post("setGameScore", params).await
    }

    /// Use this method to get data for high score tables.
    ///
    /// Calls the Telegram `getGameHighScores` API method.
    pub async fn get_game_high_scores_raw(
        &self,
        user_id: i64,
        chat_id: Option<i64>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
    ) -> Result<Vec<games::game_high_score::GameHighScore>> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        self.do_post("getGameHighScores", params).await
    }
}
