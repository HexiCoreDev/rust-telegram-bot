
use serde::{Deserialize, Serialize};

use super::order_info::OrderInfo;

/// Basic information about a successful payment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars.
    pub currency: String,

    /// Total price in the smallest units of the currency.
    pub total_amount: i64,

    /// Bot-specified invoice payload.
    pub invoice_payload: String,

    /// Telegram payment identifier.
    pub telegram_payment_charge_id: String,

    /// Provider payment identifier.
    pub provider_payment_charge_id: String,

    /// Identifier of the shipping option chosen by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,

    /// Order info provided by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,

    /// Expiration date of the subscription as a Unix timestamp; for recurring payments only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_expiration_date: Option<i64>,

    /// True if the payment is for a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<bool>,

    /// True if this is the first payment of a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_first_recurring: Option<bool>,
}
