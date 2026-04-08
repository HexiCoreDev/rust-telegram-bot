
use serde::{Deserialize, Serialize};

use crate::types::files::location::Location;

/// A venue — a named place with an address and optional third-party IDs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Venue {
    /// Geographic coordinates of the venue.
    pub location: Location,

    /// Name of the venue.
    pub title: String,

    /// Address of the venue.
    pub address: String,

    /// Foursquare identifier of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,

    /// Foursquare type of the venue (e.g. `"arts_entertainment/aquarium"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,

    /// Google Places identifier of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,

    /// Google Places type of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

impl Venue {
    /// Creates a new `Venue` with the given location, title, and address.
    pub fn new(
        location: Location,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            location,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }
}
