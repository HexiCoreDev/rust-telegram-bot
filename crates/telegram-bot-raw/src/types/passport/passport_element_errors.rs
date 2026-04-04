use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents an issue in one of the data fields provided by the user.
///
/// Resolved when the field's value changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorDataField {
    /// The section of the user's Telegram Passport which has the error.
    /// One of `"personal_details"`, `"passport"`, `"driver_license"`, `"identity_card"`,
    /// `"internal_passport"`, `"address"`.
    #[serde(rename = "type")]
    pub element_type: String,

    /// Name of the data field which has the error.
    pub field_name: String,

    /// Base64-encoded data hash.
    pub data_hash: String,

    /// Error message.
    pub message: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Represents an issue with a document scan.
///
/// Resolved when the file with the document scan changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFile {
    /// The section of the user's Telegram Passport which has the issue.
    /// One of `"utility_bill"`, `"bank_statement"`, `"rental_agreement"`,
    /// `"passport_registration"`, `"temporary_registration"`.
    #[serde(rename = "type")]
    pub element_type: String,

    /// Base64-encoded file hash.
    pub file_hash: String,

    /// Error message.
    pub message: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Represents an issue with a list of scans.
///
/// Resolved when the list of files with the document scans changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFiles {
    /// The section of the user's Telegram Passport which has the issue.
    /// One of `"utility_bill"`, `"bank_statement"`, `"rental_agreement"`,
    /// `"passport_registration"`, `"temporary_registration"`.
    #[serde(rename = "type")]
    pub element_type: String,

    /// List of base64-encoded file hashes.
    pub file_hashes: Vec<String>,

    /// Error message.
    pub message: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Represents an issue with the front side of a document.
///
/// Resolved when the file with the front side of the document changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFrontSide {
    /// The section of the user's Telegram Passport which has the issue.
    /// One of `"passport"`, `"driver_license"`, `"identity_card"`, `"internal_passport"`.
    #[serde(rename = "type")]
    pub element_type: String,

    /// Base64-encoded hash of the file with the front side of the document.
    pub file_hash: String,

    /// Error message.
    pub message: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Represents an issue with the reverse side of a document.
///
/// Resolved when the file with the reverse side of the document changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorReverseSide {
    /// The section of the user's Telegram Passport which has the issue.
    /// One of `"driver_license"`, `"identity_card"`.
    #[serde(rename = "type")]
    pub element_type: String,

    /// Base64-encoded hash of the file with the reverse side of the document.
    pub file_hash: String,

    /// Error message.
    pub message: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Represents an issue with the selfie photo with a document.
///
/// Resolved when the file with the selfie changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorSelfie {
    /// The section of the user's Telegram Passport which has the issue.
    /// One of `"passport"`, `"driver_license"`, `"identity_card"`, `"internal_passport"`.
    #[serde(rename = "type")]
    pub element_type: String,

    /// Base64-encoded hash of the file with the selfie.
    pub file_hash: String,

    /// Error message.
    pub message: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Represents an issue with one of the files that constitute the translation of a document.
///
/// Resolved when the file changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFile {
    /// Type of element of the user's Telegram Passport which has the issue.
    /// One of `"passport"`, `"driver_license"`, `"identity_card"`, `"internal_passport"`,
    /// `"utility_bill"`, `"bank_statement"`, `"rental_agreement"`,
    /// `"passport_registration"`, `"temporary_registration"`.
    #[serde(rename = "type")]
    pub element_type: String,

    /// Base64-encoded hash of the file.
    pub file_hash: String,

    /// Error message.
    pub message: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Represents an issue with the translated version of a document.
///
/// Resolved when a file with the document translation changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    /// Type of element of the user's Telegram Passport which has the issue.
    /// One of `"passport"`, `"driver_license"`, `"identity_card"`, `"internal_passport"`,
    /// `"utility_bill"`, `"bank_statement"`, `"rental_agreement"`,
    /// `"passport_registration"`, `"temporary_registration"`.
    #[serde(rename = "type")]
    pub element_type: String,

    /// List of base64-encoded file hashes.
    pub file_hashes: Vec<String>,

    /// Error message.
    pub message: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Represents an issue in an unspecified place.
///
/// Resolved when new data is added.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorUnspecified {
    /// Type of element of the user's Telegram Passport which has the issue.
    #[serde(rename = "type")]
    pub element_type: String,

    /// Base64-encoded element hash.
    pub element_hash: String,

    /// Error message.
    pub message: String,

    /// Extra fields not yet covered by this struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Polymorphic error in a Telegram Passport element submitted by the user.
///
/// The `"source"` field in the JSON payload selects the variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum PassportElementError {
    /// Issue in one of the data fields.
    Data(PassportElementErrorDataField),

    /// Issue with a single document scan file.
    File(PassportElementErrorFile),

    /// Issue with the list of document scan files.
    Files(PassportElementErrorFiles),

    /// Issue with the front side of a document.
    FrontSide(PassportElementErrorFrontSide),

    /// Issue with the reverse side of a document.
    ReverseSide(PassportElementErrorReverseSide),

    /// Issue with the selfie photo.
    Selfie(PassportElementErrorSelfie),

    /// Issue with a single translation file.
    TranslationFile(PassportElementErrorTranslationFile),

    /// Issue with the list of translation files.
    TranslationFiles(PassportElementErrorTranslationFiles),

    /// Issue in an unspecified place.
    Unspecified(PassportElementErrorUnspecified),
}
