//! Telegram [`Message`] type — ported from `python-telegram-bot/src/telegram/_message.py`.
//!
//! All 80+ optional fields from the Python source are represented.
//! Only data fields are included. No Bot reference, no API shortcuts.
//!
//! ## Required fields
//! - `message_id`
//! - `date`
//! - `chat`
//!
//! Everything else is `Option<T>`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::chat::Chat;
use super::chat_background::ChatBackground;
use super::chat_boost::ChatBoostAdded;
use super::chat_owner::{ChatOwnerChanged, ChatOwnerLeft};
use super::checklists::{Checklist, ChecklistTasksAdded, ChecklistTasksDone};
use super::dice::Dice;
use super::direct_message_price_changed::DirectMessagePriceChanged;
use super::direct_messages_topic::DirectMessagesTopic;
use super::files::animation::Animation;
use super::files::audio::Audio;
use super::files::contact::Contact;
use super::files::document::Document;
use super::files::location::Location;
use super::files::photo_size::PhotoSize;
use super::files::sticker::Sticker;
use super::files::venue::Venue;
use super::files::video::Video;
use super::files::video_note::VideoNote;
use super::files::voice::Voice;
use super::forum_topic::{
    ForumTopicClosed, ForumTopicCreated, ForumTopicEdited, ForumTopicReopened,
    GeneralForumTopicHidden, GeneralForumTopicUnhidden,
};
use super::games::game::Game;
use super::gifts::GiftInfo;
use super::giveaway::{Giveaway, GiveawayCompleted, GiveawayCreated, GiveawayWinners};
use super::inline::inline_keyboard_markup::InlineKeyboardMarkup;
use super::link_preview_options::LinkPreviewOptions;
use super::managed_bot::ManagedBotCreated;
use super::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
use super::message_entity::MessageEntity;
use super::message_origin::MessageOrigin;
use super::paid_media::PaidMediaInfo;
use super::paid_message_price_changed::PaidMessagePriceChanged;
use super::passport::passport_data::PassportData;
use super::payment::invoice::Invoice;
use super::payment::refunded_payment::RefundedPayment;
use super::payment::successful_payment::SuccessfulPayment;
use super::poll::{Poll, PollOptionAdded, PollOptionDeleted};
use super::proximity_alert_triggered::ProximityAlertTriggered;
use super::reply::{ExternalReplyInfo, TextQuote};
use super::shared::{ChatShared, UsersShared};
use super::story::Story;
use super::suggested_post::{
    SuggestedPostApprovalFailed, SuggestedPostApproved, SuggestedPostDeclined, SuggestedPostInfo,
    SuggestedPostPaid, SuggestedPostRefunded,
};
use super::unique_gift::UniqueGiftInfo;
use super::user::User;
use super::video_chat::{
    VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted,
};
use super::web_app_data::WebAppData;
use super::write_access_allowed::WriteAccessAllowed;

