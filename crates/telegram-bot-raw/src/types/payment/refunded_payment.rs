use serde::{Deserialize, Serialize};

/// Basic information about a refunded payment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundedPayment {
    /// Three-letter ISO 4217 currency code, or `XTR`. Currently always `XTR`.
    pub currency: String,

    /// Total refunded price in the smallest units of the currency.
    pub total_amount: i64,

    /// Bot-specified invoice payload.
    pub invoice_payload: String,

    /// Telegram payment identifier.
    pub telegram_payment_charge_id: String,

    /// Provider payment identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payment_charge_id: Option<String>,
}
