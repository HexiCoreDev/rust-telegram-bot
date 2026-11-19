//! Builder pattern for Widget and chat share theme API methods (Bot API 10.4).
//!
//! Mirrors the pattern established in [`bot_builders`]: create a builder
//! from a `Bot` factory method, set the optional parameters, then await it.

#![allow(clippy::too_many_arguments)]

use crate::bot::{Bot, ChatId};
use crate::error::Result;
use crate::types::{chat_share_theme, message, message_entity, reply};
use serde::Serialize;

macro_rules! impl_into_future {
    ($builder:ident, $output:ty) => {
        impl<'a> std::future::IntoFuture for $builder<'a> {
            type Output = Result<$output>;
            type IntoFuture =
                std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'a>>;
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send())
            }
        }
    };
}

// =========================================================================
// SendWidgetBuilder
// =========================================================================

/// Builder for the [`Bot::send_widget`] API method.
#[derive(Serialize)]
pub struct SendWidgetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    html: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_queries: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<reply::ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
}

impl<'a> SendWidgetBuilder<'a> {
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `width` parameter.
    pub fn width(mut self, val: i64) -> Self {
        self.width = Some(val);
        self
    }
    /// Sets the `height` parameter.
    pub fn height(mut self, val: i64) -> Self {
        self.height = Some(val);
        self
    }
    /// Sets the `public_data` parameter.
    pub fn public_data(mut self, val: impl Into<String>) -> Self {
        self.public_data = Some(val.into());
        self
    }
    /// Sets the `chat_data` parameter.
    pub fn chat_data(mut self, val: impl Into<String>) -> Self {
        self.chat_data = Some(val.into());
        self
    }
    /// Sets the `caption` parameter.
    pub fn caption(mut self, val: impl Into<String>) -> Self {
        self.caption = Some(val.into());
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `caption_entities` parameter.
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }
    /// Sets the `show_caption_above_media` parameter.
    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = Some(val);
        self
    }
    /// Sets the `allow_queries` parameter.
    pub fn allow_queries(mut self, val: bool) -> Self {
        self.allow_queries = Some(val);
        self
    }
    /// Sets the `is_shared` parameter.
    pub fn is_shared(mut self, val: bool) -> Self {
        self.is_shared = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("sendWidget", &payload).await
    }
}

impl_into_future!(SendWidgetBuilder, message::Message);

// =========================================================================
// SendSharedWidgetBuilder
// =========================================================================

/// Builder for the [`Bot::send_shared_widget`] API method.
#[derive(Serialize)]
pub struct SendSharedWidgetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    widget_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<reply::ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<serde_json::Value>,
}

impl<'a> SendSharedWidgetBuilder<'a> {
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `chat_data` parameter.
    pub fn chat_data(mut self, val: impl Into<String>) -> Self {
        self.chat_data = Some(val.into());
        self
    }
    /// Sets the `caption` parameter.
    pub fn caption(mut self, val: impl Into<String>) -> Self {
        self.caption = Some(val.into());
        self
    }
    /// Sets the `parse_mode` parameter.
    pub fn parse_mode(mut self, val: impl Into<String>) -> Self {
        self.parse_mode = Some(val.into());
        self
    }
    /// Sets the `caption_entities` parameter.
    pub fn caption_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }
    /// Sets the `show_caption_above_media` parameter.
    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = Some(val);
        self
    }
    /// Sets the `disable_notification` parameter.
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("sendSharedWidget", &payload).await
    }
}

impl_into_future!(SendSharedWidgetBuilder, message::Message);

// =========================================================================
// AnswerWidgetQueryBuilder
// =========================================================================

/// Builder for the [`Bot::answer_widget_query`] API method.
#[derive(Serialize)]
pub struct AnswerWidgetQueryBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    widget_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
}

impl<'a> AnswerWidgetQueryBuilder<'a> {
    /// Sets the `data` parameter.
    pub fn data(mut self, val: impl Into<String>) -> Self {
        self.data = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot.do_post_json("answerWidgetQuery", &payload).await
    }
}

impl_into_future!(AnswerWidgetQueryBuilder, bool);

// =========================================================================
// EditMessageWidgetDataBuilder
// =========================================================================

/// Builder for the [`Bot::edit_message_widget_data`] API method.
#[derive(Serialize)]
pub struct EditMessageWidgetDataBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_data: Option<String>,
}

