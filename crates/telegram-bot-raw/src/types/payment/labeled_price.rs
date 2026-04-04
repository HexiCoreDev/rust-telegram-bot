use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A portion of the total price for goods or services.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabeledPrice {
    /// Portion label.
    pub label: String,

    /// Price of the product in the smallest units of the currency.
    pub amount: i64,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
