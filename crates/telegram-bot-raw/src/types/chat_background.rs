use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::files::document::Document;

// ---------------------------------------------------------------------------
// BackgroundFill — tagged union on the "type" field
// ---------------------------------------------------------------------------

/// The background is filled using a single color.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundFillSolid {
    /// The color of the background fill in the RGB24 format.
    pub color: i64,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The background is a gradient fill.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundFillGradient {
    /// Top color of the gradient in the RGB24 format.
    pub top_color: i64,
    /// Bottom color of the gradient in the RGB24 format.
    pub bottom_color: i64,
    /// Clockwise rotation angle of the background fill in degrees (0–359).
    pub rotation_angle: i64,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The background is a freeform gradient that rotates after every message in the chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundFillFreeformGradient {
    /// A list of the 3 or 4 base colors that are used to generate the freeform gradient
    /// in the RGB24 format.
    pub colors: Vec<i64>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The background fill of a chat. Discriminated by the `"type"` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackgroundFill {
    Solid(BackgroundFillSolid),
    Gradient(BackgroundFillGradient),
    FreeformGradient(BackgroundFillFreeformGradient),
}

// ---------------------------------------------------------------------------
// BackgroundType — tagged union on the "type" field
// ---------------------------------------------------------------------------

/// The background is automatically filled based on the selected colors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundTypeFill {
    /// The background fill.
    pub fill: BackgroundFill,
    /// Dimming of the background in dark themes, as a percentage (0–100).
    pub dark_theme_dimming: i64,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The background is a wallpaper in JPEG format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundTypeWallpaper {
    /// Document with the wallpaper.
    pub document: Document,
    /// Dimming of the background in dark themes, as a percentage (0–100).
    pub dark_theme_dimming: i64,

    /// True if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blurred: Option<bool>,
    /// True if the background moves slightly when the device is tilted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The background is a PNG or TGV pattern combined with a fill chosen by the user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundTypePattern {
    /// Document with the pattern.
    pub document: Document,
    /// The background fill that is combined with the pattern.
    pub fill: BackgroundFill,
    /// Intensity of the pattern when shown above the filled background (0–100).
    pub intensity: i64,

    /// True if the background fill must be applied only to the pattern itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inverted: Option<bool>,
    /// True if the background moves slightly when the device is tilted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The background is taken directly from a built-in chat theme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundTypeChatTheme {
    /// Name of the chat theme, which is usually an emoji.
    pub theme_name: String,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The type of background applied to a chat. Discriminated by the `"type"` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackgroundType {
    Fill(BackgroundTypeFill),
    Wallpaper(BackgroundTypeWallpaper),
    Pattern(BackgroundTypePattern),
    ChatTheme(BackgroundTypeChatTheme),
}

// ---------------------------------------------------------------------------
// ChatBackground
// ---------------------------------------------------------------------------

/// Represents a chat background.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatBackground {
    /// Type of the background.
    #[serde(rename = "type")]
    pub background_type: BackgroundType,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
