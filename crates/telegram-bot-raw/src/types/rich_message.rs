use serde::{Deserialize, Serialize};

use super::rich_block::RichBlock;

// ---------------------------------------------------------------------------
// RichMessage
// ---------------------------------------------------------------------------

/// A rich formatted message.
///
/// Corresponds to the Bot API [`RichMessage`](https://core.telegram.org/bots/api#richmessage) type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct RichMessage {
    /// Content of the message.
    pub blocks: Vec<RichBlock>,

    /// `true` if the rich message must be shown right-to-left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rtl: Option<bool>,
}

impl RichMessage {
    /// Creates a rich message from a list of blocks.
    pub fn new(blocks: Vec<RichBlock>) -> Self {
        Self {
            blocks,
            is_rtl: None,
        }
    }
}
