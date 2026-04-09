use serde::{Deserialize, Serialize};

use super::inline_query_result_article::InlineQueryResultArticle;
use super::inline_query_result_audio::InlineQueryResultAudio;
use super::inline_query_result_cached_audio::InlineQueryResultCachedAudio;
use super::inline_query_result_cached_document::InlineQueryResultCachedDocument;
use super::inline_query_result_cached_gif::InlineQueryResultCachedGif;
use super::inline_query_result_cached_mpeg4_gif::InlineQueryResultCachedMpeg4Gif;
use super::inline_query_result_cached_photo::InlineQueryResultCachedPhoto;
use super::inline_query_result_cached_sticker::InlineQueryResultCachedSticker;
use super::inline_query_result_cached_video::InlineQueryResultCachedVideo;
use super::inline_query_result_cached_voice::InlineQueryResultCachedVoice;
use super::inline_query_result_contact::InlineQueryResultContact;
use super::inline_query_result_document::InlineQueryResultDocument;
use super::inline_query_result_game::InlineQueryResultGame;
use super::inline_query_result_gif::InlineQueryResultGif;
use super::inline_query_result_location::InlineQueryResultLocation;
use super::inline_query_result_mpeg4_gif::InlineQueryResultMpeg4Gif;
use super::inline_query_result_photo::InlineQueryResultPhoto;
use super::inline_query_result_venue::InlineQueryResultVenue;
use super::inline_query_result_video::InlineQueryResultVideo;
use super::inline_query_result_voice::InlineQueryResultVoice;

/// Represents one result of an inline query.
///
/// Deserialization is untagged because several result types share the same `"type"` string
/// (e.g. `"audio"` covers both `InlineQueryResultAudio` and `InlineQueryResultCachedAudio`).
/// Serde tries each variant in declaration order and picks the first that succeeds.
/// Cached variants are listed before their URL-based counterparts so that the presence of a
/// `*_file_id` field takes precedence during deserialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum InlineQueryResult {
    /// A link to an article or web page.
    Article(InlineQueryResultArticle),
    /// A link to an mp3 audio file stored on the Telegram servers.
    CachedAudio(InlineQueryResultCachedAudio),
    /// A link to an mp3 audio file.
    Audio(InlineQueryResultAudio),
    /// A link to a file stored on the Telegram servers.
    CachedDocument(InlineQueryResultCachedDocument),
    /// A link to a file.
    Document(InlineQueryResultDocument),
    /// A link to an animated GIF stored on the Telegram servers.
    CachedGif(InlineQueryResultCachedGif),
    /// A link to an animated GIF file.
    Gif(InlineQueryResultGif),
    /// A link to a video animation (MPEG-4) stored on the Telegram servers.
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    /// A link to a video animation (MPEG-4).
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    /// A link to a photo stored on the Telegram servers.
    CachedPhoto(InlineQueryResultCachedPhoto),
    /// A link to a photo.
    Photo(InlineQueryResultPhoto),
    /// A link to a sticker stored on the Telegram servers.
    CachedSticker(InlineQueryResultCachedSticker),
    /// A link to a video file stored on the Telegram servers.
    CachedVideo(InlineQueryResultCachedVideo),
    /// A link to a page containing an embedded video player or a video file.
    Video(InlineQueryResultVideo),
    /// A link to a voice message stored on the Telegram servers.
    CachedVoice(InlineQueryResultCachedVoice),
    /// A link to a voice recording.
    Voice(InlineQueryResultVoice),
    /// A contact with a phone number.
    Contact(InlineQueryResultContact),
    /// A game.
    Game(InlineQueryResultGame),
    /// A location on a map.
    Location(InlineQueryResultLocation),
    /// A venue.
    Venue(InlineQueryResultVenue),
}
