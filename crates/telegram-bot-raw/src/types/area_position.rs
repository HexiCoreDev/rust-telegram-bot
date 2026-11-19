use serde::{Deserialize, Serialize};

/// Describes the position of a rectangle area on a story media or a TON wallet card.
///
/// Corresponds to the Bot API [`AreaPosition`](https://core.telegram.org/bots/api#areaposition) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct AreaPosition {
    /// Abscissa of the area's center, as a percentage of the media or the card width.
    pub x_percentage: f64,

    /// Ordinate of the area's center, as a percentage of the media or the card height.
    pub y_percentage: f64,

    /// Width of the area's rectangle, as a percentage of the media or the card width.
    pub width_percentage: f64,

    /// Height of the area's rectangle, as a percentage of the media or the card height.
    pub height_percentage: f64,

    /// Clockwise rotation angle of the rectangle, in degrees; 0-360.
    pub rotation_angle: f64,

    /// Radius of the rectangle corner rounding, as a percentage of the media or the card width.
    pub corner_radius_percentage: f64,
}

impl AreaPosition {
    /// Creates a new `AreaPosition`.
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