impl<'a> EditMessageWidgetDataBuilder<'a> {
    /// Sets the `chat_id` parameter.
    pub fn chat_id(mut self, val: impl Into<ChatId>) -> Self {
        self.chat_id = Some(val.into());
        self
    }
    /// Sets the `message_id` parameter.
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = Some(val);
        self
    }
    /// Sets the `inline_message_id` parameter.
    pub fn inline_message_id(mut self, val: impl Into<String>) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }
    /// Sets the `public_data` parameter.
    pub fn public_data(mut self, val: impl Into<String>) -> Self {
        self.public_data = Some(val.into());
        self
    }
    /// Sets the `chat_data` parameter.
    pub fn chat_data(mut self, val: impl Into<String>) -> Self {
        self.chat_data = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("editMessageWidgetData", &payload)
            .await
    }
}

impl_into_future!(EditMessageWidgetDataBuilder, bool);

// =========================================================================
// EditSharedWidgetDataBuilder
// =========================================================================

/// Builder for the [`Bot::edit_shared_widget_data`] API method.
#[derive(Serialize)]
pub struct EditSharedWidgetDataBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    widget_id: String,
    public_data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<i64>,
}

impl<'a> EditSharedWidgetDataBuilder<'a> {
    /// Sets the `cache_time` parameter.
    pub fn cache_time(mut self, val: i64) -> Self {
        self.cache_time = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let payload = serde_json::to_vec(&self)?;
        self.bot
            .do_post_json("editSharedWidgetData", &payload)
            .await
    }
}

impl_into_future!(EditSharedWidgetDataBuilder, bool);

// =========================================================================
// GetChatShareThemeBackdropsBuilder
// =========================================================================

/// Builder for the [`Bot::get_chat_share_theme_backdrops`] API method.
pub struct GetChatShareThemeBackdropsBuilder<'a> {
    bot: &'a Bot,
}

impl<'a> GetChatShareThemeBackdropsBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<chat_share_theme::ChatShareThemeBackdrops> {
        self.bot
            .do_api_request("getChatShareThemeBackdrops", Vec::new())
            .await
    }
}

impl_into_future!(
    GetChatShareThemeBackdropsBuilder,
    chat_share_theme::ChatShareThemeBackdrops
);

// =========================================================================
// Bot factory methods
// =========================================================================

impl Bot {
    /// Build a `sendWidget` request.
    pub fn send_widget(
        &self,
        chat_id: impl Into<ChatId>,
        html: impl Into<String>,
    ) -> SendWidgetBuilder<'_> {
        SendWidgetBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_thread_id: None,
            html: html.into(),
            width: None,
            height: None,
            public_data: None,
            chat_data: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            allow_queries: None,
            is_shared: None,
            disable_notification: None,
            protect_content: None,
            allow_paid_broadcast: None,
            message_effect_id: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    /// Build a `sendSharedWidget` request.
    pub fn send_shared_widget(
        &self,
        chat_id: impl Into<ChatId>,
        widget_id: impl Into<String>,
    ) -> SendSharedWidgetBuilder<'_> {
        SendSharedWidgetBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_thread_id: None,
            widget_id: widget_id.into(),
            chat_data: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            disable_notification: None,
            protect_content: None,
            allow_paid_broadcast: None,
            message_effect_id: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    /// Build an `answerWidgetQuery` request.
    pub fn answer_widget_query(
        &self,
        widget_query_id: impl Into<String>,
    ) -> AnswerWidgetQueryBuilder<'_> {
        AnswerWidgetQueryBuilder {
            bot: self,
            widget_query_id: widget_query_id.into(),
            data: None,
        }
    }

    /// Build an `editMessageWidgetData` request.
    ///
    /// Identify the message either with `chat_id` + `message_id` or with `inline_message_id`.
    pub fn edit_message_widget_data(&self) -> EditMessageWidgetDataBuilder<'_> {
        EditMessageWidgetDataBuilder {
            bot: self,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            public_data: None,
            chat_data: None,
        }
    }

    /// Build an `editSharedWidgetData` request.
    pub fn edit_shared_widget_data(
        &self,
        widget_id: impl Into<String>,
        public_data: impl Into<String>,
    ) -> EditSharedWidgetDataBuilder<'_> {
        EditSharedWidgetDataBuilder {
            bot: self,
            widget_id: widget_id.into(),
            public_data: public_data.into(),
            cache_time: None,
        }
    }

    /// Build a `getChatShareThemeBackdrops` request.
    pub fn get_chat_share_theme_backdrops(&self) -> GetChatShareThemeBackdropsBuilder<'_> {
        GetChatShareThemeBackdropsBuilder { bot: self }
    }
}
