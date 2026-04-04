use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::user::User;

use super::shipping_address::ShippingAddress;

/// Information about an incoming shipping query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier.
    pub id: String,

    /// User who sent the query. JSON field name is `"from"`.
    #[serde(rename = "from")]
    pub from_user: User,

    /// Bot-specified invoice payload.
    pub invoice_payload: String,

    /// User specified shipping address.
    pub shipping_address: ShippingAddress,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
