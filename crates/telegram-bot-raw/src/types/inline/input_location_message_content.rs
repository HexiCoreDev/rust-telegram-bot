use serde::{Deserialize, Serialize};

/// Represents the content of a location message to be sent as the result of an inline query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees.
    pub latitude: f64,

    /// Longitude of the location in degrees.
    pub longitude: f64,

    /// The radius of uncertainty for the location, measured in meters; 0-1500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,

    /// Period in seconds for which the location will be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,

    /// Direction in which the user is moving, in degrees; 1-360.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i32>,

    /// Maximum distance for proximity alerts about approaching another chat member, in meters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i32>,
}

impl InputLocationMessageContent {
    /// Creates a new `InputLocationMessageContent`.
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            ..Default::default()
        }
    }
}
