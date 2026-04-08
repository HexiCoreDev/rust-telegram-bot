use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::chat::Chat;
use super::checklists::Checklist;
use super::dice::Dice;
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
use super::games::game::Game;
use super::giveaway::{Giveaway, GiveawayWinners};
use super::link_preview_options::LinkPreviewOptions;
use super::message_entity::MessageEntity;
use super::message_origin::MessageOrigin;
use super::paid_media::PaidMediaInfo;
use super::payment::invoice::Invoice;
use super::poll::Poll;
use super::story::Story;

/// Information about a message that is being replied to, which may come from another chat or
/// forum topic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalReplyInfo {
    /// Origin of the message replied to by the given message.
    pub origin: MessageOrigin,

    /// Chat the original message belongs to. Available only if the chat is a supergroup or a
    /// channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,

    /// Unique message identifier inside the original chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    /// Options used for link preview generation for the original message, if it is a text message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    /// Message is an animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    /// Message is an audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    /// Message is a general file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    /// Message is a photo; available sizes of the photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,

    /// Message is a sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    /// Message is a forwarded story.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,

    /// Message is a video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    /// Message is a video note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,

    /// Message is a voice message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,

    /// `true` if the message media is covered by a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,

    /// Message is a checklist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist: Option<Checklist>,

    /// Message is a shared contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    /// Message is a dice with random value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,

    /// Message is a game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,

    /// Message is a scheduled giveaway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,

    /// A giveaway with public winners was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,

    /// Message is an invoice for a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,

    /// Message is a shared location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// Message is a native poll.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    /// Message is a venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,

    /// Message contains paid media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,
}

/// The quoted part of a message that is replied to by the given message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TextQuote {
    /// Text of the quoted part of a message that is replied to by the given message.
    pub text: String,

    /// Approximate quote position in the original message in UTF-16 code units.
    pub position: i64,

    /// Special entities that appear in the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    /// `true` if the quote was chosen manually by the message sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
}

impl_new!(TextQuote {
    text: String,
    position: i64
});

/// Reply parameters for the message that is being sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplyParameters {
    /// Identifier of the message that will be replied to in the current chat, or in the chat
    /// `chat_id` if it is specified.
    pub message_id: i64,

    /// If the message to be replied to is from a different chat, the identifier or username of
    /// that chat. Kept as Value since it can be either an integer chat id or a string username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Value>,

    /// Pass `true` if the message should be sent even if the specified message to be replied to
    /// is not found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    /// Quoted part of the message to be replied to; 0-1024 characters after entities parsing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,

    /// Mode for parsing entities in the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<String>,

    /// Special entities that appear in the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<MessageEntity>>,

    /// Position of the quote in the original message in UTF-16 code units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<i64>,

    /// Identifier of the specific checklist task to be replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_task_id: Option<i64>,

    /// Identifier of the specific poll option to be replied to.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_option_id: Option<String>,
}

impl_new!(ReplyParameters { message_id: i64 });
