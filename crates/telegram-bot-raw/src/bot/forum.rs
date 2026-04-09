use super::{push_opt, push_opt_str, Bot, ChatId, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::forum_topic;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Forum topics
    // ======================================================================

    /// Use this method to create a topic in a forum supergroup chat.
    ///
    /// Calls the Telegram `createForumTopic` API method.
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

    /// Use this method to edit name and icon of a topic in a forum supergroup chat.
    ///
    /// Calls the Telegram `editForumTopic` API method.
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

    /// Use this method to close an open topic in a forum supergroup chat.
    ///
    /// Calls the Telegram `closeForumTopic` API method.
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

    /// Use this method to reopen a closed topic in a forum supergroup chat.
    ///
    /// Calls the Telegram `reopenForumTopic` API method.
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

    /// Use this method to delete a forum topic along with all its messages.
    ///
    /// Calls the Telegram `deleteForumTopic` API method.
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

    /// Use this method to clear the list of pinned messages in a forum topic.
    ///
    /// Calls the Telegram `unpinAllForumTopicMessages` API method.
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

    /// Use this method to clear the list of pinned messages in a General forum topic.
    ///
    /// Calls the Telegram `unpinAllGeneralForumTopicMessages` API method.
    pub async fn unpin_all_general_forum_topic_messages(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("unpinAllGeneralForumTopicMessages", params)
            .await
    }

    /// Use this method to edit the name of the 'General' topic in a forum supergroup chat.
    ///
    /// Calls the Telegram `editGeneralForumTopic` API method.
    pub async fn edit_general_forum_topic(&self, chat_id: ChatId, name: &str) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
        ];
        self.do_post("editGeneralForumTopic", params).await
    }

    /// Use this method to close an open 'General' topic in a forum supergroup chat.
    ///
    /// Calls the Telegram `closeGeneralForumTopic` API method.
    pub async fn close_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("closeGeneralForumTopic", params).await
    }

    /// Use this method to reopen a closed 'General' topic in a forum supergroup chat.
    ///
    /// Calls the Telegram `reopenGeneralForumTopic` API method.
    pub async fn reopen_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("reopenGeneralForumTopic", params).await
    }

    /// Use this method to hide the 'General' topic in a forum supergroup chat.
    ///
    /// Calls the Telegram `hideGeneralForumTopic` API method.
    pub async fn hide_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("hideGeneralForumTopic", params).await
    }

    /// Use this method to unhide the 'General' topic in a forum supergroup chat.
    ///
    /// Calls the Telegram `unhideGeneralForumTopic` API method.
    pub async fn unhide_general_forum_topic(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("unhideGeneralForumTopic", params).await
    }
}
