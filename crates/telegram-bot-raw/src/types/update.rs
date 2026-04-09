//! Telegram [`Update`] type — ported from `python-telegram-bot/src/telegram/_update.py`.
//!
//! Only data fields are included. No Bot reference, no API shortcuts.
//! The three computed properties from Python (`effective_user`, `effective_chat`,
//! `effective_message`) are provided as plain `fn` methods returning `Option` references.

use std::fmt;

use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number
    /// and increase sequentially.
    pub update_id: i64,

    /// The payload carried by the update.
    #[serde(flatten, default)]
    pub kind: UpdateKind,
}

/// The payload carried by an incoming [`Update`].
///
/// Telegram sends update kinds as a single top-level key such as `message`,
/// `callback_query`, or `poll`. This enum preserves that wire format via a
/// custom serde implementation.
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateKind {
    /// New incoming message of any kind — text, photo, sticker, etc.
    Message(Message),
    /// New version of a message that is known to the bot and was edited.
    EditedMessage(Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    ChannelPost(Message),
    /// New version of a channel post that is known to the bot and was edited.
    EditedChannelPost(Message),
    /// The bot was connected to or disconnected from a business account.
    BusinessConnection(BusinessConnection),
    /// New non-service message from a connected business account.
    BusinessMessage(Message),
    /// New version of a message from a connected business account.
    EditedBusinessMessage(Message),
    /// Messages were deleted from a connected business account.
    DeletedBusinessMessages(BusinessMessagesDeleted),
    /// New incoming inline query.
    InlineQuery(InlineQuery),
    /// The result of an inline query that was chosen by a user.
    ChosenInlineResult(ChosenInlineResult),
    /// New incoming callback query.
    CallbackQuery(CallbackQuery),
    /// New incoming shipping query.
    ShippingQuery(ShippingQuery),
    /// New incoming pre-checkout query.
    PreCheckoutQuery(PreCheckoutQuery),
    /// A user purchased paid media.
    PurchasedPaidMedia(PaidMediaPurchased),
    /// New poll state.
    Poll(Poll),
    /// A user changed their answer in a non-anonymous poll.
    PollAnswer(PollAnswer),
    /// The bot's chat member status was updated in a chat.
    MyChatMember(ChatMemberUpdated),
    /// A chat member's status was updated in a chat.
    ChatMember(ChatMemberUpdated),
    /// A request to join the chat has been sent.
    ChatJoinRequest(ChatJoinRequest),
    /// A chat boost was added or changed.
    ChatBoost(ChatBoostUpdated),
    /// A boost was removed from a chat.
    RemovedChatBoost(ChatBoostRemoved),
    /// A reaction to a message was changed by a user.
    MessageReaction(MessageReactionUpdated),
    /// Reactions to a message with anonymous reactions were changed.
    MessageReactionCount(MessageReactionCountUpdated),
    /// A managed bot was created or updated.
    ManagedBot(ManagedBotUpdated),
    /// Unknown or unsupported update types, or an update with no payload fields.
    Unknown,
}

impl Default for UpdateKind {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Serialize for UpdateKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Message(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 0, "message", value)
            }
            Self::EditedMessage(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 1, "edited_message", value)
            }
            Self::ChannelPost(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 2, "channel_post", value)
            }
            Self::EditedChannelPost(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 3, "edited_channel_post", value)
            }
            Self::BusinessConnection(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 4, "business_connection", value)
            }
            Self::BusinessMessage(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 5, "business_message", value)
            }
            Self::EditedBusinessMessage(value) => serializer.serialize_newtype_variant(
                "UpdateKind",
                6,
                "edited_business_message",
                value,
            ),
            Self::DeletedBusinessMessages(value) => serializer.serialize_newtype_variant(
                "UpdateKind",
                7,
                "deleted_business_messages",
                value,
            ),
            Self::InlineQuery(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 8, "inline_query", value)
            }
            Self::ChosenInlineResult(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 9, "chosen_inline_result", value)
            }
            Self::CallbackQuery(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 10, "callback_query", value)
            }
            Self::ShippingQuery(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 11, "shipping_query", value)
            }
            Self::PreCheckoutQuery(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 12, "pre_checkout_query", value)
            }
            Self::PurchasedPaidMedia(value) => serializer.serialize_newtype_variant(
                "UpdateKind",
                13,
                "purchased_paid_media",
                value,
            ),
            Self::Poll(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 14, "poll", value)
            }
            Self::PollAnswer(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 15, "poll_answer", value)
            }
            Self::MyChatMember(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 16, "my_chat_member", value)
            }
            Self::ChatMember(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 17, "chat_member", value)
            }
            Self::ChatJoinRequest(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 18, "chat_join_request", value)
            }
            Self::ChatBoost(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 19, "chat_boost", value)
            }
            Self::RemovedChatBoost(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 20, "removed_chat_boost", value)
            }
            Self::MessageReaction(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 21, "message_reaction", value)
            }
            Self::MessageReactionCount(value) => serializer.serialize_newtype_variant(
                "UpdateKind",
                22,
                "message_reaction_count",
                value,
            ),
            Self::ManagedBot(value) => {
                serializer.serialize_newtype_variant("UpdateKind", 23, "managed_bot", value)
            }
            Self::Unknown => {
                let map = serializer.serialize_map(Some(0))?;
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for UpdateKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Grow stack for deeply nested Message types (reply_to_message chains).
        // Matches teloxide's approach to prevent stack overflow.
        stacker::maybe_grow(256 * 1024, 1024 * 1024, || {
            deserializer.deserialize_map(UpdateKindVisitor)
        })
    }
}

