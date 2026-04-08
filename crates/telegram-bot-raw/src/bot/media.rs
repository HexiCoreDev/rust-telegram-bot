use super::{input_file_param, push_opt, push_opt_file, push_opt_str, Bot, ChatId, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{files, message, message_entity, reply, suggested_post};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Sending media
    // ======================================================================

    pub(crate) async fn send_photo_raw(
        &self,
        chat_id: ChatId,
        photo: files::input_file::InputFile,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        has_spoiler: Option<bool>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        show_caption_above_media: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("photo", photo),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(&mut params, "has_spoiler", &has_spoiler)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(
            &mut params,
            "show_caption_above_media",
            &show_caption_above_media,
        )?;
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
        self.do_post("sendPhoto", params).await
    }

    pub(crate) async fn send_audio_raw(
        &self,
        chat_id: ChatId,
        audio: files::input_file::InputFile,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        duration: Option<i64>,
        performer: Option<&str>,
        title: Option<&str>,
        thumbnail: Option<files::input_file::InputFile>,
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
            input_file_param("audio", audio),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "duration", &duration)?;
        push_opt_str(&mut params, "performer", performer);
        push_opt_str(&mut params, "title", title);
        push_opt_file(&mut params, "thumbnail", thumbnail);
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
        self.do_post("sendAudio", params).await
    }

    pub(crate) async fn send_document_raw(
        &self,
        chat_id: ChatId,
        document: files::input_file::InputFile,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        disable_content_type_detection: Option<bool>,
        thumbnail: Option<files::input_file::InputFile>,
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
            input_file_param("document", document),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(
            &mut params,
            "disable_content_type_detection",
            &disable_content_type_detection,
        )?;
        push_opt_file(&mut params, "thumbnail", thumbnail);
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
        self.do_post("sendDocument", params).await
    }

    pub(crate) async fn send_video_raw(
        &self,
        chat_id: ChatId,
        video: files::input_file::InputFile,
        duration: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        supports_streaming: Option<bool>,
        thumbnail: Option<files::input_file::InputFile>,
        has_spoiler: Option<bool>,
        show_caption_above_media: Option<bool>,
        cover: Option<files::input_file::InputFile>,
        start_timestamp: Option<i64>,
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
            input_file_param("video", video),
        ];
        push_opt(&mut params, "duration", &duration)?;
        push_opt(&mut params, "width", &width)?;
        push_opt(&mut params, "height", &height)?;
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "supports_streaming", &supports_streaming)?;
        push_opt_file(&mut params, "thumbnail", thumbnail);
        push_opt(&mut params, "has_spoiler", &has_spoiler)?;
        push_opt(
            &mut params,
            "show_caption_above_media",
            &show_caption_above_media,
        )?;
        push_opt_file(&mut params, "cover", cover);
        push_opt(&mut params, "start_timestamp", &start_timestamp)?;
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
        self.do_post("sendVideo", params).await
    }

    pub(crate) async fn send_animation_raw(
        &self,
        chat_id: ChatId,
        animation: files::input_file::InputFile,
        duration: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        thumbnail: Option<files::input_file::InputFile>,
        has_spoiler: Option<bool>,
        show_caption_above_media: Option<bool>,
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
            input_file_param("animation", animation),
        ];
        push_opt(&mut params, "duration", &duration)?;
        push_opt(&mut params, "width", &width)?;
        push_opt(&mut params, "height", &height)?;
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt_file(&mut params, "thumbnail", thumbnail);
        push_opt(&mut params, "has_spoiler", &has_spoiler)?;
        push_opt(
            &mut params,
            "show_caption_above_media",
            &show_caption_above_media,
        )?;
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
        self.do_post("sendAnimation", params).await
    }

    pub(crate) async fn send_voice_raw(
        &self,
        chat_id: ChatId,
        voice: files::input_file::InputFile,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        duration: Option<i64>,
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
            input_file_param("voice", voice),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "duration", &duration)?;
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
        self.do_post("sendVoice", params).await
    }

    pub(crate) async fn send_video_note_raw(
        &self,
        chat_id: ChatId,
        video_note: files::input_file::InputFile,
        duration: Option<i64>,
        length: Option<i64>,
        thumbnail: Option<files::input_file::InputFile>,
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
            input_file_param("video_note", video_note),
        ];
        push_opt(&mut params, "duration", &duration)?;
        push_opt(&mut params, "length", &length)?;
        push_opt_file(&mut params, "thumbnail", thumbnail);
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
        self.do_post("sendVideoNote", params).await
    }

    pub async fn send_media_group(
        &self,
        chat_id: ChatId,
        media: Vec<serde_json::Value>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        message_thread_id: Option<i64>,
        reply_parameters: Option<reply::ReplyParameters>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<Vec<message::Message>> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("media", serde_json::to_value(&media)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
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
        self.do_post("sendMediaGroup", params).await
    }

    pub async fn send_paid_media(
        &self,
        chat_id: ChatId,
        star_count: i64,
        media: Vec<serde_json::Value>,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        show_caption_above_media: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
        payload: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
        message_thread_id: Option<i64>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("star_count", serde_json::to_value(star_count)?),
            RequestParameter::new("media", serde_json::to_value(&media)?),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(
            &mut params,
            "show_caption_above_media",
            &show_caption_above_media,
        )?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        push_opt_str(&mut params, "payload", payload);
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
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        self.do_post("sendPaidMedia", params).await
    }
}