/// This object represents a message.
///
/// Corresponds to the Bot API [`Message`](https://core.telegram.org/bots/api#message) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    // ── Required fields ───────────────────────────────────────────────────────

    /// Unique message identifier inside this chat.
    pub message_id: i64,

    /// Date the message was sent in Unix time.
    pub date: i64,

    /// Conversation the message belongs to.
    pub chat: Chat,

    // ── Sender info ───────────────────────────────────────────────────────────

    /// Sender of the message; empty for messages sent to channels. For backward
    /// compatibility, the field contains a fake sender user in non-channel chats,
    /// if the message was sent on behalf of a chat.
    ///
    /// Renamed from Python's `from` (reserved keyword).
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from_user: Option<User>,

    /// Sender of the message when sent on behalf of a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Chat>,

    /// If the sender of the message boosted the chat, the number of boosts added by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_boost_count: Option<i32>,

    /// Bot through which the message was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_business_bot: Option<User>,

    /// Unique identifier of the business connection from which the message was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    // ── Thread / topic ────────────────────────────────────────────────────────

    /// Unique identifier of a message thread to which the message belongs; for supergroups only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// `true` if the message is sent to a forum topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,

    // ── Forward info ──────────────────────────────────────────────────────────

    /// Information about the original message for forwarded messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_origin: Option<MessageOrigin>,

    /// `true` if the message is a channel post that was automatically forwarded to the
    /// connected discussion group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,

    // ── Reply info ────────────────────────────────────────────────────────────

    /// For replies in the same chat and message thread, the original message.
    /// Note: The Message object in this field will not contain further `reply_to_message`
    /// fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,

    /// Information about the message that is being replied to, which may come from another
    /// chat or forum topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<Box<ExternalReplyInfo>>,

    /// For replies that quote part of the original message, the quoted part of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<TextQuote>,

    /// For replies to a story, the original story.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_story: Option<Story>,

    /// Bot that originated the query if the message is a response to an inline query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,

    // ── Edit date ─────────────────────────────────────────────────────────────

    /// Date the message was last edited in Unix time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,

    // ── Flags ─────────────────────────────────────────────────────────────────

    /// `true` if the message can't be forwarded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,

    /// `true` if the message was sent by an implicit action, for example, as an away or a
    /// greeting business message, or as a scheduled message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_offline: Option<bool>,

    /// `true` if the message is a paid post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paid_post: Option<bool>,

    // ── Media group ───────────────────────────────────────────────────────────

    /// The unique identifier of a media message group this message belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,

    // ── Signature / author ────────────────────────────────────────────────────

    /// Signature of the post author for messages in channels, or the custom title of an
    /// anonymous group administrator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,

    /// Tag of the user who posted the message if it is anonymous.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_tag: Option<String>,

    // ── Text & entities ───────────────────────────────────────────────────────

    /// For text messages, the actual UTF-8 text of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// For text messages, special entities like usernames, URLs, bot commands etc. that
    /// appear in the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    /// Options used for link preview generation for the message, if it is a text message and
    /// link preview options were changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    /// Unique identifier of the message effect added to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_id: Option<String>,

    // ── Media attachments ─────────────────────────────────────────────────────

    /// Message is an animation, information about the animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    /// Message is an audio file, information about the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    /// Message is a general file, information about the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    /// Message contains paid media; information about the paid media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,

    /// Message is a photo, available sizes of the photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,

    /// Message is a sticker, information about the sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    /// Message is a forwarded story.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,

    /// Message is a video, information about the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    /// Message is a video note, information about the video message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,

    /// Message is a voice message, information about the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,

    // ── Caption ───────────────────────────────────────────────────────────────

    /// Caption for the animation, audio, document, paid media, photo, video or voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs, bot commands etc.
    /// that appear in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// `true` if the caption must be shown above the message media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    /// `true` if the message media is covered by a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,

    // ── Contact / location / venue ────────────────────────────────────────────

    /// Message is a shared contact, information about the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    /// Message is a dice with random value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,

    /// Message is a game, information about the game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,

    /// Message is a native poll, information about the poll.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    /// Message is a venue, information about the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,

    /// Message is a shared location, information about the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    // ── Service messages — members ────────────────────────────────────────────

    /// New members that were added to the group or supergroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,

    /// A member was removed from the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,

    /// A chat title was changed to this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,

    /// A chat photo was changed to this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,

    /// Service message: the chat photo was deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,

    /// Service message: the group has been created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,

    /// Service message: the supergroup has been created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,

    /// Service message: the channel has been created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,

    /// Service message: auto-delete timer settings changed in the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,

    /// The group has been migrated to a supergroup with the specified identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    /// The supergroup has been migrated from a group with the specified identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,

    // ── Pinned / invoice / payment ────────────────────────────────────────────

    /// Specified message was pinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,

    /// Message is an invoice for a payment, information about the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,

    /// Message is a service message about a successful payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,

    /// Message is a service message about a refunded payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_payment: Option<RefundedPayment>,

    /// Message is a service message about the paid message price changing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_message_price_changed: Option<PaidMessagePriceChanged>,

    /// The number of Telegram Stars that were paid by the sender of the message to send it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_star_count: Option<i32>,

    // ── Website / Passport / connected ────────────────────────────────────────

    /// Service message: the user is connected to a website via the connected website feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,

    /// Write access was allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<WriteAccessAllowed>,

    /// Telegram Passport data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,

    // ── Proximity alert ───────────────────────────────────────────────────────

    /// Service message: a user in the chat triggered another user's proximity alert
    /// while sharing Live Location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,

    // ── Boost ─────────────────────────────────────────────────────────────────

    /// Service message: user boosted the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_added: Option<ChatBoostAdded>,

    // ── Chat background ───────────────────────────────────────────────────────

    /// Service message: chat background set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_background_set: Option<ChatBackground>,

    // ── Forum topic service messages ──────────────────────────────────────────

    /// Service message: forum topic created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<ForumTopicCreated>,

    /// Service message: forum topic edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<ForumTopicEdited>,

    /// Service message: forum topic closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<ForumTopicClosed>,

    /// Service message: forum topic reopened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<ForumTopicReopened>,

    /// Service message: the General forum topic hidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,

    /// Service message: the General forum topic unhidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,

    // ── Giveaway ──────────────────────────────────────────────────────────────

    /// Service message: a scheduled giveaway was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_created: Option<GiveawayCreated>,

    /// The message is a scheduled giveaway message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,

    /// A giveaway with public winners was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,

    /// Service message: a giveaway without public winners was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<GiveawayCompleted>,

    // ── Video chat ────────────────────────────────────────────────────────────

    /// Service message: video chat scheduled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<VideoChatScheduled>,

    /// Service message: video chat started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<VideoChatStarted>,

    /// Service message: video chat ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<VideoChatEnded>,

    /// Service message: new participants invited to a video chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,

    // ── Web app ───────────────────────────────────────────────────────────────

    /// Service message: data sent by a Web App.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,

    // ── Reply markup ──────────────────────────────────────────────────────────

    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    // ── Shared resources ──────────────────────────────────────────────────────

    /// Service message: users were shared with the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_shared: Option<UsersShared>,

    /// Service message: a chat was shared with the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_shared: Option<ChatShared>,

    // ── Gifts ─────────────────────────────────────────────────────────────────

    /// Message is a gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<GiftInfo>,

    /// A unique gift was sent or received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift: Option<UniqueGiftInfo>,

    /// Service message: a gift was upgraded and the original gift was sent to the receiver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_upgrade_sent: Option<GiftInfo>,

    // ── Checklists ────────────────────────────────────────────────────────────

    /// Message contains a checklist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist: Option<Checklist>,

    /// Service message: checklist tasks were done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_tasks_done: Option<ChecklistTasksDone>,

    /// Service message: checklist tasks were added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_tasks_added: Option<ChecklistTasksAdded>,

    /// Identifier of the checklist task the message is a reply to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_checklist_task_id: Option<i64>,

    // ── Direct messages ───────────────────────────────────────────────────────

    /// Service message: direct message price changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_message_price_changed: Option<DirectMessagePriceChanged>,

    /// Service message: the topic of a direct messages chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic: Option<DirectMessagesTopic>,

    // ── Suggested posts ───────────────────────────────────────────────────────

    /// Service message: a suggested post was declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_declined: Option<SuggestedPostDeclined>,

    /// Service message: a suggested post was paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_paid: Option<SuggestedPostPaid>,

    /// Service message: a suggested post was refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_refunded: Option<SuggestedPostRefunded>,

    /// Information about a suggested post in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_info: Option<SuggestedPostInfo>,

    /// Service message: a suggested post was approved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_approved: Option<SuggestedPostApproved>,

    /// Service message: a suggested post approval failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_approval_failed: Option<SuggestedPostApprovalFailed>,

    // ── Chat ownership changes ────────────────────────────────────────────────

    /// Service message: chat owner changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_owner_changed: Option<ChatOwnerChanged>,

    /// Service message: chat owner left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_owner_left: Option<ChatOwnerLeft>,

    // ── Managed bots (Bot API 9.6) ───────────────────────────────────────────

    /// Service message: user created a bot that will be managed by the current bot.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_bot_created: Option<ManagedBotCreated>,

    // ── Poll options (Bot API 9.6) ───────────────────────────────────────────

    /// Service message: answer option was added to a poll.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_option_added: Option<PollOptionAdded>,

    /// Service message: answer option was deleted from a poll.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_option_deleted: Option<PollOptionDeleted>,

    /// Persistent identifier of the poll option the message is a reply to.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_poll_option_id: Option<String>,

    // ── Catch-all ─────────────────────────────────────────────────────────────

    /// Catch-all for any extra fields returned by the Bot API not yet modelled here.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// InaccessibleMessage
