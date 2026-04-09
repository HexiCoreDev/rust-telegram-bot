use serde::{Deserialize, Serialize};

/// The withdrawal is in progress.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RevenueWithdrawalStatePending {}

/// The withdrawal succeeded.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RevenueWithdrawalStateSucceeded {
    /// Date the withdrawal was completed as a Unix timestamp.
    pub date: i64,

    /// An HTTPS URL to see transaction details.
    pub url: String,
}

/// The withdrawal failed and the transaction was refunded.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RevenueWithdrawalStateFailed {}

/// State of a revenue withdrawal operation.
///
/// Discriminated by the `"type"` JSON field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum RevenueWithdrawalState {
    /// Withdrawal is in progress.
    Pending(RevenueWithdrawalStatePending),

    /// Withdrawal succeeded.
    Succeeded(RevenueWithdrawalStateSucceeded),

    /// Withdrawal failed.
    Failed(RevenueWithdrawalStateFailed),
}
