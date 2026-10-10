use serde::{Deserialize, Serialize};

use super::chat::Chat;
use super::files::animation::Animation;
use super::files::audio::Audio;
use super::files::document::Document;
use super::files::input_media::{
    InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaLink, InputMediaLivePhoto,
    InputMediaLocation, InputMediaPhoto, InputMediaSticker, InputMediaVenue, InputMediaVideo,
};
use super::files::location::Location;
use super::files::photo_size::PhotoSize;
use super::files::sticker::Sticker;
use super::files::venue::Venue;
use super::files::video::Video;
use super::link::Link;
use super::live_photo::LivePhoto;
use super::message::Message;
use super::message_entity::MessageEntity;
use super::user::User;

/// One answer option in a poll to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InputPollOption {
    /// Option text; 1-100 characters.
    pub text: String,

    /// Mode for parsing entities in the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,

    /// Special entities that appear in the option text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,

    /// Media added to the poll option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<InputPollOptionMedia>,
}

impl_new!(InputPollOption { text: String });

/// One answer option in a received poll.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PollOption {
    /// Option text; 1-100 characters.
    pub text: String,

    /// Number of users that voted for this option.
    pub voter_count: i64,

    /// Special entities that appear in the option text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,

    /// Media added to the poll option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Box<PollMedia>>,

    /// Persistent identifier for the option.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_id: Option<String>,

    /// User that added the option. Absent for options created with the poll.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_by_user: Option<User>,

    /// Chat that added the option. Absent for options created with the poll.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_by_chat: Option<Chat>,

    /// Unix timestamp when the option was added.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addition_date: Option<i64>,
}

/// An answer of a user in a non-anonymous poll.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PollAnswer {
    /// Unique poll identifier.
    pub poll_id: String,

    /// Identifiers of answer options chosen by the user. May be empty if the user retracted
    /// their vote.
    pub option_ids: Vec<i64>,

    /// The user that changed the answer to the poll, if the voter isn't anonymous.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// The chat that changed the answer to the poll, if the voter is anonymous.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Chat>,

    /// Persistent identifiers of the chosen options.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_persistent_ids: Option<Vec<String>>,
}

/// Information about a poll.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Poll {
    /// Unique poll identifier.
    pub id: String,

    /// Poll question; 1-300 characters.
    pub question: String,

    /// List of poll options.
    pub options: Vec<PollOption>,

    /// Total number of users that voted in the poll.
    pub total_voter_count: i64,

    /// `true` if the poll is closed.
    pub is_closed: bool,

    /// `true` if the poll is anonymous.
    pub is_anonymous: bool,

    /// Poll type, either `"regular"` or `"quiz"`.
    #[serde(rename = "type")]
    pub poll_type: String,

    /// `true` if the poll allows multiple answers.
    pub allows_multiple_answers: bool,

    /// Zero-based identifier of the correct answer option. Available only for closed quiz polls
    /// sent or received by the bot.
    ///
    /// Deprecated in Bot API 9.6 — use `correct_option_ids` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,

    /// Zero-based identifiers of the correct answer options. Available only for closed quiz polls
    /// sent or received by the bot.
    ///
    /// Added in Bot API 9.6 (replaces `correct_option_id`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_ids: Option<Vec<i64>>,

    /// Text shown when a user chooses an incorrect answer or taps the lamp icon in a quiz poll.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Special entities in the explanation text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,

    /// Amount of time in seconds the poll will be active after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,

    /// Unix timestamp when the poll will be automatically closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,

    /// Special entities that appear in the poll question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,

    /// `true` if voters can revote.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_revoting: Option<bool>,

    /// Description of the poll.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Special entities in the description.
    ///
    /// Added in Bot API 9.6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_entities: Option<Vec<MessageEntity>>,

    /// Media added to the poll description; for polls inside a `Message` only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Box<PollMedia>>,

    /// Media added to a quiz explanation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_media: Option<Box<PollMedia>>,

    /// `true` if voting is limited to users who have been members of the chat
    /// where the poll was sent for more than 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_only: Option<bool>,

    /// Two-letter ISO 3166-1 alpha-2 country codes of countries whose users may vote.
    /// `"FT"` allows users with anonymous numbers. Omitted means all countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// PollOptionAdded