struct UpdateKindVisitor;

impl<'de> Visitor<'de> for UpdateKindVisitor {
    type Value = UpdateKind;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Telegram update payload")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut kind = None;

        macro_rules! parse_kind {
            ($variant:ident, $ty:ty) => {{
                if kind.is_none() {
                    kind = Some(UpdateKind::$variant(map.next_value::<$ty>()?));
                } else {
                    map.next_value::<IgnoredAny>()?;
                }
            }};
        }

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "message" => parse_kind!(Message, Message),
                "edited_message" => parse_kind!(EditedMessage, Message),
                "channel_post" => parse_kind!(ChannelPost, Message),
                "edited_channel_post" => parse_kind!(EditedChannelPost, Message),
                "business_connection" => {
                    parse_kind!(BusinessConnection, BusinessConnection)
                }
                "business_message" => parse_kind!(BusinessMessage, Message),
                "edited_business_message" => {
                    parse_kind!(EditedBusinessMessage, Message)
                }
                "deleted_business_messages" => {
                    parse_kind!(DeletedBusinessMessages, BusinessMessagesDeleted)
                }
                "inline_query" => parse_kind!(InlineQuery, InlineQuery),
                "chosen_inline_result" => {
                    parse_kind!(ChosenInlineResult, ChosenInlineResult)
                }
                "callback_query" => parse_kind!(CallbackQuery, CallbackQuery),
                "shipping_query" => parse_kind!(ShippingQuery, ShippingQuery),
                "pre_checkout_query" => {
                    parse_kind!(PreCheckoutQuery, PreCheckoutQuery)
                }
                "purchased_paid_media" => {
                    parse_kind!(PurchasedPaidMedia, PaidMediaPurchased)
                }
                "poll" => parse_kind!(Poll, Poll),
                "poll_answer" => parse_kind!(PollAnswer, PollAnswer),
                "my_chat_member" => parse_kind!(MyChatMember, ChatMemberUpdated),
                "chat_member" => parse_kind!(ChatMember, ChatMemberUpdated),
                "chat_join_request" => parse_kind!(ChatJoinRequest, ChatJoinRequest),
                "chat_boost" => parse_kind!(ChatBoost, ChatBoostUpdated),
                "removed_chat_boost" => {
                    parse_kind!(RemovedChatBoost, ChatBoostRemoved)
                }
                "message_reaction" => {
                    parse_kind!(MessageReaction, MessageReactionUpdated)
                }
                "message_reaction_count" => {
                    parse_kind!(MessageReactionCount, MessageReactionCountUpdated)
                }
                "managed_bot" => parse_kind!(ManagedBot, ManagedBotUpdated),
                "update_id" => {
                    map.next_value::<IgnoredAny>()?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }

