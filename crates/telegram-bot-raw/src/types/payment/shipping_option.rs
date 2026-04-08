
use serde::{Deserialize, Serialize};

use super::labeled_price::LabeledPrice;

/// One shipping option.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingOption {
    /// Shipping option identifier.
    pub id: String,

    /// Option title.
    pub title: String,

    /// List of price portions.
    pub prices: Vec<LabeledPrice>,
}

impl ShippingOption {
    /// Creates a new `ShippingOption` with the given id, title, and prices.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            prices,
        }
    }
}