// ---------------------------------------------------------------------------

/// Describes a message that was deleted or is otherwise inaccessible to the bot.
///
/// Corresponds to the Bot API
/// [`InaccessibleMessage`](https://core.telegram.org/bots/api#inaccessiblemessage) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InaccessibleMessage {
    /// Chat the message belonged to.
    pub chat: Chat,

    /// Unique message identifier inside the chat.
    pub message_id: i64,

    /// Always 0. The field can be used to differentiate regular and inaccessible messages.
    pub date: i64,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// MaybeInaccessibleMessage
// ---------------------------------------------------------------------------

/// Union type: either a full [`Message`] or an [`InaccessibleMessage`].
///
/// Discriminated by whether `date == 0` (inaccessible) or not.
///
/// Corresponds to the Bot API
/// [`MaybeInaccessibleMessage`](https://core.telegram.org/bots/api#maybeinaccessiblemessage) type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    /// A regular, accessible message (`date != 0`).
    Message(Message),
    /// A deleted or otherwise inaccessible message (`date == 0`).
    Inaccessible(InaccessibleMessage),
}

impl MaybeInaccessibleMessage {
    /// Returns a reference to the [`Chat`] regardless of variant.
    #[must_use]
    pub fn chat(&self) -> &Chat {
        match self {
            Self::Message(m) => &m.chat,
            Self::Inaccessible(m) => &m.chat,
        }
    }

    /// Returns the message id regardless of variant.
    #[must_use]
    pub fn message_id(&self) -> i64 {
        match self {
            Self::Message(m) => m.message_id,
            Self::Inaccessible(m) => m.message_id,
        }
    }

    /// Returns a reference to the inner [`Message`] if accessible, `None` otherwise.
    #[must_use]
    pub fn as_message(&self) -> Option<&Message> {
        match self {
            Self::Message(m) => Some(m),
            Self::Inaccessible(_) => None,
        }
    }
}
