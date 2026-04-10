//! Builder pattern for Telegram Bot API methods -- batch 4.
//!
//! Covers: editing (live location, checklist, stop poll), games, inline,
//! media group / paid media, core Bot methods (get_me, log_out, close, etc.),
//! send_checklist, passport, reactions, stories, suggested posts, user profile,
//! and verification.

#![allow(clippy::too_many_arguments)]

use crate::bot::{Bot, ChatId, MessageOrBool};
use crate::error::Result;
use crate::types::{
    chat_boost, games, input_checklist, message, message_entity, poll, reply, sent_web_app_message,
    story, suggested_post, update, user, user_profile_audios, user_profile_photos, webhook_info,
};
use serde::Serialize;

// ---------------------------------------------------------------------------
// Macro
// ---------------------------------------------------------------------------

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
// editing.rs builders
// =========================================================================

// -------------------------------------------------------------------------
// EditMessageLiveLocationBuilder
// -------------------------------------------------------------------------

/// Builder for the [`editMessageLiveLocation`] API method.
pub struct EditMessageLiveLocationBuilder<'a> {
    bot: &'a Bot,
    latitude: f64,
    longitude: f64,
    chat_id: Option<ChatId>,
    message_id: Option<i64>,
    inline_message_id: Option<String>,
    horizontal_accuracy: Option<f64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
    reply_markup: Option<serde_json::Value>,
    live_period: Option<i64>,
    business_connection_id: Option<String>,
}

impl<'a> EditMessageLiveLocationBuilder<'a> {
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
    /// Sets the `horizontal_accuracy` parameter.
    pub fn horizontal_accuracy(mut self, val: f64) -> Self {
        self.horizontal_accuracy = Some(val);
        self
    }
    /// Sets the `heading` parameter.
    pub fn heading(mut self, val: i64) -> Self {
        self.heading = Some(val);
        self
    }
    /// Sets the `proximity_alert_radius` parameter.
    pub fn proximity_alert_radius(mut self, val: i64) -> Self {
        self.proximity_alert_radius = Some(val);
        self
    }
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `live_period` parameter.
    pub fn live_period(mut self, val: i64) -> Self {
        self.live_period = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<MessageOrBool> {
        self.bot
            .edit_message_live_location_raw(
                self.latitude,
                self.longitude,
                self.chat_id,
                self.message_id,
                self.inline_message_id.as_deref(),
                self.horizontal_accuracy,
                self.heading,
                self.proximity_alert_radius,
                self.reply_markup,
                self.live_period,
                self.business_connection_id.as_deref(),
            )
            .await
    }
}

impl_into_future!(EditMessageLiveLocationBuilder, MessageOrBool);

// -------------------------------------------------------------------------
// StopMessageLiveLocationBuilder
// -------------------------------------------------------------------------

/// Builder for the [`stopMessageLiveLocation`] API method.
pub struct StopMessageLiveLocationBuilder<'a> {
    bot: &'a Bot,
    chat_id: Option<ChatId>,
    message_id: Option<i64>,
    inline_message_id: Option<String>,
    reply_markup: Option<serde_json::Value>,
    business_connection_id: Option<String>,
}

impl<'a> StopMessageLiveLocationBuilder<'a> {
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
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<MessageOrBool> {
        self.bot
            .stop_message_live_location_raw(
                self.chat_id,
                self.message_id,
                self.inline_message_id.as_deref(),
                self.reply_markup,
                self.business_connection_id.as_deref(),
            )
            .await
    }
}

impl_into_future!(StopMessageLiveLocationBuilder, MessageOrBool);

// -------------------------------------------------------------------------
// EditMessageChecklistBuilder
// -------------------------------------------------------------------------

/// Builder for the [`editMessageChecklist`] API method.
pub struct EditMessageChecklistBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    chat_id: i64,
    message_id: i64,
    checklist: input_checklist::InputChecklist,
    reply_markup: Option<serde_json::Value>,
}

impl<'a> EditMessageChecklistBuilder<'a> {
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        self.bot
            .edit_message_checklist_raw(
                &self.business_connection_id,
                self.chat_id,
                self.message_id,
                self.checklist,
                self.reply_markup,
            )
            .await
    }
}

impl_into_future!(EditMessageChecklistBuilder, message::Message);

// -------------------------------------------------------------------------
// StopPollBuilder
// -------------------------------------------------------------------------

/// Builder for the [`stopPoll`] API method.
pub struct StopPollBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    message_id: i64,
    reply_markup: Option<serde_json::Value>,
    business_connection_id: Option<String>,
}

impl<'a> StopPollBuilder<'a> {
    /// Sets the `reply_markup` parameter.
    pub fn reply_markup(mut self, val: serde_json::Value) -> Self {
        self.reply_markup = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<poll::Poll> {
        self.bot
            .stop_poll_raw(
                self.chat_id,
                self.message_id,
                self.reply_markup,
                self.business_connection_id.as_deref(),
            )
            .await
    }
}

impl_into_future!(StopPollBuilder, poll::Poll);

// =========================================================================
// games_methods.rs builders
// =========================================================================

// -------------------------------------------------------------------------
// SendGameBuilder
// -------------------------------------------------------------------------

/// Builder for the [`sendGame`] API method.
pub struct SendGameBuilder<'a> {
    bot: &'a Bot,
    chat_id: i64,
    game_short_name: String,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    message_thread_id: Option<i64>,
    business_connection_id: Option<String>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
}

