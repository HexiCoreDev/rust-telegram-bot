use serde::{Deserialize, Serialize};

/// Inline keyboard button that copies specified text to the clipboard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CopyTextButton {
    /// The text to be copied to the clipboard; 1-256 characters.
    pub text: String,
}

impl_new!(CopyTextButton { text: String });
