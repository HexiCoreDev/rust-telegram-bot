//! A single parameter in a Telegram Bot API request.
//!
//! This mirrors `telegram.request.RequestParameter` from python-telegram-bot.  The
//! Python class handles many Telegram-specific helper types (InputFile, InputMedia,
//! TelegramObject, …) during conversion.  In the Rust port those domain types will
//! eventually convert themselves into a `RequestParameter` via `From`/`Into`
//! implementations; until the full type layer exists the value is kept as a raw
//! `serde_json::Value` together with an optional list of attached binary files.

use std::borrow::Cow;

use serde_json::Value;

/// Metadata for a single file that is uploaded as part of a multipart form.
///
/// The `attach_name` is the name used in the `attach://<name>` URI scheme.
/// When it is `None` the file must be sent as the sole binary part and the
/// parameter value itself is omitted from the JSON payload (matching the
/// Python `if value.attach_uri` branch).
#[derive(Debug, Clone)]
pub struct InputFileRef {
    /// Multipart form part name.  When present the JSON value is
    /// `"attach://<attach_name>"`.  When absent the file is sent directly and
    /// the parameter's JSON value is `None`.
    pub attach_name: Option<String>,
    /// Raw file bytes.
    pub bytes: Vec<u8>,
    /// Optional MIME type (defaults to `application/octet-stream`).
    pub mime_type: Option<String>,
    /// Optional file name hint sent to Telegram.
    pub file_name: Option<String>,
}

impl InputFileRef {
    /// Build an [`InputFileRef`] that is uploaded directly (no attach URI).
    pub fn direct(bytes: Vec<u8>) -> Self {
        Self {
            attach_name: None,
            bytes,
            mime_type: None,
            file_name: None,
        }
    }

    /// Build an [`InputFileRef`] uploaded via an `attach://<name>` URI.
    pub fn with_attach_name(attach_name: impl Into<String>, bytes: Vec<u8>) -> Self {
        Self {
            attach_name: Some(attach_name.into()),
            bytes,
            mime_type: None,
            file_name: None,
        }
    }

    /// The `attach://` URI string, if this file has an attach name.
    pub fn attach_uri(&self) -> Option<String> {
        self.attach_name.as_deref().map(|n| format!("attach://{n}"))
    }

    /// The MIME type string used when building the multipart part, falling back
    /// to `application/octet-stream`.
    pub fn effective_mime(&self) -> &str {
        self.mime_type
            .as_deref()
            .unwrap_or("application/octet-stream")
    }
}

/// A single named parameter sent to the Telegram Bot API.
///
/// # Relationship to Python source
///
/// | Python attribute | Rust field |
/// |---|---|
/// | `name` | [`RequestParameter::name`] |
/// | `value` | [`RequestParameter::value`] |
/// | `input_files` | [`RequestParameter::input_files`] |
///
/// The `json_value` and `multipart_data` Python properties are implemented as
/// methods here: [`RequestParameter::json_value`] and
/// [`RequestParameter::multipart_data`].
#[derive(Debug, Clone)]
pub struct RequestParameter {
    /// The API parameter name, e.g. `"chat_id"` or `"photo"`.
    pub name: Cow<'static, str>,

    /// The JSON-serialisable value.  `None` is used when the parameter consists
    /// solely of a file that must be uploaded without an attach URI (the Python
    /// branch `return None, [value]`).
    pub value: Option<Value>,

    /// Files to upload together with this parameter.
    pub input_files: Option<Vec<InputFileRef>>,
}

impl RequestParameter {
    /// Construct a plain (non-file) parameter.
    ///
    /// ```
    /// use telegram_bot_raw::request::request_parameter::RequestParameter;
    /// use serde_json::json;
    ///
    /// let p = RequestParameter::new("chat_id", json!(12345));
    /// assert_eq!(p.json_value().unwrap(), "12345");
    /// ```
    pub fn new(name: impl Into<Cow<'static, str>>, value: impl Into<Value>) -> Self {
        Self {
            name: name.into(),
            value: Some(value.into()),
            input_files: None,
        }
    }

    /// Construct a parameter that carries attached files alongside a JSON value.
    pub fn with_files(
        name: impl Into<Cow<'static, str>>,
        value: impl Into<Value>,
        files: Vec<InputFileRef>,
    ) -> Self {
        Self {
            name: name.into(),
            value: Some(value.into()),
            input_files: Some(files),
        }
    }

