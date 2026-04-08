use super::{push_opt, push_opt_str, Bot, ChatId, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{input_checklist, message, message_entity, reply, suggested_post};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Sending other content
    // ======================================================================

    pub(crate) async fn send_location_raw(
        &self,
        chat_id: ChatId,
        latitude: f64,
        longitude: f64,
        horizontal_accuracy: Option<f64>,
        live_period: Option<i64>,
        heading: Option<i64>,
        proximity_alert_radius: Option<i64>,
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
            RequestParameter::new("latitude", serde_json::to_value(latitude)?),
            RequestParameter::new("longitude", serde_json::to_value(longitude)?),
        ];
        push_opt(&mut params, "horizontal_accuracy", &horizontal_accuracy)?;
        push_opt(&mut params, "live_period", &live_period)?;
        push_opt(&mut params, "heading", &heading)?;
        push_opt(
            &mut params,
            "proximity_alert_radius",
            &proximity_alert_radius,
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
        self.do_post("sendLocation", params).await
    }

    pub(crate) async fn send_venue_raw(
        &self,
        chat_id: ChatId,
        latitude: f64,
        longitude: f64,
        title: &str,
        address: &str,
        foursquare_id: Option<&str>,
        foursquare_type: Option<&str>,
        google_place_id: Option<&str>,
        google_place_type: Option<&str>,
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
            RequestParameter::new("latitude", serde_json::to_value(latitude)?),
            RequestParameter::new("longitude", serde_json::to_value(longitude)?),
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
            RequestParameter::new("address", serde_json::Value::String(address.to_owned())),
        ];
        push_opt_str(&mut params, "foursquare_id", foursquare_id);
        push_opt_str(&mut params, "foursquare_type", foursquare_type);
        push_opt_str(&mut params, "google_place_id", google_place_id);
        push_opt_str(&mut params, "google_place_type", google_place_type);
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
        self.do_post("sendVenue", params).await
    }

    pub(crate) async fn send_contact_raw(
        &self,
        chat_id: ChatId,
        phone_number: &str,
        first_name: &str,
        last_name: Option<&str>,
        vcard: Option<&str>,
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
            RequestParameter::new(
                "phone_number",
                serde_json::Value::String(phone_number.to_owned()),
            ),
            RequestParameter::new(
                "first_name",
                serde_json::Value::String(first_name.to_owned()),
            ),
        ];
        push_opt_str(&mut params, "last_name", last_name);
        push_opt_str(&mut params, "vcard", vcard);
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
        self.do_post("sendContact", params).await
    }

    pub(crate) async fn send_poll_raw(
        &self,
        chat_id: ChatId,
        question: &str,
        options: Vec<serde_json::Value>,
        is_anonymous: Option<bool>,
        poll_type: Option<&str>,
        allows_multiple_answers: Option<bool>,
        correct_option_id: Option<i64>,
        explanation: Option<&str>,
        explanation_parse_mode: Option<&str>,
        explanation_entities: Option<Vec<message_entity::MessageEntity>>,
        open_period: Option<i64>,
        close_date: Option<i64>,
        is_closed: Option<bool>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        question_parse_mode: Option<&str>,
        question_entities: Option<Vec<message_entity::MessageEntity>>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("question", serde_json::Value::String(question.to_owned())),
            RequestParameter::new("options", serde_json::to_value(&options)?),
        ];
        push_opt(&mut params, "is_anonymous", &is_anonymous)?;
        push_opt_str(&mut params, "type", poll_type);
        push_opt(
            &mut params,
            "allows_multiple_answers",
            &allows_multiple_answers,
        )?;
        push_opt(&mut params, "correct_option_id", &correct_option_id)?;
        push_opt_str(&mut params, "explanation", explanation);
        push_opt_str(
            &mut params,
            "explanation_parse_mode",
            explanation_parse_mode,
        );
        push_opt(&mut params, "explanation_entities", &explanation_entities)?;
        push_opt(&mut params, "open_period", &open_period)?;
        push_opt(&mut params, "close_date", &close_date)?;
        push_opt(&mut params, "is_closed", &is_closed)?;
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
        push_opt_str(&mut params, "question_parse_mode", question_parse_mode);
        push_opt(&mut params, "question_entities", &question_entities)?;
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
        self.do_post("sendPoll", params).await
    }

    pub(crate) async fn send_dice_raw(
        &self,
        chat_id: ChatId,
        emoji: Option<&str>,
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
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt_str(&mut params, "emoji", emoji);
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
        self.do_post("sendDice", params).await
    }

    pub(crate) async fn send_chat_action_raw(
        &self,
        chat_id: ChatId,
        action: &str,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("action", serde_json::Value::String(action.to_owned())),
        ];
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        self.do_post("sendChatAction", params).await
    }

    pub async fn send_checklist(
        &self,
        business_connection_id: &str,
        chat_id: i64,
        checklist: input_checklist::InputChecklist,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        message_effect_id: Option<&str>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("checklist", serde_json::to_value(&checklist)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        self.do_post("sendChecklist", params).await
    }
}
