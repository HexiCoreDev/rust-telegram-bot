use serde::{Deserialize, Serialize};

use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;

/// Represents a venue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct InlineQueryResultVenue {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// Latitude of the venue location in degrees.
    pub latitude: f64,

    /// Longitude of the venue location in degrees.
    pub longitude: f64,

    /// Title of the venue.
    pub title: String,

    /// Address of the venue.
    pub address: String,

    /// Foursquare identifier of the venue, if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,

    /// Foursquare type of the venue, if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,

    /// Google Places identifier of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,

    /// Google Places type of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,

    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    /// URL of the thumbnail for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,

    /// Thumbnail width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i32>,

    /// Thumbnail height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i32>,
}

impl InlineQueryResultVenue {
    /// Creates a new `InlineQueryResultVenue`.
    pub fn new(
        id: impl Into<String>,
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            ..Default::default()
        }
    }
}
