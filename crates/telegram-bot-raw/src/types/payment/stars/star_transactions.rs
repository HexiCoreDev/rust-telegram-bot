use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::transaction_partner::TransactionPartner;

/// A Telegram Star transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarTransaction {
    /// Unique identifier of the transaction.
    pub id: String,

    /// Integer amount of Telegram Stars transferred by the transaction.
    pub amount: i64,

    /// Fractional nanostar shares transferred; from 0 to 999999999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,

    /// Date the transaction was created as a Unix timestamp.
    pub date: i64,

    /// Source of an incoming transaction; only for incoming transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TransactionPartner>,

    /// Receiver of an outgoing transaction; only for outgoing transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<TransactionPartner>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A list of Telegram Star transactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarTransactions {
    /// The list of transactions.
    pub transactions: Vec<StarTransaction>,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