impl<'a> SendGameBuilder<'a> {
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
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        self.bot
            .send_game_raw(
                self.chat_id,
                &self.game_short_name,
                self.disable_notification,
                self.protect_content,
                self.reply_parameters,
                self.reply_markup,
                self.message_thread_id,
                self.business_connection_id.as_deref(),
                self.message_effect_id.as_deref(),
                self.allow_paid_broadcast,
            )
            .await
    }
}

impl_into_future!(SendGameBuilder, message::Message);

// -------------------------------------------------------------------------
// SetGameScoreBuilder
// -------------------------------------------------------------------------

/// Builder for the [`setGameScore`] API method.
pub struct SetGameScoreBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    score: i64,
    force: Option<bool>,
    disable_edit_message: Option<bool>,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<String>,
}

impl<'a> SetGameScoreBuilder<'a> {
    /// Sets the `force` parameter.
    pub fn force(mut self, val: bool) -> Self {
        self.force = Some(val);
        self
    }
    /// Sets the `disable_edit_message` parameter.
    pub fn disable_edit_message(mut self, val: bool) -> Self {
        self.disable_edit_message = Some(val);
        self
    }
    /// Sets the `chat_id` parameter.
    pub fn chat_id(mut self, val: i64) -> Self {
        self.chat_id = Some(val);
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

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<MessageOrBool> {
        self.bot
            .set_game_score_raw(
                self.user_id,
                self.score,
                self.force,
                self.disable_edit_message,
                self.chat_id,
                self.message_id,
                self.inline_message_id.as_deref(),
            )
            .await
    }
}

impl_into_future!(SetGameScoreBuilder, MessageOrBool);

// -------------------------------------------------------------------------
// GetGameHighScoresBuilder
// -------------------------------------------------------------------------

/// Builder for the [`getGameHighScores`] API method.
pub struct GetGameHighScoresBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<String>,
}

impl<'a> GetGameHighScoresBuilder<'a> {
    /// Sets the `chat_id` parameter.
    pub fn chat_id(mut self, val: i64) -> Self {
        self.chat_id = Some(val);
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

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<Vec<games::game_high_score::GameHighScore>> {
        self.bot
            .get_game_high_scores_raw(
                self.user_id,
                self.chat_id,
                self.message_id,
                self.inline_message_id.as_deref(),
            )
            .await
    }
}

impl_into_future!(
    GetGameHighScoresBuilder,
    Vec<games::game_high_score::GameHighScore>
);

// =========================================================================
// inline_methods.rs builders
// =========================================================================

// -------------------------------------------------------------------------
// SavePreparedInlineMessageBuilder
// -------------------------------------------------------------------------

/// Builder for the [`savePreparedInlineMessage`] API method.
pub struct SavePreparedInlineMessageBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    result: serde_json::Value,
    allow_user_chats: Option<bool>,
    allow_bot_chats: Option<bool>,
    allow_group_chats: Option<bool>,
    allow_channel_chats: Option<bool>,
}

impl<'a> SavePreparedInlineMessageBuilder<'a> {
    /// Sets the `allow_user_chats` parameter.
    pub fn allow_user_chats(mut self, val: bool) -> Self {
        self.allow_user_chats = Some(val);
        self
    }
    /// Sets the `allow_bot_chats` parameter.
    pub fn allow_bot_chats(mut self, val: bool) -> Self {
        self.allow_bot_chats = Some(val);
        self
    }
    /// Sets the `allow_group_chats` parameter.
    pub fn allow_group_chats(mut self, val: bool) -> Self {
        self.allow_group_chats = Some(val);
        self
    }
    /// Sets the `allow_channel_chats` parameter.
    pub fn allow_channel_chats(mut self, val: bool) -> Self {
        self.allow_channel_chats = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<serde_json::Value> {
        self.bot
            .save_prepared_inline_message_raw(
                self.user_id,
                self.result,
                self.allow_user_chats,
                self.allow_bot_chats,
                self.allow_group_chats,
                self.allow_channel_chats,
            )
            .await
    }
}

impl_into_future!(SavePreparedInlineMessageBuilder, serde_json::Value);

// -------------------------------------------------------------------------
// AnswerWebAppQueryBuilder
// -------------------------------------------------------------------------

/// Builder for the [`answerWebAppQuery`] API method.
pub struct AnswerWebAppQueryBuilder<'a> {
    bot: &'a Bot,
    web_app_query_id: String,
    result: serde_json::Value,
}

impl<'a> AnswerWebAppQueryBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<sent_web_app_message::SentWebAppMessage> {
        self.bot
            .answer_web_app_query_raw(&self.web_app_query_id, self.result)
            .await
    }
}

impl_into_future!(
    AnswerWebAppQueryBuilder,
    sent_web_app_message::SentWebAppMessage
);

// =========================================================================
// media.rs builders (send_media_group, send_paid_media)
// =========================================================================

// -------------------------------------------------------------------------
// SendMediaGroupBuilder
// -------------------------------------------------------------------------

