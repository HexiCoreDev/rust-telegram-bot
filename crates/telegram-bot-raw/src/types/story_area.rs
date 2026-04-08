
use serde::{Deserialize, Serialize};

use super::reaction::ReactionType;

/// Position of a clickable area within a story.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryAreaPosition {
    /// Abscissa of the area's center, as a percentage of the media width.
    pub x_percentage: f64,

    /// Ordinate of the area's center, as a percentage of the media height.
    pub y_percentage: f64,

    /// Width of the area's rectangle, as a percentage of the media width.
    pub width_percentage: f64,

    /// Height of the area's rectangle, as a percentage of the media height.
    pub height_percentage: f64,

    /// Clockwise rotation angle of the rectangle, in degrees.
    pub rotation_angle: f64,

    /// Radius of the rectangle corner rounding, as a percentage of the media width.
    pub corner_radius_percentage: f64,
}

/// Physical address of a location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationAddress {
    /// Two-letter ISO 3166-1 alpha-2 country code.
    pub country_code: String,

    /// State of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// City of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Street address of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
}

/// A story area pointing to a geographic location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryAreaTypeLocationData {
    /// Location latitude in degrees.
    pub latitude: f64,

    /// Location longitude in degrees.
    pub longitude: f64,

    /// Address of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<LocationAddress>,
}

/// A story area pointing to a suggested reaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryAreaTypeSuggestedReactionData {
    /// Type of the reaction.
    pub reaction_type: ReactionType,

    /// `true` if the reaction area has a dark background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dark: Option<bool>,

    /// `true` if the reaction area corner is flipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flipped: Option<bool>,
}

/// A story area pointing to an HTTP or `tg://` link.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryAreaTypeLinkData {
    /// HTTP or `tg://` URL to be opened when the area is clicked.
    pub url: String,
}

/// A story area containing weather information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryAreaTypeWeatherData {
    /// Temperature in degrees Celsius.
    pub temperature: f64,

    /// Emoji representing the weather.
    pub emoji: String,

    /// Background color of the area in ARGB format.
    pub background_color: i64,
}

/// A story area pointing to a unique gift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryAreaTypeUniqueGiftData {
    /// Unique name of the gift.
    pub name: String,
}

/// Polymorphic type of a clickable area on a story, selected by the `"type"` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StoryAreaType {
    /// Area pointing to a geographic location.
    Location(StoryAreaTypeLocationData),

    /// Area pointing to a suggested reaction.
    SuggestedReaction(StoryAreaTypeSuggestedReactionData),

    /// Area pointing to an HTTP or `tg://` link.
    Link(StoryAreaTypeLinkData),

    /// Area containing weather information.
    Weather(StoryAreaTypeWeatherData),

    /// Area pointing to a unique gift.
    UniqueGift(StoryAreaTypeUniqueGiftData),
}

/// A clickable area on a story media.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryArea {
    /// Position of the area.
    pub position: StoryAreaPosition,

    /// Type of the area.
    #[serde(rename = "type")]
    pub area_type: StoryAreaType,
}
