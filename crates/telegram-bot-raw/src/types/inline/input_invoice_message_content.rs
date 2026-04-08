
use serde::{Deserialize, Serialize};

use crate::types::payment::labeled_price::LabeledPrice;

/// Represents the content of an invoice message to be sent as the result of an inline query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InputInvoiceMessageContent {
    /// Product name, 1-32 characters.
    pub title: String,

    /// Product description, 1-255 characters.
    pub description: String,

    /// Bot-defined invoice payload, 1-128 bytes.
    pub payload: String,

    /// Payment provider token. Pass an empty string for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,

    /// Three-letter ISO 4217 currency code. Pass "XTR" for payments in Telegram Stars.
    pub currency: String,

    /// Price breakdown, a list of components.
    pub prices: Vec<LabeledPrice>,

    /// The maximum accepted amount for tips in the smallest units of the currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,

    /// Suggested amounts of tip in the smallest units of the currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,

    /// An object for data about the invoice, which will be shared with the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,

    /// URL of the product photo for the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i32>,

    /// Pass `true` if you require the user's full name to complete the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,

    /// Pass `true` if the final price depends on the shipping method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}
