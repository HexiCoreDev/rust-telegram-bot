//! Collects all parameters and files needed for one request to the Telegram Bot API.
//!
//! This mirrors `telegram.request.RequestData` from python-telegram-bot.

use std::collections::HashMap;

use serde_json::Value;

use super::request_parameter::RequestParameter;

/// Aggregates all [`RequestParameter`]s for a single Bot API call.
///
/// # Relationship to Python source
///
/// | Python attribute / property | Rust equivalent |
/// |---|---|
/// | `contains_files` | [`RequestData::contains_files()`] |
/// | `parameters` | [`RequestData::parameters()`] |
/// | `json_parameters` | [`RequestData::json_parameters()`] |
/// | `json_payload` | [`RequestData::json_payload()`] |
/// | `multipart_data` | [`RequestData::multipart_data()`] |
/// | `url_encoded_parameters` | [`RequestData::url_encoded_parameters()`] |
/// | `parametrized_url` | [`RequestData::parametrized_url()`] |
#[derive(Debug, Clone, Default)]
pub struct RequestData {
    parameters: Vec<RequestParameter>,
}

impl RequestData {
    /// Create an empty [`RequestData`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a [`RequestData`] from an existing list of parameters.
    ///
    /// ```
    /// use rust_tg_bot_raw::request::request_data::RequestData;
    /// use rust_tg_bot_raw::request::request_parameter::RequestParameter;
    /// use serde_json::json;
    ///
    /// let params = vec![RequestParameter::new("chat_id", json!(42))];
    /// let data = RequestData::from_parameters(params);
    /// assert!(!data.contains_files());
    /// ```
    pub fn from_parameters(parameters: Vec<RequestParameter>) -> Self {
        Self { parameters }
    }

    /// Append a single parameter.
    pub fn push(&mut self, param: RequestParameter) {
        self.parameters.push(param);
    }

    /// Iterate over all parameters.
    pub fn iter(&self) -> impl Iterator<Item = &RequestParameter> {
        self.parameters.iter()
    }

    /// Returns `true` when at least one parameter carries attached files.
    ///
    /// Mirrors `RequestData.contains_files` in Python.
    ///
    /// ```
    /// use rust_tg_bot_raw::request::request_data::RequestData;
    /// use rust_tg_bot_raw::request::request_parameter::{InputFileRef, RequestParameter};
    ///
    /// let file = InputFileRef::direct(vec![0u8]);
    /// let p = RequestParameter::file_only("photo", file);
    /// let data = RequestData::from_parameters(vec![p]);
    /// assert!(data.contains_files());
    /// ```
    pub fn contains_files(&self) -> bool {
        self.parameters.iter().any(|p| p.input_files.is_some())
    }

    /// All parameters as a `HashMap<name, Value>`, excluding those whose value
    /// is `None`.
    ///
    /// Mirrors `RequestData.parameters` in Python.
    pub fn parameters(&self) -> HashMap<&str, &Value> {
        self.parameters
            .iter()
            .filter_map(|p| p.value.as_ref().map(|v| (p.name.as_ref(), v)))
            .collect()
    }

    /// All parameters as `HashMap<name, json_encoded_string>`, excluding those
    /// whose JSON value is `None`.
    ///
    /// Mirrors `RequestData.json_parameters` in Python.
    ///
    /// ```
    /// use rust_tg_bot_raw::request::request_data::RequestData;
    /// use rust_tg_bot_raw::request::request_parameter::RequestParameter;
    /// use serde_json::json;
    ///
    /// let data = RequestData::from_parameters(vec![
    ///     RequestParameter::new("chat_id", json!(99)),
    ///     RequestParameter::new("text", json!("hello")),
    /// ]);
    /// let jp = data.json_parameters();
    /// assert_eq!(jp.get("chat_id").map(String::as_str), Some("99"));
    /// assert_eq!(jp.get("text").map(String::as_str), Some("hello"));
    /// ```
    pub fn json_parameters(&self) -> HashMap<String, String> {
        self.parameters
            .iter()
            .filter_map(|p| p.json_value().map(|v| (p.name.as_ref().to_owned(), v)))
            .collect()
    }

