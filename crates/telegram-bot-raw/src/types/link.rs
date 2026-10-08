use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Link
// ---------------------------------------------------------------------------

/// An HTTP link.
///
/// Corresponds to the Bot API [`Link`](https://core.telegram.org/bots/api#link) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Link {
    /// URL of the link.
    pub url: String,
}

impl Link {
    /// Creates a link.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}
