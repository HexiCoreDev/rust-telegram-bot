
use serde::{Deserialize, Serialize};

use crate::types::user::User;

use super::order_info::OrderInfo;

/// Information about an incoming pre-checkout query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier.
    pub id: String,

    /// User who sent the query. JSON field name is `"from"`.
    #[serde(rename = "from")]
    pub from_user: User,

    /// Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars.
    pub currency: String,

    /// Total price in the smallest units of the currency.
    pub total_amount: i64,

    /// Bot-specified invoice payload.
    pub invoice_payload: String,

    /// Identifier of the shipping option chosen by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,

    /// Order info provided by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}
