use serde::{Deserialize, Serialize};

/// A portion of the total price for goods or services.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LabeledPrice {
    /// Portion label.
    pub label: String,

    /// Price of the product in the smallest units of the currency.
    pub amount: i64,
}

impl_new!(LabeledPrice {
    label: String,
    amount: i64
});