    /// Serialize the JSON parameters to a UTF-8 byte payload.
    ///
    /// Mirrors `RequestData.json_payload` in Python.
    ///
    /// ```
    /// use rust_tg_bot_raw::request::request_data::RequestData;
    /// use rust_tg_bot_raw::request::request_parameter::RequestParameter;
    /// use serde_json::json;
    ///
    /// let data = RequestData::from_parameters(vec![
    ///     RequestParameter::new("chat_id", json!(1)),
    /// ]);
    /// let payload = data.json_payload();
    /// let parsed: serde_json::Value = serde_json::from_slice(&payload).unwrap();
    /// assert_eq!(parsed["chat_id"], json!("1"));
    /// ```
    pub fn json_payload(&self) -> Vec<u8> {
        let map = self.json_parameters();
        // serde_json serialises HashMap<String, String> as a JSON object.
        serde_json::to_vec(&map).expect("HashMap<String,String> is always serialisable")
    }

    /// URL-encode all JSON parameters.
    ///
    /// Mirrors `RequestData.url_encoded_parameters` in Python (without the
    /// `encode_kwargs` variant — use [`Self::url_encoded_parameters_with`] for
    /// that).
    ///
    /// ```
    /// use rust_tg_bot_raw::request::request_data::RequestData;
    /// use rust_tg_bot_raw::request::request_parameter::RequestParameter;
    /// use serde_json::json;
    ///
    /// let data = RequestData::from_parameters(vec![
    ///     RequestParameter::new("a", json!("1")),
    /// ]);
    /// let encoded = data.url_encoded_parameters();
    /// assert!(encoded.contains("a=1"));
    /// ```
    pub fn url_encoded_parameters(&self) -> String {
        let map = self.json_parameters();
        // Build the query string manually to avoid pulling in an extra crate.
        // This matches `urllib.parse.urlencode` for the common case.
        map.iter()
            .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
            .collect::<Vec<_>>()
            .join("&")
    }

    /// Attach the URL-encoded parameters to a base URL with a `?` separator.
    ///
    /// Mirrors `RequestData.parametrized_url` in Python.
    ///
    /// ```
    /// use rust_tg_bot_raw::request::request_data::RequestData;
    /// use rust_tg_bot_raw::request::request_parameter::RequestParameter;
    /// use serde_json::json;
    ///
    /// let data = RequestData::from_parameters(vec![
    ///     RequestParameter::new("offset", json!("0")),
    /// ]);
    /// let url = data.parametrized_url("https://api.telegram.org/bot<token>/getUpdates");
    /// assert!(url.starts_with("https://api.telegram.org/bot<token>/getUpdates?"));
    /// ```
    pub fn parametrized_url(&self, url: &str) -> String {
        format!("{}?{}", url, self.url_encoded_parameters())
    }

    /// Collect multipart form parts contributed by all parameters.
    ///
    /// Returns `None` when [`Self::contains_files`] is `false`.
    ///
    /// The outer `HashMap` maps multipart part names to `(bytes, mime_type,
    /// file_name)` triples so that [`crate::request::reqwest_impl`] can
    /// assemble a `reqwest::multipart::Form` without needing to reach back into
    /// this module.
    pub fn multipart_data(&self) -> Option<HashMap<String, MultipartPart>> {
        if !self.contains_files() {
            return None;
        }

        let mut out: HashMap<String, MultipartPart> = HashMap::new();

        for param in &self.parameters {
            if let Some(parts) = param.multipart_data() {
                for (part_name, file_ref) in parts {
                    out.insert(
                        part_name,
                        MultipartPart {
                            bytes: file_ref.bytes.clone(),
                            mime_type: file_ref.effective_mime().to_owned(),
                            file_name: file_ref.file_name.clone(),
                        },
                    );
                }
            }
        }

        Some(out)
    }
}