// ---------------------------------------------------------------------------

/// Service message about an option added to a poll.
///
/// Corresponds to the Bot API [`PollOptionAdded`](https://core.telegram.org/bots/api#polloptionadded) object.
///
/// Added in Bot API 9.6.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PollOptionAdded {
    /// Message containing the poll to which the option was added, if known.
    /// Note that the `Message` object in this field will not contain the `reply_to_message`
    /// field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_message: Option<Box<Message>>,

    /// Unique identifier of the added option.
    pub option_persistent_id: String,

    /// Option text.
    pub option_text: String,

    /// Special entities that appear in the option text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_text_entities: Option<Vec<MessageEntity>>,
}

// ---------------------------------------------------------------------------
// PollOptionDeleted
// ---------------------------------------------------------------------------

/// Service message about an option deleted from a poll.
///
/// Corresponds to the Bot API [`PollOptionDeleted`](https://core.telegram.org/bots/api#polloptiondeleted) object.
///
/// Added in Bot API 9.6.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PollOptionDeleted {
    /// Message containing the poll from which the option was deleted, if known.
    /// Note that the `Message` object in this field will not contain the `reply_to_message`
    /// field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_message: Option<Box<Message>>,

    /// Unique identifier of the deleted option.
    pub option_persistent_id: String,

    /// Option text.
    pub option_text: String,

    /// Special entities that appear in the option text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_text_entities: Option<Vec<MessageEntity>>,
}

// ---------------------------------------------------------------------------
// PollMedia
// ---------------------------------------------------------------------------

/// Media attached to a poll, a quiz explanation, or a poll option.
///
/// At most one of the optional fields is present in any given object.
/// Corresponds to the Bot API [`PollMedia`](https://core.telegram.org/bots/api#pollmedia) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct PollMedia {
    /// Media is an animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    /// Media is an audio file. Cannot currently be received in a poll option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    /// Media is a general file. Cannot currently be received in a poll option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    /// Media is a live photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_photo: Option<LivePhoto>,

    /// Media is a shared location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// Media is a photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,

    /// Media is a sticker. Currently, for poll options only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    /// Media is a venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,

    /// Media is a video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    /// The HTTP link attached to the poll option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}

impl_new!(PollMedia {});

// ---------------------------------------------------------------------------
// InputPollMedia / InputPollOptionMedia
// ---------------------------------------------------------------------------

/// Content of a poll description or quiz explanation to be sent.
/// Corresponds to the Bot API [`InputPollMedia`](https://core.telegram.org/bots/api#inputpollmedia) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputPollMedia {
    /// An animation.
    Animation(InputMediaAnimation),
    /// An audio file.
    Audio(InputMediaAudio),
    /// A general file.
    Document(InputMediaDocument),
    /// A live photo.
    LivePhoto(InputMediaLivePhoto),
    /// A location.
    Location(InputMediaLocation),
    /// A photo.
    Photo(InputMediaPhoto),
    /// A venue.
    Venue(InputMediaVenue),
    /// A video.
    Video(InputMediaVideo),
}

/// Content of a poll option to be sent.
/// Corresponds to the Bot API [`InputPollOptionMedia`](https://core.telegram.org/bots/api#inputpolloptionmedia) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputPollOptionMedia {
    /// An animation.
    Animation(InputMediaAnimation),
    /// An HTTP link.
    Link(InputMediaLink),
    /// A live photo.
    LivePhoto(InputMediaLivePhoto),
    /// A location.
    Location(InputMediaLocation),
    /// A photo.
    Photo(InputMediaPhoto),
    /// A sticker.
    Sticker(InputMediaSticker),
    /// A venue.
    Venue(InputMediaVenue),
    /// A video.
    Video(InputMediaVideo),
}
