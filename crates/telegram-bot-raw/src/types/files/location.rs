
use serde::{Deserialize, Serialize};

/// A point on the map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Location {
    /// Longitude as defined by the sender.
    pub longitude: f64,

    /// Latitude as defined by the sender.
    pub latitude: f64,

    /// Radius of uncertainty for the location, measured in meters (0–1500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,

    /// Time in seconds relative to the message sending date during which the location
    /// can be updated. For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,

    /// Direction in which the user is moving, in degrees (1–360). For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,

    /// Maximum distance for proximity alerts about approaching another chat member, in meters.
    /// For sent live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
}

impl Location {
    /// Creates a new `Location` with the given latitude and longitude.
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            ..Default::default()
        }
    }
}