/// A single binary part to be included in a `multipart/form-data` upload.
#[derive(Debug, Clone)]
pub struct MultipartPart {
    /// Raw file bytes.
    pub bytes: Vec<u8>,
    /// MIME type, e.g. `"image/jpeg"`.
    pub mime_type: String,
    /// Optional filename hint.
    pub file_name: Option<String>,
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Percent-encode a string for use in a URL query string.
///
/// Encodes all characters outside the unreserved set (A–Z a–z 0–9 `-` `_` `.`
/// `~`) as `%XX`.  Spaces are encoded as `%20` (not `+`), matching the
/// behaviour of Python's `urllib.parse.quote` (which `urlencode` uses
/// internally for both keys and values).
fn percent_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            _ => {
                use std::fmt::Write as _;
                let _ = write!(out, "%{byte:02X}");
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::request::request_parameter::{InputFileRef, RequestParameter};

    use super::*;

    fn make_plain(name: &'static str, v: Value) -> RequestParameter {
        RequestParameter::new(name, v)
    }

    // ------------------------------------------------------------------
    // contains_files
    // ------------------------------------------------------------------

    #[test]
    fn contains_files_false_for_plain_params() {
        let data = RequestData::from_parameters(vec![
            make_plain("chat_id", json!(1)),
            make_plain("text", json!("hi")),
        ]);
        assert!(!data.contains_files());
    }

    #[test]
    fn contains_files_true_when_file_present() {
        let file = InputFileRef::direct(vec![0xAB]);
        let p = RequestParameter::file_only("photo", file);
        let data = RequestData::from_parameters(vec![p]);
        assert!(data.contains_files());
    }

    // ------------------------------------------------------------------
    // json_parameters
    // ------------------------------------------------------------------

    #[test]
    fn json_parameters_excludes_none_values() {
        let file = InputFileRef::direct(vec![0]);
        let p = RequestParameter::file_only("photo", file);
        let data = RequestData::from_parameters(vec![
            make_plain("chat_id", json!(7)),
            p, // value is None
        ]);
        let jp = data.json_parameters();
        assert!(jp.contains_key("chat_id"));
        assert!(!jp.contains_key("photo"));
    }

    #[test]
    fn json_parameters_string_not_double_encoded() {
        let data = RequestData::from_parameters(vec![make_plain("text", json!("hello"))]);
        let jp = data.json_parameters();
        assert_eq!(jp["text"], "hello");
    }

    // ------------------------------------------------------------------
    // json_payload
    // ------------------------------------------------------------------

    #[test]
    fn json_payload_round_trips() {
        let data = RequestData::from_parameters(vec![
            make_plain("chat_id", json!(99)),
            make_plain("text", json!("world")),
        ]);
        let payload = data.json_payload();
        let parsed: serde_json::Value = serde_json::from_slice(&payload).unwrap();
        assert_eq!(parsed["chat_id"], json!("99"));
        assert_eq!(parsed["text"], json!("world"));
    }

    // ------------------------------------------------------------------
    // url_encoded_parameters
    // ------------------------------------------------------------------

    #[test]
    fn url_encoded_basic() {
        let data = RequestData::from_parameters(vec![make_plain("key", json!("val"))]);
        assert!(data.url_encoded_parameters().contains("key=val"));
    }

    #[test]
    fn url_encoded_spaces_encoded_as_percent20() {
        let data = RequestData::from_parameters(vec![make_plain("text", json!("hello world"))]);
        let encoded = data.url_encoded_parameters();
        assert!(encoded.contains("hello%20world"), "got: {encoded}");
    }

    // ------------------------------------------------------------------
    // parametrized_url
    // ------------------------------------------------------------------

    #[test]
    fn parametrized_url_has_question_mark() {
        let data = RequestData::from_parameters(vec![make_plain("x", json!("1"))]);
        let url = data.parametrized_url("https://example.com/api");
        assert!(url.starts_with("https://example.com/api?"));
    }

    // ------------------------------------------------------------------
    // multipart_data
    // ------------------------------------------------------------------

    #[test]
    fn multipart_data_none_without_files() {
        let data = RequestData::from_parameters(vec![make_plain("chat_id", json!(1))]);
        assert!(data.multipart_data().is_none());
    }

    #[test]
    fn multipart_data_includes_file_bytes() {
        let file = InputFileRef::direct(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let p = RequestParameter::file_only("sticker", file);
        let data = RequestData::from_parameters(vec![p]);
        let parts = data.multipart_data().unwrap();
        let part = parts.get("sticker").expect("part named 'sticker'");
        assert_eq!(part.bytes, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn multipart_data_uses_attach_name_as_key() {
        let file = InputFileRef::with_attach_name("media0", vec![0x01]);
        let p = RequestParameter::with_files("media", json!("attach://media0"), vec![file]);
        let data = RequestData::from_parameters(vec![p]);
        let parts = data.multipart_data().unwrap();
        assert!(
            parts.contains_key("media0"),
            "expected key 'media0', got: {parts:?}"
        );
    }

    // ------------------------------------------------------------------
    // percent_encode helper
    // ------------------------------------------------------------------

    #[test]
    fn percent_encode_unreserved_unchanged() {
        assert_eq!(percent_encode("abc-_.~"), "abc-_.~");
    }

    #[test]
    fn percent_encode_space() {
        assert_eq!(percent_encode(" "), "%20");
    }

    #[test]
    fn percent_encode_ampersand() {
        assert_eq!(percent_encode("a&b"), "a%26b");
    }
}
