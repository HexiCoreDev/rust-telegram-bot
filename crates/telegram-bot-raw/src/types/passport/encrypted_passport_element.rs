use serde::{Deserialize, Serialize};

use crate::types::passport::passport_file::PassportFile;

/// Contains information about documents or other Telegram Passport elements shared with the bot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptedPassportElement {
    /// Element type. One of `"personal_details"`, `"passport"`, `"driver_license"`,
    /// `"identity_card"`, `"internal_passport"`, `"address"`, `"utility_bill"`,
    /// `"bank_statement"`, `"rental_agreement"`, `"passport_registration"`,
    /// `"temporary_registration"`, `"phone_number"`, `"email"`.
    #[serde(rename = "type")]
    pub element_type: String,

    /// Base64-encoded element hash for use in `PassportElementErrorUnspecified`.
    pub hash: String,

    /// Base64-encoded encrypted Telegram Passport element data; available for
    /// `"personal_details"`, `"passport"`, `"driver_license"`, `"identity_card"`,
    /// `"internal_passport"` and `"address"` types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// User's verified phone number; available only for the `"phone_number"` type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    /// User's verified email address; available only for the `"email"` type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Array of encrypted files with documents provided by the user; available for
    /// `"utility_bill"`, `"bank_statement"`, `"rental_agreement"`,
    /// `"passport_registration"` and `"temporary_registration"` types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,

    /// Encrypted file with the front side of the document; available for `"passport"`,
    /// `"driver_license"`, `"identity_card"` and `"internal_passport"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,

    /// Encrypted file with the reverse side of the document; available for
    /// `"driver_license"` and `"identity_card"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,

    /// Encrypted file with the selfie of the user holding a document; available for
    /// `"passport"`, `"driver_license"`, `"identity_card"` and `"internal_passport"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,

    /// Array of encrypted files with translated versions of documents; available for
    /// `"passport"`, `"driver_license"`, `"identity_card"`, `"internal_passport"`,
    /// `"utility_bill"`, `"bank_statement"`, `"rental_agreement"`,
    /// `"passport_registration"` and `"temporary_registration"` types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}