/// Builder for the [`sendMediaGroup`] API method.
pub struct SendMediaGroupBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    media: Vec<serde_json::Value>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    message_thread_id: Option<i64>,
    reply_parameters: Option<reply::ReplyParameters>,
    business_connection_id: Option<String>,
    message_effect_id: Option<String>,
    allow_paid_broadcast: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
}

impl<'a> SendMediaGroupBuilder<'a> {
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
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }
    /// Sets the `reply_parameters` parameter.
    pub fn reply_parameters(mut self, val: reply::ReplyParameters) -> Self {
        self.reply_parameters = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `message_effect_id` parameter.
    pub fn message_effect_id(mut self, val: impl Into<String>) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<Vec<message::Message>> {
        self.bot
            .send_media_group_raw(
                self.chat_id,
                self.media,
                self.disable_notification,
                self.protect_content,
                self.message_thread_id,
                self.reply_parameters,
                self.business_connection_id.as_deref(),
                self.message_effect_id.as_deref(),
                self.allow_paid_broadcast,
                self.direct_messages_topic_id,
                self.suggested_post_parameters,
            )
            .await
    }
}

impl_into_future!(SendMediaGroupBuilder, Vec<message::Message>);

// -------------------------------------------------------------------------
// SendPaidMediaBuilder
// -------------------------------------------------------------------------

/// Builder for the [`sendPaidMedia`] API method.
pub struct SendPaidMediaBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    star_count: i64,
    media: Vec<serde_json::Value>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    show_caption_above_media: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
    business_connection_id: Option<String>,
    payload: Option<String>,
    allow_paid_broadcast: Option<bool>,
    direct_messages_topic_id: Option<i64>,
    suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    message_thread_id: Option<i64>,
}

impl<'a> SendPaidMediaBuilder<'a> {
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
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }
    /// Sets the `payload` parameter.
    pub fn payload(mut self, val: impl Into<String>) -> Self {
        self.payload = Some(val.into());
        self
    }
    /// Sets the `allow_paid_broadcast` parameter.
    pub fn allow_paid_broadcast(mut self, val: bool) -> Self {
        self.allow_paid_broadcast = Some(val);
        self
    }
    /// Sets the `direct_messages_topic_id` parameter.
    pub fn direct_messages_topic_id(mut self, val: i64) -> Self {
        self.direct_messages_topic_id = Some(val);
        self
    }
    /// Sets the `suggested_post_parameters` parameter.
    pub fn suggested_post_parameters(
        mut self,
        val: suggested_post::SuggestedPostParameters,
    ) -> Self {
        self.suggested_post_parameters = Some(val);
        self
    }
    /// Sets the `message_thread_id` parameter.
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<message::Message> {
        self.bot
            .send_paid_media_raw(
                self.chat_id,
                self.star_count,
                self.media,
                self.caption.as_deref(),
                self.parse_mode.as_deref(),
                self.caption_entities,
                self.show_caption_above_media,
                self.disable_notification,
                self.protect_content,
                self.reply_parameters,
                self.reply_markup,
                self.business_connection_id.as_deref(),
                self.payload.as_deref(),
                self.allow_paid_broadcast,
                self.direct_messages_topic_id,
                self.suggested_post_parameters,
                self.message_thread_id,
            )
            .await
    }
}

impl_into_future!(SendPaidMediaBuilder, message::Message);

// =========================================================================
// mod.rs builders (core Bot methods)
// =========================================================================

// -------------------------------------------------------------------------
// GetMeBuilder
// -------------------------------------------------------------------------

/// Builder for the [`getMe`] API method.
#[derive(Serialize)]
pub struct GetMeBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
}

impl<'a> GetMeBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<user::User> {
        self.bot.get_me_raw().await
    }
}

impl_into_future!(GetMeBuilder, user::User);

// -------------------------------------------------------------------------
// LogOutBuilder
// -------------------------------------------------------------------------

/// Builder for the [`logOut`] API method.
#[derive(Serialize)]
pub struct LogOutBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
}

impl<'a> LogOutBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot.log_out_raw().await
    }
}

impl_into_future!(LogOutBuilder, bool);

// -------------------------------------------------------------------------
// CloseBuilder
// -------------------------------------------------------------------------

/// Builder for the [`close`] API method.
#[derive(Serialize)]
pub struct CloseBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
}

impl<'a> CloseBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot.close_raw().await
    }
}

impl_into_future!(CloseBuilder, bool);

// -------------------------------------------------------------------------
// GetUpdatesBuilder
// -------------------------------------------------------------------------

/// Builder for the [`getUpdates`] API method.
pub struct GetUpdatesBuilder<'a> {
    bot: &'a Bot,
    offset: Option<i64>,
    limit: Option<i32>,
    timeout: Option<i32>,
    allowed_updates: Option<Vec<String>>,
}