    /// Construct a file-only parameter where the JSON value is absent (the
    /// Python `return None, [value]` case).
    pub fn file_only(name: impl Into<Cow<'static, str>>, file: InputFileRef) -> Self {
        Self {
            name: name.into(),
            value: None,
            input_files: Some(vec![file]),
        }
    }

    /// The JSON-encoded string representation of [`Self::value`], or `None`
    /// when the value is absent.
    ///
    /// Mirrors `RequestParameter.json_value` in Python:
    /// - `str` values are returned as-is (without extra JSON quotes).
    /// - All other values are serialised with `serde_json::to_string`.
    ///
    /// ```
    /// use telegram_bot_raw::request::request_parameter::RequestParameter;
    /// use serde_json::json;
    ///
    /// let string_param = RequestParameter::new("text", json!("hello"));
    /// // String values are returned verbatim, not double-encoded.
    /// assert_eq!(string_param.json_value().unwrap(), "hello");
    ///
    /// let int_param = RequestParameter::new("count", json!(42));
    /// assert_eq!(int_param.json_value().unwrap(), "42");
    ///
    /// let bool_param = RequestParameter::new("enabled", json!(true));
    /// assert_eq!(bool_param.json_value().unwrap(), "true");
    /// ```
    pub fn json_value(&self) -> Option<String> {
        match &self.value {
            None => None,
            Some(Value::String(s)) => Some(s.clone()),
            Some(v) => Some(v.to_string()),
        }
    }

    /// Produce the multipart parts contributed by this parameter's files.
    ///
    /// Returns `None` when there are no attached files.
    ///
    /// The returned iterator yields `(part_name, file)` pairs where `part_name`
    /// is either the file's `attach_name` or, when absent, the parameter name.
    /// This mirrors the Python dict comprehension:
    /// ```python
    /// {(input_file.attach_name or self.name): input_file.field_tuple ...}
    /// ```
    pub fn multipart_data(&self) -> Option<Vec<(String, &InputFileRef)>> {
        let files = self.input_files.as_ref()?;
        let parts: Vec<(String, &InputFileRef)> = files
            .iter()
            .map(|f| {
                let part_name = f
                    .attach_name
                    .clone()
                    .unwrap_or_else(|| self.name.as_ref().to_owned());
                (part_name, f)
            })
            .collect();
        Some(parts)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn json_value_string_not_double_encoded() {
        let p = RequestParameter::new("text", json!("hello world"));
        assert_eq!(p.json_value().unwrap(), "hello world");
    }

    #[test]
    fn json_value_integer() {
        let p = RequestParameter::new("chat_id", json!(99));
        assert_eq!(p.json_value().unwrap(), "99");
    }

    #[test]
    fn json_value_bool() {
        let p = RequestParameter::new("disable_notification", json!(false));
        assert_eq!(p.json_value().unwrap(), "false");
    }

    #[test]
    fn json_value_none_when_file_only() {
        let file = InputFileRef::direct(vec![0u8, 1, 2]);
        let p = RequestParameter::file_only("photo", file);
        assert!(p.json_value().is_none());
    }

    #[test]
    fn json_value_object() {
        let p = RequestParameter::new("reply_markup", json!({"inline_keyboard": []}));
        let s = p.json_value().unwrap();
        let reparsed: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(reparsed["inline_keyboard"], json!([]));
    }

    #[test]
    fn multipart_data_none_for_plain_param() {
        let p = RequestParameter::new("chat_id", json!(1));
        assert!(p.multipart_data().is_none());
    }

    #[test]
    fn multipart_data_uses_attach_name() {
        let file = InputFileRef::with_attach_name("my_file", vec![0xff]);
        let p = RequestParameter::with_files("document", json!("attach://my_file"), vec![file]);
        let parts = p.multipart_data().unwrap();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].0, "my_file");
    }

    #[test]
    fn multipart_data_falls_back_to_param_name() {
        let file = InputFileRef::direct(vec![0xde, 0xad]);
        let p = RequestParameter::file_only("photo", file);
        let parts = p.multipart_data().unwrap();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].0, "photo");
    }

    #[test]
    fn attach_uri_present_when_name_set() {
        let file = InputFileRef::with_attach_name("vid", vec![]);
        assert_eq!(file.attach_uri().unwrap(), "attach://vid");
    }

    #[test]
    fn attach_uri_none_for_direct_file() {
        let file = InputFileRef::direct(vec![]);
        assert!(file.attach_uri().is_none());
    }
}
