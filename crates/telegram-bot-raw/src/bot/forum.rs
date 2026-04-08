use super::{push_opt, push_opt_str, Bot, ChatId, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::forum_topic;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Forum topics
    // ======================================================================

    pub async fn create_forum_topic(
        &self,
        chat_id: ChatId,
        name: &str,
        icon_color: Option<i64>,
        icon_custom_emoji_id: Option<&str>,
    ) -> Result<forum_topic::ForumTopic> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
        ];
        push_opt(&mut params, "icon_color", &icon_color)?;
        push_opt_str(&mut params, "icon_custom_emoji_id", icon_custom_emoji_id);
        self.do_post("createForumTopic", params).await
    }

    pub async fn edit_forum_topic(
        &self,
        chat_id: ChatId,
        message_thread_id: i64,
        name: Option<&str>,
        icon_custom_emoji_id: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "message_thread_id",
                serde_json::to_value(message_thread_id)?,
            ),
        ];
        push_opt_str(&mut params, "name", name);
        push_opt_str(&mut params, "icon_custom_emoji_id", icon_custom_emoji_id);
        self.do_post("editForumTopic", params).await
    }

    pub async fn close_forum_topic(&self, chat_id: ChatId, message_thread_id: i64) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "message_thread_id",
                serde_json::to_value(message_thread_id)?,
            ),
        ];
        self.do_post("closeForumTopic", params).await
    }

    pub async fn reopen_forum_topic(
        &self,
        chat_id: ChatId,
        message_thread_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "message_thread_id",
                serde_json::to_value(message_thread_id)?,
            ),
        ];
        self.do_post("reopenForumTopic", params).await
    }

    pub async fn delete_forum_topic(
        &self,
        chat_id: ChatId,
        message_thread_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "message_thread_id",
                serde_json::to_value(message_thread_id)?,
            ),
        ];
        self.do_post("deleteForumTopic", params).await
    }

    pub async fn unpin_all_forum_topic_messages(
        &self,
        chat_id: ChatId,
        message_thread_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "message_thread_id",
                serde_json::to_value(message_thread_id)?,
            ),
        ];
        self.do_post("unpinAllForumTopicMessages", params).await
    }

    pub async fn unpin_all_general_forum_topic_messages(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("unpinAllGeneralForumTopicMessages", params)
            .await
    }

    pub async fn edit_general_forum_topic(&self, chat_id: ChatId, name: &str) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
        ];
        self.do_post("editGeneralForumTopic", params).await
    }

    pub async fn close_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("closeGeneralForumTopic", params).await
    }

    pub async fn reopen_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("reopenGeneralForumTopic", params).await
    }

    pub async fn hide_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("hideGeneralForumTopic", params).await
    }

    pub async fn unhide_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("unhideGeneralForumTopic", params).await
    }
}