        Ok(kind.unwrap_or(UpdateKind::Unknown))
    }
}

macro_rules! update_kind_accessors {
    ($(($name:ident, $variant:ident, $ty:ty)),* $(,)?) => {
        $(
            #[must_use]
            pub fn $name(&self) -> Option<&$ty> {
                match &self.kind {
                    UpdateKind::$variant(value) => Some(value),
                    _ => None,
                }
            }
        )*
    };
}

impl Update {
    /// Returns the message payload for message-like update kinds.
    #[must_use]
    pub fn message(&self) -> Option<&Message> {
        match &self.kind {
            UpdateKind::Message(message)
            | UpdateKind::EditedMessage(message)
            | UpdateKind::ChannelPost(message)
            | UpdateKind::EditedChannelPost(message)
            | UpdateKind::BusinessMessage(message)
            | UpdateKind::EditedBusinessMessage(message) => Some(message),
            _ => None,
        }
    }

    update_kind_accessors!(
        (edited_message, EditedMessage, Message),
        (channel_post, ChannelPost, Message),
        (edited_channel_post, EditedChannelPost, Message),
        (business_connection, BusinessConnection, BusinessConnection),
        (business_message, BusinessMessage, Message),
        (edited_business_message, EditedBusinessMessage, Message),
        (
            deleted_business_messages,
            DeletedBusinessMessages,
            BusinessMessagesDeleted
        ),
        (inline_query, InlineQuery, InlineQuery),
        (chosen_inline_result, ChosenInlineResult, ChosenInlineResult),
        (callback_query, CallbackQuery, CallbackQuery),
        (shipping_query, ShippingQuery, ShippingQuery),
        (pre_checkout_query, PreCheckoutQuery, PreCheckoutQuery),
        (purchased_paid_media, PurchasedPaidMedia, PaidMediaPurchased),
        (poll, Poll, Poll),
        (poll_answer, PollAnswer, PollAnswer),
        (my_chat_member, MyChatMember, ChatMemberUpdated),
        (chat_member, ChatMember, ChatMemberUpdated),
        (chat_join_request, ChatJoinRequest, ChatJoinRequest),
        (chat_boost, ChatBoost, ChatBoostUpdated),
        (removed_chat_boost, RemovedChatBoost, ChatBoostRemoved),
        (message_reaction, MessageReaction, MessageReactionUpdated),
        (
            message_reaction_count,
            MessageReactionCount,
            MessageReactionCountUpdated
        ),
        (managed_bot, ManagedBot, ManagedBotUpdated),
    );

