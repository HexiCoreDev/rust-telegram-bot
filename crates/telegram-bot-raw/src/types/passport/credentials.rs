use serde::{Deserialize, Serialize};

/// Contains data required for decrypting and authenticating `EncryptedPassportElement`.
///
/// See the [Telegram Passport documentation] for a complete description of the data decryption
/// and authentication processes.
///
/// [Telegram Passport documentation]: https://core.telegram.org/passport
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and
    /// secrets required for `EncryptedPassportElement` decryption and authentication.
    pub data: String,

    /// Base64-encoded data hash for data authentication.
    pub hash: String,

    /// Base64-encoded secret, encrypted with the bot's public RSA key, required for data
    /// decryption.
    pub secret: String,
}

/// Decrypted credentials required to decrypt `EncryptedPassportElement` data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Credentials {
    /// Credentials for encrypted data.
    pub secure_data: SecureData,

    /// Bot-specified nonce.
    pub nonce: String,
}

/// Credentials used to decrypt the encrypted values for each requested passport field.
///
/// All fields are optional and depend on which fields were requested.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecureData {
    /// Credentials for encrypted personal details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<SecureValue>,

    /// Credentials for encrypted passport.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<SecureValue>,

    /// Credentials for encrypted internal passport.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_passport: Option<SecureValue>,

    /// Credentials for encrypted driver license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_license: Option<SecureValue>,

    /// Credentials for encrypted identity card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_card: Option<SecureValue>,

    /// Credentials for encrypted residential address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<SecureValue>,

    /// Credentials for encrypted utility bill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_bill: Option<SecureValue>,

    /// Credentials for encrypted bank statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_statement: Option<SecureValue>,

    /// Credentials for encrypted rental agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rental_agreement: Option<SecureValue>,

    /// Credentials for encrypted registration from internal passport.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_registration: Option<SecureValue>,

    /// Credentials for encrypted temporary registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_registration: Option<SecureValue>,
}

/// Credentials used to decrypt an individual encrypted passport value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecureValue {
    /// Credentials for encrypted Telegram Passport data (personal details, address, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DataCredentials>,

    /// Credentials for the encrypted document front side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<FileCredentials>,

    /// Credentials for the encrypted document reverse side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<FileCredentials>,

    /// Credentials for the encrypted selfie with a document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<FileCredentials>,

    /// Credentials for encrypted files (scans, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileCredentials>>,

    /// Credentials for encrypted translation files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<FileCredentials>>,
}

/// Credentials used to decrypt encrypted data from the `data` field of
/// `EncryptedPassportElement`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DataCredentials {
    /// Checksum of encrypted data.
    pub data_hash: String,

    /// Secret of encrypted data.
    pub secret: String,
}

/// Credentials used to decrypt encrypted files from `EncryptedPassportElement`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FileCredentials {
    /// Checksum of the encrypted file.
    pub file_hash: String,

    /// Secret of the encrypted file.
    pub secret: String,
}
