use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::shipping_address::ShippingAddress;

/// Information about an order provided by the user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderInfo {
    /// User name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// User's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    /// User email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// User shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
