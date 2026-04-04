//! Telegram [`Update`] type — ported from `python-telegram-bot/src/telegram/_update.py`.
//!
//! Only data fields are included. No Bot reference, no API shortcuts.
//! The three computed properties from Python (`effective_user`, `effective_chat`,
//! `effective_message`) are provided as plain `fn` methods returning `Option` references.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::business::{BusinessConnection, BusinessMessagesDeleted};
use super::callback_query::CallbackQuery;
use super::chat::Chat;
use super::chat_boost::{ChatBoostRemoved, ChatBoostUpdated};
use super::chat_join_request::ChatJoinRequest;
use super::chat_member_updated::ChatMemberUpdated;
use super::chosen_inline_result::ChosenInlineResult;
use super::inline::inline_query::InlineQuery;
use super::managed_bot::ManagedBotUpdated;
use super::message::Message;
use super::message_reaction_updated::{MessageReactionCountUpdated, MessageReactionUpdated};
use super::paid_media::PaidMediaPurchased;
use super::payment::pre_checkout_query::PreCheckoutQuery;
use super::payment::shipping_query::ShippingQuery;
use super::poll::{Poll, PollAnswer};
use super::user::User;

/// This object represents an incoming update.
///
/// Corresponds to the Bot API [`Update`](https://core.telegram.org/bots/api#update) object.
///
/// At most one of the optional fields can be present in any given update.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Update {
    // ── Required ──────────────────────────────────────────────────────────────

    /// The update's unique identifier. Update identifiers start from a certain positive number
    /// and increase sequentially.
    pub update_id: i64,

    // ── Message variants ──────────────────────────────────────────────────────

    /// New incoming message of any kind — text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,

    /// New version of a message that is known to the bot and was edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Message>,

    /// New incoming channel post of any kind — text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Message>,

    /// New version of a channel post that is known to the bot and was edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Message>,

    // ── Business messages ─────────────────────────────────────────────────────

    /// The bot was connected to or disconnected from a business account, or a user edited an
    /// existing connection with the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection: Option<BusinessConnection>,

    /// New non-service message from a connected business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_message: Option<Message>,

    /// New version of a message from a connected business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_business_message: Option<Message>,

    /// Messages were deleted from a connected business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_business_messages: Option<BusinessMessagesDeleted>,

    // ── Queries & results ─────────────────────────────────────────────────────

    /// New incoming inline query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,

    /// The result of an inline query that was chosen by a user and sent to their chat partner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,

    /// New incoming callback query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,

    /// New incoming shipping query; only for invoices with flexible price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,

    /// New incoming pre-checkout query; contains full information about checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,

    /// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_paid_media: Option<PaidMediaPurchased>,

    // ── Polls ─────────────────────────────────────────────────────────────────

    /// New poll state. Bots receive only updates about stopped polls and polls which are
    /// sent by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    /// A user changed their answer in a non-anonymous poll.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,

    // ── Chat member events ────────────────────────────────────────────────────

    /// The bot's chat member status was updated in a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,

    /// A chat member's status was updated in a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,

    /// A request to join the chat has been sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,

    // ── Chat boost events ─────────────────────────────────────────────────────

    /// A chat boost was added or changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_boost: Option<ChatBoostUpdated>,

    /// A boost was removed from a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_chat_boost: Option<ChatBoostRemoved>,

    // ── Reaction events ───────────────────────────────────────────────────────

    /// A reaction to a message was changed by a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction: Option<MessageReactionUpdated>,

    /// Reactions to a message with anonymous reactions were changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction_count: Option<MessageReactionCountUpdated>,

    // ── Managed bots ──────────────────────────────────────────────────────────

    /// A new bot was created to be managed by the bot, or the token of a managed bot was changed.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_bot: Option<ManagedBotUpdated>,

    // ── Catch-all ─────────────────────────────────────────────────────────────

    /// Catch-all for any extra fields returned by the Bot API not yet modelled here.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Update {
    /// Returns a reference to the [`User`] that sent this update, regardless of update type.
    ///
    /// Returns `None` for updates that have no associated user (channel posts, polls, chat
    /// boosts, removed chat boosts, message reaction count, deleted business messages).
    ///
    /// Mirrors Python's `Update.effective_user` property.
    #[must_use]
    pub fn effective_user(&self) -> Option<&User> {
        if let Some(msg) = &self.message {
            if let Some(user) = &msg.from_user {
                return Some(user);
            }
        }
        if let Some(msg) = &self.edited_message {
            if let Some(user) = &msg.from_user {
                return Some(user);
            }
        }
        if let Some(cbq) = &self.callback_query {
            return Some(&cbq.from_user);
        }
        if let Some(cir) = &self.chosen_inline_result {
            return Some(&cir.from_user);
        }
        if let Some(msg) = &self.business_message {
            if let Some(user) = &msg.from_user {
                return Some(user);
            }
        }
        if let Some(msg) = &self.edited_business_message {
            if let Some(user) = &msg.from_user {
                return Some(user);
            }
        }
        if let Some(iq) = &self.inline_query {
            return Some(&iq.from_user);
        }
        if let Some(sq) = &self.shipping_query {
            return Some(&sq.from_user);
        }
        if let Some(pq) = &self.pre_checkout_query {
            return Some(&pq.from_user);
        }
        if let Some(pa) = &self.poll_answer {
            if let Some(user) = &pa.user {
                return Some(user);
            }
        }
        if let Some(cmu) = &self.my_chat_member {
            return Some(&cmu.from_user);
        }
        if let Some(cmu) = &self.chat_member {
            return Some(&cmu.from_user);
        }
        if let Some(cjr) = &self.chat_join_request {
            return Some(&cjr.from_user);
        }
        if let Some(mru) = &self.message_reaction {
            if let Some(user) = &mru.user {
                return Some(user);
            }
        }
        if let Some(bc) = &self.business_connection {
            return Some(&bc.user);
        }
        if let Some(ppm) = &self.purchased_paid_media {
            return Some(&ppm.from_user);
        }
        if let Some(mbu) = &self.managed_bot {
            return Some(&mbu.user);
        }
        None
    }

    /// Returns a reference to the [`Chat`] in which this update occurred, if any.
    ///
    /// Returns `None` for updates with no associated chat (inline queries, chosen inline
    /// results, shipping/pre-checkout queries, polls, poll answers, business connections,
    /// purchased paid media, managed bot updates).
    ///
    /// Mirrors Python's `Update.effective_chat` property.
    #[must_use]
    pub fn effective_chat(&self) -> Option<&Chat> {
        if let Some(msg) = &self.message {
            return Some(&msg.chat);
        }
        if let Some(msg) = &self.edited_message {
            return Some(&msg.chat);
        }
        if let Some(cbq) = &self.callback_query {
            if let Some(msg) = cbq.message.as_deref() {
                return Some(msg.chat());
            }
        }
        if let Some(msg) = &self.channel_post {
            return Some(&msg.chat);
        }
        if let Some(msg) = &self.edited_channel_post {
            return Some(&msg.chat);
        }
        if let Some(msg) = &self.business_message {
            return Some(&msg.chat);
        }
        if let Some(msg) = &self.edited_business_message {
            return Some(&msg.chat);
        }
        if let Some(cmu) = &self.my_chat_member {
            return Some(&cmu.chat);
        }
        if let Some(cmu) = &self.chat_member {
            return Some(&cmu.chat);
        }
        if let Some(cjr) = &self.chat_join_request {
            return Some(&cjr.chat);
        }
        if let Some(cbu) = &self.chat_boost {
            return Some(&cbu.chat);
        }
        if let Some(cbr) = &self.removed_chat_boost {
            return Some(&cbr.chat);
        }
        if let Some(mru) = &self.message_reaction {
            return Some(&mru.chat);
        }
        if let Some(mrcu) = &self.message_reaction_count {
            return Some(&mrcu.chat);
        }
        if let Some(dbm) = &self.deleted_business_messages {
            return Some(&dbm.chat);
        }
        None
    }

    /// Returns a reference to the [`Message`] carried by this update, if any.
    ///
    /// Considers `message`, `edited_message`, `channel_post`, `edited_channel_post`,
    /// the message inside `callback_query`, `business_message`, and
    /// `edited_business_message`.
    ///
    /// For callback queries where the message is inaccessible (date == 0), this returns `None`
    /// for the callback query branch.
    ///
    /// Mirrors Python's `Update.effective_message` property.
    #[must_use]
    pub fn effective_message(&self) -> Option<&Message> {
        if let Some(msg) = &self.message {
            return Some(msg);
        }
        if let Some(msg) = &self.edited_message {
            return Some(msg);
        }
        if let Some(cbq) = &self.callback_query {
            if let Some(msg) = &cbq.message {
                return msg.as_message();
            }
        }
        if let Some(msg) = &self.channel_post {
            return Some(msg);
        }
        if let Some(msg) = &self.edited_channel_post {
            return Some(msg);
        }
        if let Some(msg) = &self.business_message {
            return Some(msg);
        }
        if let Some(msg) = &self.edited_business_message {
            return Some(msg);
        }
        None
    }
}
