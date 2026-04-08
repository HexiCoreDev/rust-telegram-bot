
use serde::{Deserialize, Serialize};

use crate::types::files::location::Location;

/// Represents a location to which a chat is connected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected. Cannot be a live location.
    pub location: Location,
    /// Location address as defined by the chat owner (1–64 characters).
    pub address: String,
}