impl<'a> GetUpdatesBuilder<'a> {
    /// Sets the `offset` parameter.
    pub fn offset(mut self, val: i64) -> Self {
        self.offset = Some(val);
        self
    }
    /// Sets the `limit` parameter.
    pub fn limit(mut self, val: i32) -> Self {
        self.limit = Some(val);
        self
    }
    /// Sets the `timeout` parameter.
    pub fn timeout(mut self, val: i32) -> Self {
        self.timeout = Some(val);
        self
    }
    /// Sets the `allowed_updates` parameter.
    pub fn allowed_updates(mut self, val: Vec<String>) -> Self {
        self.allowed_updates = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<Vec<update::Update>> {
        self.bot
            .get_updates_raw(self.offset, self.limit, self.timeout, self.allowed_updates)
            .await
    }
}

impl_into_future!(GetUpdatesBuilder, Vec<update::Update>);

// -------------------------------------------------------------------------
// GetWebhookInfoBuilder
// -------------------------------------------------------------------------

/// Builder for the [`getWebhookInfo`] API method.
#[derive(Serialize)]
pub struct GetWebhookInfoBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
}

impl<'a> GetWebhookInfoBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<webhook_info::WebhookInfo> {
        self.bot.get_webhook_info_raw().await
    }
}

impl_into_future!(GetWebhookInfoBuilder, webhook_info::WebhookInfo);

// -------------------------------------------------------------------------
// DownloadFileBuilder
// -------------------------------------------------------------------------

/// Builder for the `download_file` method.
pub struct DownloadFileBuilder<'a> {
    bot: &'a Bot,
    file_path: String,
}

impl<'a> DownloadFileBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<Vec<u8>> {
        self.bot.download_file_raw(&self.file_path).await
    }
}

impl_into_future!(DownloadFileBuilder, Vec<u8>);

// =========================================================================
// other_content.rs builder (send_checklist)
// =========================================================================

// -------------------------------------------------------------------------
// SendChecklistBuilder
// -------------------------------------------------------------------------

/// Builder for the [`sendChecklist`] API method.
pub struct SendChecklistBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    chat_id: i64,
    checklist: input_checklist::InputChecklist,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    message_effect_id: Option<String>,
    reply_parameters: Option<reply::ReplyParameters>,
    reply_markup: Option<serde_json::Value>,
}

impl<'a> SendChecklistBuilder<'a> {
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
        self.bot
            .send_checklist_raw(
                &self.business_connection_id,
                self.chat_id,
                self.checklist,
                self.disable_notification,
                self.protect_content,
                self.message_effect_id.as_deref(),
                self.reply_parameters,
                self.reply_markup,
            )
            .await
    }
}

impl_into_future!(SendChecklistBuilder, message::Message);

// =========================================================================
// passport.rs builder
// =========================================================================

// -------------------------------------------------------------------------
// SetPassportDataErrorsBuilder
// -------------------------------------------------------------------------

/// Builder for the [`setPassportDataErrors`] API method.
pub struct SetPassportDataErrorsBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    errors: Vec<serde_json::Value>,
}

impl<'a> SetPassportDataErrorsBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot
            .set_passport_data_errors_raw(self.user_id, self.errors)
            .await
    }
}

impl_into_future!(SetPassportDataErrorsBuilder, bool);

// =========================================================================
// reactions.rs builders
// =========================================================================

// -------------------------------------------------------------------------
// SetMessageReactionBuilder
// -------------------------------------------------------------------------

/// Builder for the [`setMessageReaction`] API method.
pub struct SetMessageReactionBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    message_id: i64,
    reaction: Option<Vec<serde_json::Value>>,
    is_big: Option<bool>,
}

impl<'a> SetMessageReactionBuilder<'a> {
    /// Sets the `reaction` parameter.
    pub fn reaction(mut self, val: Vec<serde_json::Value>) -> Self {
        self.reaction = Some(val);
        self
    }
    /// Sets the `is_big` parameter.
    pub fn is_big(mut self, val: bool) -> Self {
        self.is_big = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot
            .set_message_reaction_raw(self.chat_id, self.message_id, self.reaction, self.is_big)
            .await
    }
}

impl_into_future!(SetMessageReactionBuilder, bool);

// -------------------------------------------------------------------------
// GetUserChatBoostsBuilder
// -------------------------------------------------------------------------

/// Builder for the [`getUserChatBoosts`] API method.
pub struct GetUserChatBoostsBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    user_id: i64,
}

impl<'a> GetUserChatBoostsBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<chat_boost::UserChatBoosts> {
        self.bot
            .get_user_chat_boosts_raw(self.chat_id, self.user_id)
            .await
    }
}

impl_into_future!(GetUserChatBoostsBuilder, chat_boost::UserChatBoosts);

// =========================================================================
// stories.rs builders
// =========================================================================

// -------------------------------------------------------------------------
// PostStoryBuilder
// -------------------------------------------------------------------------

/// Builder for the [`postStory`] API method.
pub struct PostStoryBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    content: serde_json::Value,
    active_period: i64,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    areas: Option<Vec<serde_json::Value>>,
    post_to_chat_page: Option<bool>,
    protect_content: Option<bool>,
}