    /// Returns a reference to the [`User`] that sent this update, regardless of update type.
    ///
    /// Returns `None` for updates that have no associated user (channel posts, polls, chat
    /// boosts, removed chat boosts, message reaction count, deleted business messages).
    ///
    /// Mirrors Python's `Update.effective_user` property.
    #[must_use]
    pub fn effective_user(&self) -> Option<&User> {
        match &self.kind {
            UpdateKind::Message(msg)
            | UpdateKind::EditedMessage(msg)
            | UpdateKind::BusinessMessage(msg)
            | UpdateKind::EditedBusinessMessage(msg) => msg.from_user.as_ref(),
            UpdateKind::CallbackQuery(cbq) => Some(&cbq.from_user),
            UpdateKind::ChosenInlineResult(cir) => Some(&cir.from_user),
            UpdateKind::InlineQuery(iq) => Some(&iq.from_user),
            UpdateKind::ShippingQuery(sq) => Some(&sq.from_user),
            UpdateKind::PreCheckoutQuery(pq) => Some(&pq.from_user),
            UpdateKind::PollAnswer(pa) => pa.user.as_ref(),
            UpdateKind::MyChatMember(cmu) | UpdateKind::ChatMember(cmu) => Some(&cmu.from_user),
            UpdateKind::ChatJoinRequest(cjr) => Some(&cjr.from_user),
            UpdateKind::MessageReaction(mru) => mru.user.as_ref(),
            UpdateKind::BusinessConnection(bc) => Some(&bc.user),
            UpdateKind::PurchasedPaidMedia(ppm) => Some(&ppm.from_user),
            UpdateKind::ManagedBot(mbu) => Some(&mbu.user),
            UpdateKind::ChannelPost(_)
            | UpdateKind::EditedChannelPost(_)
            | UpdateKind::DeletedBusinessMessages(_)
            | UpdateKind::Poll(_)
            | UpdateKind::ChatBoost(_)
            | UpdateKind::RemovedChatBoost(_)
            | UpdateKind::MessageReactionCount(_)
            | UpdateKind::Unknown => None,
        }
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
        match &self.kind {
            UpdateKind::Message(msg)
            | UpdateKind::EditedMessage(msg)
            | UpdateKind::ChannelPost(msg)
            | UpdateKind::EditedChannelPost(msg)
            | UpdateKind::BusinessMessage(msg)
            | UpdateKind::EditedBusinessMessage(msg) => Some(&msg.chat),
            UpdateKind::CallbackQuery(cbq) => cbq.message.as_deref().map(|message| message.chat()),
            UpdateKind::MyChatMember(cmu) | UpdateKind::ChatMember(cmu) => Some(&cmu.chat),
            UpdateKind::ChatJoinRequest(cjr) => Some(&cjr.chat),
            UpdateKind::ChatBoost(cbu) => Some(&cbu.chat),
            UpdateKind::RemovedChatBoost(cbr) => Some(&cbr.chat),
            UpdateKind::MessageReaction(mru) => Some(&mru.chat),
            UpdateKind::MessageReactionCount(mrcu) => Some(&mrcu.chat),
            UpdateKind::DeletedBusinessMessages(dbm) => Some(&dbm.chat),
            UpdateKind::BusinessConnection(_)
            | UpdateKind::InlineQuery(_)
            | UpdateKind::ChosenInlineResult(_)
            | UpdateKind::ShippingQuery(_)
            | UpdateKind::PreCheckoutQuery(_)
            | UpdateKind::PurchasedPaidMedia(_)
            | UpdateKind::Poll(_)
            | UpdateKind::PollAnswer(_)
            | UpdateKind::ManagedBot(_)
            | UpdateKind::Unknown => None,
        }
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
        match &self.kind {
            UpdateKind::Message(msg)
            | UpdateKind::EditedMessage(msg)
            | UpdateKind::ChannelPost(msg)
            | UpdateKind::EditedChannelPost(msg)
            | UpdateKind::BusinessMessage(msg)
            | UpdateKind::EditedBusinessMessage(msg) => Some(msg),
            UpdateKind::CallbackQuery(cbq) => cbq
                .message
                .as_ref()
                .and_then(|message| message.as_message()),
            UpdateKind::BusinessConnection(_)
            | UpdateKind::DeletedBusinessMessages(_)
            | UpdateKind::InlineQuery(_)
            | UpdateKind::ChosenInlineResult(_)
            | UpdateKind::ShippingQuery(_)
            | UpdateKind::PreCheckoutQuery(_)
            | UpdateKind::PurchasedPaidMedia(_)
            | UpdateKind::Poll(_)
            | UpdateKind::PollAnswer(_)
            | UpdateKind::MyChatMember(_)
            | UpdateKind::ChatMember(_)
            | UpdateKind::ChatJoinRequest(_)
            | UpdateKind::ChatBoost(_)
            | UpdateKind::RemovedChatBoost(_)
            | UpdateKind::MessageReaction(_)
            | UpdateKind::MessageReactionCount(_)
            | UpdateKind::ManagedBot(_)
            | UpdateKind::Unknown => None,
        }
    }
}
