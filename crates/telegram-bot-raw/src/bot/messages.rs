use super::{push_opt, push_opt_str, Bot, ChatId, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{link_preview_options, message, message_entity, message_id, reply, suggested_post};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Sending messages
    // ======================================================================

    pub(crate) async fn send_message_raw(
        &self,
        chat_id: ChatId,
        text: &str,
        parse_mode: Option<&str>,
        entities: Option<Vec<message_entity::MessageEntity>>,
        link_preview_options: Option<link_preview_options::LinkPreviewOptions>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("text", serde_json::Value::String(text.to_owned())),
        ];
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "entities", &entities)?;
        push_opt(&mut params, "link_preview_options", &link_preview_options)?;
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
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &suggested_post_parameters,
        )?;
        self.do_post("sendMessage", params).await
    }

    pub async fn send_message_draft(
        &self,
        chat_id: i64,
        draft_id: i64,
        text: &str,
        message_thread_id: Option<i64>,
        parse_mode: Option<&str>,
        entities: Option<Vec<message_entity::MessageEntity>>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("draft_id", serde_json::to_value(draft_id)?),
            RequestParameter::new("text", serde_json::Value::String(text.to_owned())),
        ];
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "entities", &entities)?;
        self.do_post("sendMessageDraft", params).await
    }

    pub async fn forward_message(
        &self,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_id: i64,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        message_thread_id: Option<i64>,
        video_start_timestamp: Option<i64>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
        message_effect_id: Option<&str>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("from_chat_id", serde_json::to_value(&from_chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(&mut params, "video_start_timestamp", &video_start_timestamp)?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &suggested_post_parameters,
        )?;
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        self.do_post("forwardMessage", params).await
    }

    pub async fn forward_messages(
        &self,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_ids: Vec<i64>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        message_thread_id: Option<i64>,
        direct_messages_topic_id: Option<i64>,
    ) -> Result<Vec<message_id::MessageId>> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("from_chat_id", serde_json::to_value(&from_chat_id)?),
            RequestParameter::new("message_ids", serde_json::to_value(&message_ids)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &direct_messages_topic_id,
        )?;
        self.do_post("forwardMessages", params).await
    }

    pub async fn copy_message(
        &self,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_id: i64,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        show_caption_above_media: Option<bool>,
        allow_paid_broadcast: Option<bool>,
        video_start_timestamp: Option<i64>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
        message_effect_id: Option<&str>,
    ) -> Result<message_id::MessageId> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("from_chat_id", serde_json::to_value(&from_chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(
            &mut params,
            "show_caption_above_media",
            &show_caption_above_media,
        )?;
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(&mut params, "video_start_timestamp", &video_start_timestamp)?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &suggested_post_parameters,
        )?;
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        self.do_post("copyMessage", params).await
    }

    pub async fn copy_messages(
        &self,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_ids: Vec<i64>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        message_thread_id: Option<i64>,
        remove_caption: Option<bool>,
        direct_messages_topic_id: Option<i64>,
    ) -> Result<Vec<message_id::MessageId>> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("from_chat_id", serde_json::to_value(&from_chat_id)?),
            RequestParameter::new("message_ids", serde_json::to_value(&message_ids)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(&mut params, "remove_caption", &remove_caption)?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &direct_messages_topic_id,
        )?;
        self.do_post("copyMessages", params).await
    }

    pub async fn delete_message(&self, chat_id: ChatId, message_id: i64) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        self.do_post("deleteMessage", params).await
    }

    pub async fn delete_messages(&self, chat_id: ChatId, message_ids: Vec<i64>) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_ids", serde_json::to_value(&message_ids)?),
        ];
        self.do_post("deleteMessages", params).await
    }
}