impl<'a> PostStoryBuilder<'a> {
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
    /// Sets the `areas` parameter.
    pub fn areas(mut self, val: Vec<serde_json::Value>) -> Self {
        self.areas = Some(val);
        self
    }
    /// Sets the `post_to_chat_page` parameter.
    pub fn post_to_chat_page(mut self, val: bool) -> Self {
        self.post_to_chat_page = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<story::Story> {
        self.bot
            .post_story_raw(
                &self.business_connection_id,
                self.content,
                self.active_period,
                self.caption.as_deref(),
                self.parse_mode.as_deref(),
                self.caption_entities,
                self.areas,
                self.post_to_chat_page,
                self.protect_content,
            )
            .await
    }
}

impl_into_future!(PostStoryBuilder, story::Story);

// -------------------------------------------------------------------------
// EditStoryBuilder
// -------------------------------------------------------------------------

/// Builder for the [`editStory`] API method.
pub struct EditStoryBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    story_id: i64,
    content: serde_json::Value,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<message_entity::MessageEntity>>,
    areas: Option<Vec<serde_json::Value>>,
}

impl<'a> EditStoryBuilder<'a> {
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
    /// Sets the `areas` parameter.
    pub fn areas(mut self, val: Vec<serde_json::Value>) -> Self {
        self.areas = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<story::Story> {
        self.bot
            .edit_story_raw(
                &self.business_connection_id,
                self.story_id,
                self.content,
                self.caption.as_deref(),
                self.parse_mode.as_deref(),
                self.caption_entities,
                self.areas,
            )
            .await
    }
}

impl_into_future!(EditStoryBuilder, story::Story);

// -------------------------------------------------------------------------
// DeleteStoryBuilder
// -------------------------------------------------------------------------

/// Builder for the [`deleteStory`] API method.
pub struct DeleteStoryBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    story_id: i64,
}

impl<'a> DeleteStoryBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot
            .delete_story_raw(&self.business_connection_id, self.story_id)
            .await
    }
}

impl_into_future!(DeleteStoryBuilder, bool);

// -------------------------------------------------------------------------
// RepostStoryBuilder
// -------------------------------------------------------------------------

/// Builder for the [`repostStory`] API method.
pub struct RepostStoryBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    from_chat_id: i64,
    from_story_id: i64,
    active_period: i64,
    post_to_chat_page: Option<bool>,
    protect_content: Option<bool>,
}

impl<'a> RepostStoryBuilder<'a> {
    /// Sets the `post_to_chat_page` parameter.
    pub fn post_to_chat_page(mut self, val: bool) -> Self {
        self.post_to_chat_page = Some(val);
        self
    }
    /// Sets the `protect_content` parameter.
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<story::Story> {
        self.bot
            .repost_story_raw(
                &self.business_connection_id,
                self.from_chat_id,
                self.from_story_id,
                self.active_period,
                self.post_to_chat_page,
                self.protect_content,
            )
            .await
    }
}

impl_into_future!(RepostStoryBuilder, story::Story);

// =========================================================================
// suggested_posts.rs builders
// =========================================================================

// -------------------------------------------------------------------------
// ApproveSuggestedPostBuilder
// -------------------------------------------------------------------------

/// Builder for the [`approveSuggestedPost`] API method.
pub struct ApproveSuggestedPostBuilder<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_id: i64,
    send_date: Option<i64>,
}

impl<'a> ApproveSuggestedPostBuilder<'a> {
    /// Sets the `send_date` parameter.
    pub fn send_date(mut self, val: i64) -> Self {
        self.send_date = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot
            .approve_suggested_post_raw(self.chat_id, self.message_id, self.send_date)
            .await
    }
}

impl_into_future!(ApproveSuggestedPostBuilder, bool);

// -------------------------------------------------------------------------
// DeclineSuggestedPostBuilder
// -------------------------------------------------------------------------

/// Builder for the [`declineSuggestedPost`] API method.
pub struct DeclineSuggestedPostBuilder<'a> {
    bot: &'a Bot,
    chat_id: i64,
    message_id: i64,
    comment: Option<String>,
}

impl<'a> DeclineSuggestedPostBuilder<'a> {
    /// Sets the `comment` parameter.
    pub fn comment(mut self, val: impl Into<String>) -> Self {
        self.comment = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot
            .decline_suggested_post_raw(self.chat_id, self.message_id, self.comment.as_deref())
            .await
    }
}

impl_into_future!(DeclineSuggestedPostBuilder, bool);

// =========================================================================
// user_profile.rs builders
// =========================================================================

// -------------------------------------------------------------------------
// GetUserProfilePhotosBuilder
// -------------------------------------------------------------------------

/// Builder for the [`getUserProfilePhotos`] API method.
pub struct GetUserProfilePhotosBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    offset: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetUserProfilePhotosBuilder<'a> {
    /// Sets the `offset` parameter.
    pub fn offset(mut self, val: i64) -> Self {
        self.offset = Some(val);
        self
    }
    /// Sets the `limit` parameter.
    pub fn limit(mut self, val: i64) -> Self {
        self.limit = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<user_profile_photos::UserProfilePhotos> {
        self.bot
            .get_user_profile_photos_raw(self.user_id, self.offset, self.limit)
            .await
    }
}

impl_into_future!(
    GetUserProfilePhotosBuilder,
    user_profile_photos::UserProfilePhotos
);

// -------------------------------------------------------------------------
// GetUserProfileAudiosBuilder
// -------------------------------------------------------------------------

