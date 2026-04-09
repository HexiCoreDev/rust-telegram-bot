use serde::{Deserialize, Serialize};

use crate::types::passport::credentials::EncryptedCredentials;
use crate::types::passport::encrypted_passport_element::EncryptedPassportElement;

/// Contains information about Telegram Passport data shared with the bot by the user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PassportData {
    /// Array with encrypted information about documents and other Telegram Passport elements
    /// shared with the bot.
    pub data: Vec<EncryptedPassportElement>,

    /// Encrypted credentials required to decrypt the data.
    pub credentials: EncryptedCredentials,
}
