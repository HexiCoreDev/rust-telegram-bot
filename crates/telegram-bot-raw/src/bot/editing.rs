use super::{push_opt, push_opt_str, Bot, ChatId, MessageOrBool, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{input_checklist, link_preview_options, message, message_entity, poll};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Editing messages
    // ======================================================================

    pub(crate) async fn edit_message_text_raw(
        &self,
        text: &str,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        parse_mode: Option<&str>,
        entities: Option<Vec<message_entity::MessageEntity>>,
        link_preview_options: Option<link_preview_options::LinkPreviewOptions>,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = vec![RequestParameter::new(
            "text",
            serde_json::Value::String(text.to_owned()),
        )];
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "entities", &entities)?;
        push_opt(&mut params, "link_preview_options", &link_preview_options)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        self.do_post("editMessageText", params).await
    }

    pub(crate) async fn edit_message_caption_raw(
        &self,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        reply_markup: Option<serde_json::Value>,
        show_caption_above_media: Option<bool>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(
            &mut params,
            "show_caption_above_media",
            &show_caption_above_media,
        )?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        self.do_post("editMessageCaption", params).await
    }

    pub(crate) async fn edit_message_media_raw(
        &self,
        media: serde_json::Value,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = vec![RequestParameter::new("media", media)];
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        self.do_post("editMessageMedia", params).await
    }

    pub(crate) async fn edit_message_reply_markup_raw(
        &self,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        self.do_post("editMessageReplyMarkup", params).await
    }

    pub async fn edit_message_live_location(
        &self,
        latitude: f64,
        longitude: f64,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        horizontal_accuracy: Option<f64>,
        heading: Option<i64>,
        proximity_alert_radius: Option<i64>,
        reply_markup: Option<serde_json::Value>,
        live_period: Option<i64>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = vec![
            RequestParameter::new("latitude", serde_json::to_value(latitude)?),
            RequestParameter::new("longitude", serde_json::to_value(longitude)?),
        ];
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt(&mut params, "horizontal_accuracy", &horizontal_accuracy)?;
        push_opt(&mut params, "heading", &heading)?;
        push_opt(
            &mut params,
            "proximity_alert_radius",
            &proximity_alert_radius,
        )?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "live_period", &live_period)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        self.do_post("editMessageLiveLocation", params).await
    }

    pub async fn stop_message_live_location(
        &self,
        chat_id: Option<ChatId>,
        message_id: Option<i64>,
        inline_message_id: Option<&str>,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
    ) -> Result<MessageOrBool> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(&mut params, "inline_message_id", inline_message_id);
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        self.do_post("stopMessageLiveLocation", params).await
    }

    pub async fn edit_message_checklist(
        &self,
        business_connection_id: &str,
        chat_id: i64,
        message_id: i64,
        checklist: input_checklist::InputChecklist,
        reply_markup: Option<serde_json::Value>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
            RequestParameter::new("checklist", serde_json::to_value(&checklist)?),
        ];
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        self.do_post("editMessageChecklist", params).await
    }

    pub async fn stop_poll(
        &self,
        chat_id: ChatId,
        message_id: i64,
        reply_markup: Option<serde_json::Value>,
        business_connection_id: Option<&str>,
    ) -> Result<poll::Poll> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        self.do_post("stopPoll", params).await
    }
}