/// Builder for the [`getUserProfileAudios`] API method.
pub struct GetUserProfileAudiosBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    offset: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetUserProfileAudiosBuilder<'a> {
    /// Sets the `offset` parameter.
    pub fn offset(mut self, val: i64) -> Self {
        self.offset = Some(val);
        self
    }
    /// Sets the `limit` parameter.
    pub fn limit(mut self, val: i64) -> Self {
        self.limit = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<user_profile_audios::UserProfileAudios> {
        self.bot
            .get_user_profile_audios_raw(self.user_id, self.offset, self.limit)
            .await
    }
}

impl_into_future!(
    GetUserProfileAudiosBuilder,
    user_profile_audios::UserProfileAudios
);

// -------------------------------------------------------------------------
// SetUserEmojiStatusBuilder
// -------------------------------------------------------------------------

/// Builder for the [`setUserEmojiStatus`] API method.
pub struct SetUserEmojiStatusBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    emoji_status_custom_emoji_id: Option<String>,
    emoji_status_expiration_date: Option<i64>,
}

impl<'a> SetUserEmojiStatusBuilder<'a> {
    /// Sets the `emoji_status_custom_emoji_id` parameter.
    pub fn emoji_status_custom_emoji_id(mut self, val: impl Into<String>) -> Self {
        self.emoji_status_custom_emoji_id = Some(val.into());
        self
    }
    /// Sets the `emoji_status_expiration_date` parameter.
    pub fn emoji_status_expiration_date(mut self, val: i64) -> Self {
        self.emoji_status_expiration_date = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot
            .set_user_emoji_status_raw(
                self.user_id,
                self.emoji_status_custom_emoji_id.as_deref(),
                self.emoji_status_expiration_date,
            )
            .await
    }
}

impl_into_future!(SetUserEmojiStatusBuilder, bool);

// -------------------------------------------------------------------------
// SetMyProfilePhotoBuilder
// -------------------------------------------------------------------------

/// Builder for the [`setMyProfilePhoto`] API method.
pub struct SetMyProfilePhotoBuilder<'a> {
    bot: &'a Bot,
    photo: serde_json::Value,
}

impl<'a> SetMyProfilePhotoBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot.set_my_profile_photo_raw(self.photo).await
    }
}

impl_into_future!(SetMyProfilePhotoBuilder, bool);

// -------------------------------------------------------------------------
// RemoveMyProfilePhotoBuilder
// -------------------------------------------------------------------------

/// Builder for the [`removeMyProfilePhoto`] API method.
#[derive(Serialize)]
pub struct RemoveMyProfilePhotoBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
}

impl<'a> RemoveMyProfilePhotoBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot.remove_my_profile_photo_raw().await
    }
}

impl_into_future!(RemoveMyProfilePhotoBuilder, bool);

// =========================================================================
// verification.rs builders
// =========================================================================

// -------------------------------------------------------------------------
// VerifyChatBuilder
// -------------------------------------------------------------------------

/// Builder for the [`verifyChat`] API method.
pub struct VerifyChatBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    custom_description: Option<String>,
}

impl<'a> VerifyChatBuilder<'a> {
    /// Sets the `custom_description` parameter.
    pub fn custom_description(mut self, val: impl Into<String>) -> Self {
        self.custom_description = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot
            .verify_chat_raw(self.chat_id, self.custom_description.as_deref())
            .await
    }
}

impl_into_future!(VerifyChatBuilder, bool);

// -------------------------------------------------------------------------
// VerifyUserBuilder
// -------------------------------------------------------------------------

/// Builder for the [`verifyUser`] API method.
pub struct VerifyUserBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    custom_description: Option<String>,
}

impl<'a> VerifyUserBuilder<'a> {
    /// Sets the `custom_description` parameter.
    pub fn custom_description(mut self, val: impl Into<String>) -> Self {
        self.custom_description = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot
            .verify_user_raw(self.user_id, self.custom_description.as_deref())
            .await
    }
}

impl_into_future!(VerifyUserBuilder, bool);

// -------------------------------------------------------------------------
// RemoveChatVerificationBuilder
// -------------------------------------------------------------------------

/// Builder for the [`removeChatVerification`] API method.
pub struct RemoveChatVerificationBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
}

impl<'a> RemoveChatVerificationBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot.remove_chat_verification_raw(self.chat_id).await
    }
}

impl_into_future!(RemoveChatVerificationBuilder, bool);

// -------------------------------------------------------------------------
// RemoveUserVerificationBuilder
// -------------------------------------------------------------------------

/// Builder for the [`removeUserVerification`] API method.
pub struct RemoveUserVerificationBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
}

impl<'a> RemoveUserVerificationBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        self.bot.remove_user_verification_raw(self.user_id).await
    }
}

impl_into_future!(RemoveUserVerificationBuilder, bool);

// =========================================================================
// Factory methods on Bot
// =========================================================================

impl Bot {
    // -- Editing methods --------------------------------------------------

    /// Build an `editMessageLiveLocation` request.
    pub fn edit_message_live_location(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> EditMessageLiveLocationBuilder<'_> {
        EditMessageLiveLocationBuilder {
            bot: self,
            latitude,
            longitude,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            live_period: None,
            business_connection_id: None,
        }
    }

