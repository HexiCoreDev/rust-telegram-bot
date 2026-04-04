use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Basic information about an invoice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invoice {
    /// Product name.
    pub title: String,

    /// Product description.
    pub description: String,

    /// Unique bot deep-linking parameter that can be used to generate this invoice.
    pub start_parameter: String,

    /// Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars.
    pub currency: String,

    /// Total price in the smallest units of the currency.
    pub total_amount: i64,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
