use serde::{Deserialize, Serialize};

/// Personal details as stored in a Telegram Passport document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PersonalDetails {
    /// First name.
    pub first_name: String,

    /// Last name.
    pub last_name: String,

    /// Middle name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,

    /// Date of birth in `DD.MM.YYYY` format.
    pub birth_date: String,

    /// Gender: `"male"` or `"female"`.
    pub gender: String,

    /// Citizenship (ISO 3166-1 alpha-2 country code).
    pub country_code: String,

    /// Country of residence (ISO 3166-1 alpha-2 country code).
    pub residence_country_code: String,

    /// First name in the language of the user's country of residence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_native: Option<String>,

    /// Last name in the language of the user's country of residence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_native: Option<String>,

    /// Middle name in the language of the user's country of residence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name_native: Option<String>,
}

/// Residential address as stored in a Telegram Passport document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ResidentialAddress {
    /// First line of the address.
    pub street_line1: String,

    /// Second line of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_line2: Option<String>,

    /// City.
    pub city: String,

    /// State.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// ISO 3166-1 alpha-2 country code.
    pub country_code: String,

    /// Address post code.
    pub post_code: String,
}

/// Data of an identity document (passport, driver license, identity card).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IdDocumentData {
    /// Document number.
    pub document_no: String,

    /// Date of expiry in `DD.MM.YYYY` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
}