    /// Build a `stopMessageLiveLocation` request.
    pub fn stop_message_live_location(&self) -> StopMessageLiveLocationBuilder<'_> {
        StopMessageLiveLocationBuilder {
            bot: self,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
            business_connection_id: None,
        }
    }

    /// Build an `editMessageChecklist` request.
    pub fn edit_message_checklist(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        message_id: i64,
        checklist: input_checklist::InputChecklist,
    ) -> EditMessageChecklistBuilder<'_> {
        EditMessageChecklistBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            chat_id,
            message_id,
            checklist,
            reply_markup: None,
        }
    }

    /// Build a `stopPoll` request.
    pub fn stop_poll(&self, chat_id: impl Into<ChatId>, message_id: i64) -> StopPollBuilder<'_> {
        StopPollBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
            business_connection_id: None,
        }
    }

    // -- Games methods ----------------------------------------------------

    /// Build a `sendGame` request.
    pub fn send_game(
        &self,
        chat_id: i64,
        game_short_name: impl Into<String>,
    ) -> SendGameBuilder<'_> {
        SendGameBuilder {
            bot: self,
            chat_id,
            game_short_name: game_short_name.into(),
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            message_thread_id: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
        }
    }

    /// Build a `setGameScore` request.
    pub fn set_game_score(&self, user_id: i64, score: i64) -> SetGameScoreBuilder<'_> {
        SetGameScoreBuilder {
            bot: self,
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }

    /// Build a `getGameHighScores` request.
    pub fn get_game_high_scores(&self, user_id: i64) -> GetGameHighScoresBuilder<'_> {
        GetGameHighScoresBuilder {
            bot: self,
            user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }

    // -- Inline methods ---------------------------------------------------

    /// Build a `savePreparedInlineMessage` request.
    pub fn save_prepared_inline_message(
        &self,
        user_id: i64,
        result: serde_json::Value,
    ) -> SavePreparedInlineMessageBuilder<'_> {
        SavePreparedInlineMessageBuilder {
            bot: self,
            user_id,
            result,
            allow_user_chats: None,
            allow_bot_chats: None,
            allow_group_chats: None,
            allow_channel_chats: None,
        }
    }

    /// Build an `answerWebAppQuery` request.
    pub fn answer_web_app_query(
        &self,
        web_app_query_id: impl Into<String>,
        result: serde_json::Value,
    ) -> AnswerWebAppQueryBuilder<'_> {
        AnswerWebAppQueryBuilder {
            bot: self,
            web_app_query_id: web_app_query_id.into(),
            result,
        }
    }

    // -- Media group / paid media -----------------------------------------

    /// Build a `sendMediaGroup` request.
    pub fn send_media_group(
        &self,
        chat_id: impl Into<ChatId>,
        media: Vec<serde_json::Value>,
    ) -> SendMediaGroupBuilder<'_> {
        SendMediaGroupBuilder {
            bot: self,
            chat_id: chat_id.into(),
            media,
            disable_notification: None,
            protect_content: None,
            message_thread_id: None,
            reply_parameters: None,
            business_connection_id: None,
            message_effect_id: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
        }
    }

    /// Build a `sendPaidMedia` request.
    pub fn send_paid_media(
        &self,
        chat_id: impl Into<ChatId>,
        star_count: i64,
        media: Vec<serde_json::Value>,
    ) -> SendPaidMediaBuilder<'_> {
        SendPaidMediaBuilder {
            bot: self,
            chat_id: chat_id.into(),
            star_count,
            media,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
            business_connection_id: None,
            payload: None,
            allow_paid_broadcast: None,
            direct_messages_topic_id: None,
            suggested_post_parameters: None,
            message_thread_id: None,
        }
    }

    // -- Core Bot methods -------------------------------------------------

    /// Build a `getMe` request.
    pub fn get_me(&self) -> GetMeBuilder<'_> {
        GetMeBuilder { bot: self }
    }

    /// Build a `logOut` request.
    pub fn log_out(&self) -> LogOutBuilder<'_> {
        LogOutBuilder { bot: self }
    }

    /// Build a `close` request.
    pub fn close(&self) -> CloseBuilder<'_> {
        CloseBuilder { bot: self }
    }

    /// Build a `getUpdates` request.
    pub fn get_updates(&self) -> GetUpdatesBuilder<'_> {
        GetUpdatesBuilder {
            bot: self,
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }

    /// Build a `getWebhookInfo` request.
    pub fn get_webhook_info(&self) -> GetWebhookInfoBuilder<'_> {
        GetWebhookInfoBuilder { bot: self }
    }

    /// Build a `downloadFile` request.
    pub fn download_file(&self, file_path: impl Into<String>) -> DownloadFileBuilder<'_> {
        DownloadFileBuilder {
            bot: self,
            file_path: file_path.into(),
        }
    }

    // -- Other content (send_checklist) -----------------------------------

    /// Build a `sendChecklist` request.
    pub fn send_checklist(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        checklist: input_checklist::InputChecklist,
    ) -> SendChecklistBuilder<'_> {
        SendChecklistBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            chat_id,
            checklist,
            disable_notification: None,
            protect_content: None,
            message_effect_id: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    // -- Passport ---------------------------------------------------------

    /// Build a `setPassportDataErrors` request.
    pub fn set_passport_data_errors(
        &self,
        user_id: i64,
        errors: Vec<serde_json::Value>,
    ) -> SetPassportDataErrorsBuilder<'_> {
        SetPassportDataErrorsBuilder {
            bot: self,
            user_id,
            errors,
        }
    }

    // -- Reactions ---------------------------------------------------------

    /// Build a `setMessageReaction` request.
    pub fn set_message_reaction(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> SetMessageReactionBuilder<'_> {
        SetMessageReactionBuilder {
            bot: self,
            chat_id: chat_id.into(),
            message_id,
            reaction: None,
            is_big: None,
        }
    }

    /// Build a `getUserChatBoosts` request.
    pub fn get_user_chat_boosts(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> GetUserChatBoostsBuilder<'_> {
        GetUserChatBoostsBuilder {
            bot: self,
            chat_id: chat_id.into(),
            user_id,
        }
    }

    // -- Stories ----------------------------------------------------------

    /// Build a `postStory` request.
    pub fn post_story(
        &self,
        business_connection_id: impl Into<String>,
        content: serde_json::Value,
        active_period: i64,
    ) -> PostStoryBuilder<'_> {
        PostStoryBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            content,
            active_period,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            areas: None,
            post_to_chat_page: None,
            protect_content: None,
        }
    }

    /// Build an `editStory` request.
    pub fn edit_story(
        &self,
        business_connection_id: impl Into<String>,
        story_id: i64,
        content: serde_json::Value,
    ) -> EditStoryBuilder<'_> {
        EditStoryBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            story_id,
            content,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            areas: None,
        }
    }

    /// Build a `deleteStory` request.
    pub fn delete_story(
        &self,
        business_connection_id: impl Into<String>,
        story_id: i64,
    ) -> DeleteStoryBuilder<'_> {
        DeleteStoryBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            story_id,
        }
    }

    /// Build a `repostStory` request.
    pub fn repost_story(
        &self,
        business_connection_id: impl Into<String>,
        from_chat_id: i64,
        from_story_id: i64,
        active_period: i64,
    ) -> RepostStoryBuilder<'_> {
        RepostStoryBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            from_chat_id,
            from_story_id,
            active_period,
            post_to_chat_page: None,
            protect_content: None,
        }
    }

    // -- Suggested posts --------------------------------------------------

    /// Build an `approveSuggestedPost` request.
    pub fn approve_suggested_post(
        &self,
        chat_id: i64,
        message_id: i64,
    ) -> ApproveSuggestedPostBuilder<'_> {
        ApproveSuggestedPostBuilder {
            bot: self,
            chat_id,
            message_id,
            send_date: None,
        }
    }

    /// Build a `declineSuggestedPost` request.
    pub fn decline_suggested_post(
        &self,
        chat_id: i64,
        message_id: i64,
    ) -> DeclineSuggestedPostBuilder<'_> {
        DeclineSuggestedPostBuilder {
            bot: self,
            chat_id,
            message_id,
            comment: None,
        }
    }

    // -- User profile -----------------------------------------------------

    /// Build a `getUserProfilePhotos` request.
    pub fn get_user_profile_photos(&self, user_id: i64) -> GetUserProfilePhotosBuilder<'_> {
        GetUserProfilePhotosBuilder {
            bot: self,
            user_id,
            offset: None,
            limit: None,
        }
    }

    /// Build a `getUserProfileAudios` request.
    pub fn get_user_profile_audios(&self, user_id: i64) -> GetUserProfileAudiosBuilder<'_> {
        GetUserProfileAudiosBuilder {
            bot: self,
            user_id,
            offset: None,
            limit: None,
        }
    }

    /// Build a `setUserEmojiStatus` request.
    pub fn set_user_emoji_status(&self, user_id: i64) -> SetUserEmojiStatusBuilder<'_> {
        SetUserEmojiStatusBuilder {
            bot: self,
            user_id,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
        }
    }

    /// Build a `setMyProfilePhoto` request.
    pub fn set_my_profile_photo(&self, photo: serde_json::Value) -> SetMyProfilePhotoBuilder<'_> {
        SetMyProfilePhotoBuilder { bot: self, photo }
    }

    /// Build a `removeMyProfilePhoto` request.
    pub fn remove_my_profile_photo(&self) -> RemoveMyProfilePhotoBuilder<'_> {
        RemoveMyProfilePhotoBuilder { bot: self }
    }

    // -- Verification -----------------------------------------------------

    /// Build a `verifyChat` request.
    pub fn verify_chat(&self, chat_id: impl Into<ChatId>) -> VerifyChatBuilder<'_> {
        VerifyChatBuilder {
            bot: self,
            chat_id: chat_id.into(),
            custom_description: None,
        }
    }

    /// Build a `verifyUser` request.
    pub fn verify_user(&self, user_id: i64) -> VerifyUserBuilder<'_> {
        VerifyUserBuilder {
            bot: self,
            user_id,
            custom_description: None,
        }
    }

    /// Build a `removeChatVerification` request.
    pub fn remove_chat_verification(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> RemoveChatVerificationBuilder<'_> {
        RemoveChatVerificationBuilder {
            bot: self,
            chat_id: chat_id.into(),
        }
    }

    /// Build a `removeUserVerification` request.
    pub fn remove_user_verification(&self, user_id: i64) -> RemoveUserVerificationBuilder<'_> {
        RemoveUserVerificationBuilder { bot: self, user_id }
    }
}
