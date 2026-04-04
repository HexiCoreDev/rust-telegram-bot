use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Shipping address provided by a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code.
    pub country_code: String,

    /// State, if applicable.
    pub state: String,

    /// City.
    pub city: String,

    /// First line of the address.
    pub street_line1: String,

    /// Second line of the address.
    pub street_line2: String,

    /// Address post code.
    pub post_code: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
