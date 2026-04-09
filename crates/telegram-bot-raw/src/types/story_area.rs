use serde::{Deserialize, Serialize};

use super::reaction::ReactionType;

/// Position of a clickable area within a story.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl StoryAreaPosition {
    /// Creates a new `StoryAreaPosition`.
    pub fn new(
        x_percentage: f64,
        y_percentage: f64,
        width_percentage: f64,
        height_percentage: f64,
        rotation_angle: f64,
        corner_radius_percentage: f64,
    ) -> Self {
        Self {
            x_percentage,
            y_percentage,
            width_percentage,
            height_percentage,
            rotation_angle,
            corner_radius_percentage,
        }
    }
}

/// Physical address of a location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
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

impl_new!(LocationAddress {
    country_code: String
});

/// A story area pointing to a geographic location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
pub struct StoryAreaTypeLinkData {
    /// HTTP or `tg://` URL to be opened when the area is clicked.
    pub url: String,
}

/// A story area containing weather information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct StoryAreaTypeUniqueGiftData {
    /// Unique name of the gift.
    pub name: String,
}

/// Polymorphic type of a clickable area on a story, selected by the `"type"` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
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

impl StoryAreaType {
    /// Create a location story area type.
    pub fn location(latitude: f64, longitude: f64) -> Self {
        Self::Location(StoryAreaTypeLocationData {
            latitude,
            longitude,
            address: None,
        })
    }

    /// Create a suggested reaction story area type.
    pub fn suggested_reaction(reaction_type: ReactionType) -> Self {
        Self::SuggestedReaction(StoryAreaTypeSuggestedReactionData {
            reaction_type,
            is_dark: None,
            is_flipped: None,
        })
    }

    /// Create a link story area type.
    pub fn link(url: impl Into<String>) -> Self {
        Self::Link(StoryAreaTypeLinkData { url: url.into() })
    }

    /// Create a weather story area type.
    pub fn weather(temperature: f64, emoji: impl Into<String>, background_color: i64) -> Self {
        Self::Weather(StoryAreaTypeWeatherData {
            temperature,
            emoji: emoji.into(),
            background_color,
        })
    }

    /// Create a unique gift story area type.
    pub fn unique_gift(name: impl Into<String>) -> Self {
        Self::UniqueGift(StoryAreaTypeUniqueGiftData { name: name.into() })
    }
}

/// A clickable area on a story media.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StoryArea {
    /// Position of the area.
    pub position: StoryAreaPosition,

    /// Type of the area.
    #[serde(rename = "type")]
    pub area_type: StoryAreaType,
}

impl StoryArea {
    /// Creates a new `StoryArea`.
    pub fn new(position: StoryAreaPosition, area_type: StoryAreaType) -> Self {
        Self {
            position,
            area_type,
        }
    }
}
