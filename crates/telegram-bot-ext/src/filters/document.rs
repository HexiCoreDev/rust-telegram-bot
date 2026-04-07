//! Document filters -- the `filters.Document` namespace.

use crate::filters::base::{effective_message_val, to_value, Filter, FilterResult, Update};

// ---------------------------------------------------------------------------
// Document.ALL
// ---------------------------------------------------------------------------

/// Matches any message that contains a `document` field.
pub struct DocumentAll;

impl Filter for DocumentAll {
    fn check_update(&self, update: &Update) -> FilterResult {
        let __v = to_value(update);
        if effective_message_val(&__v)
            .and_then(|m| m.get("document"))
            .map(|v| !v.is_null())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.Document.ALL"
    }
}

// ---------------------------------------------------------------------------
// Document.Category
// ---------------------------------------------------------------------------

pub struct DocumentCategory {
    category: String,
    display: String,
}

impl DocumentCategory {
    pub fn new(category: impl Into<String>) -> Self {
        let cat: String = category.into();
        let display = format!("filters.Document.Category('{}')", cat);
        Self {
            category: cat,
            display,
        }
    }
}

impl Filter for DocumentCategory {
    fn check_update(&self, update: &Update) -> FilterResult {
        let __v = to_value(update);
        if effective_message_val(&__v)
            .and_then(|m| m.get("document"))
            .and_then(|d| d.get("mime_type"))
            .and_then(|v| v.as_str())
            .map(|mime| mime.starts_with(&self.category))
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// Document.FileExtension
// ---------------------------------------------------------------------------

pub struct DocumentFileExtension {
    extension: Option<String>,
    case_sensitive: bool,
    display: String,
}

impl DocumentFileExtension {
    pub fn new(extension: Option<&str>, case_sensitive: bool) -> Self {
        let (ext, display) = match extension {
            None => (None, "filters.Document.FileExtension(None)".to_owned()),
            Some(e) => {
                let stored = if case_sensitive {
                    format!(".{}", e)
                } else {
                    format!(".{}", e).to_lowercase()
                };
                let display = if case_sensitive {
                    format!(
                        "filters.Document.FileExtension({:?}, case_sensitive=true)",
                        e
                    )
                } else {
                    format!("filters.Document.FileExtension({:?})", e.to_lowercase())
                };
                (Some(stored), display)
            }
        };
        Self {
            extension: ext,
            case_sensitive,
            display,
        }
    }
}

impl Filter for DocumentFileExtension {
    fn check_update(&self, update: &Update) -> FilterResult {
        let __v = to_value(update);
        let doc = match effective_message_val(&__v).and_then(|m| m.get("document")) {
            Some(d) if !d.is_null() => d,
            _ => return FilterResult::NoMatch,
        };
        let file_name = match doc.get("file_name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return FilterResult::NoMatch,
        };

        let matched = match &self.extension {
            None => !file_name.contains('.'),
            Some(ext) => {
                let name = if self.case_sensitive {
                    file_name.to_owned()
                } else {
                    file_name.to_lowercase()
                };
                name.ends_with(ext.as_str())
            }
        };

        if matched {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// Document.MimeType
// ---------------------------------------------------------------------------

pub struct DocumentMimeType {
    mimetype: String,
    display: String,
}

impl DocumentMimeType {
    pub fn new(mimetype: impl Into<String>) -> Self {
        let mt: String = mimetype.into();
        let display = format!("filters.Document.MimeType('{}')", mt);
        Self {
            mimetype: mt,
            display,
        }
    }
}

impl Filter for DocumentMimeType {
    fn check_update(&self, update: &Update) -> FilterResult {
        let __v = to_value(update);
        if effective_message_val(&__v)
            .and_then(|m| m.get("document"))
            .and_then(|d| d.get("mime_type"))
            .and_then(|v| v.as_str())
            .map(|mime| mime == self.mimetype)
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// Pre-built constants (namespace module)
// ---------------------------------------------------------------------------

pub mod presets {
    use super::*;

    pub const ALL: DocumentAll = DocumentAll;

    pub fn application() -> DocumentCategory {
        DocumentCategory::new("application/")
    }
    pub fn audio() -> DocumentCategory {
        DocumentCategory::new("audio/")
    }
    pub fn image() -> DocumentCategory {
        DocumentCategory::new("image/")
    }
    pub fn video() -> DocumentCategory {
        DocumentCategory::new("video/")
    }
    pub fn text() -> DocumentCategory {
        DocumentCategory::new("text/")
    }

    pub fn apk() -> DocumentMimeType {
        DocumentMimeType::new("application/vnd.android.package-archive")
    }
    pub fn doc() -> DocumentMimeType {
        DocumentMimeType::new("application/msword")
    }
    pub fn docx() -> DocumentMimeType {
        DocumentMimeType::new(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        )
    }
    pub fn exe() -> DocumentMimeType {
        DocumentMimeType::new("application/x-msdos-program")
    }
    pub fn gif() -> DocumentMimeType {
        DocumentMimeType::new("image/gif")
    }
    pub fn jpg() -> DocumentMimeType {
        DocumentMimeType::new("image/jpeg")
    }
    pub fn mp3() -> DocumentMimeType {
        DocumentMimeType::new("audio/mpeg")
    }
    pub fn mp4() -> DocumentMimeType {
        DocumentMimeType::new("video/mp4")
    }
    pub fn pdf() -> DocumentMimeType {
        DocumentMimeType::new("application/pdf")
    }
    pub fn py() -> DocumentMimeType {
        DocumentMimeType::new("text/x-python")
    }
    pub fn svg() -> DocumentMimeType {
        DocumentMimeType::new("image/svg+xml")
    }
    pub fn txt() -> DocumentMimeType {
        DocumentMimeType::new("text/plain")
    }
    pub fn targz() -> DocumentMimeType {
        DocumentMimeType::new("application/x-compressed-tar")
    }
    pub fn wav() -> DocumentMimeType {
        DocumentMimeType::new("audio/x-wav")
    }
    pub fn xml() -> DocumentMimeType {
        DocumentMimeType::new("text/xml")
    }
    pub fn zip() -> DocumentMimeType {
        DocumentMimeType::new("application/zip")
    }
}

pub use presets as document;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn doc_update(mime: &str, file_name: &str) -> Update {
        serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "document": {
                    "file_id": "f1", "file_unique_id": "u1",
                    "mime_type": mime, "file_name": file_name
                }
            }
        }))
        .unwrap()
    }

    #[test]
    fn document_all() {
        assert!(DocumentAll
            .check_update(&doc_update("application/pdf", "test.pdf"))
            .is_match());
    }

    #[test]
    fn document_all_no_doc() {
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "text": "hi"}
        })).unwrap();
        assert!(!DocumentAll.check_update(&update).is_match());
    }

    #[test]
    fn document_category_audio() {
        let f = DocumentCategory::new("audio/");
        assert!(f
            .check_update(&doc_update("audio/mpeg", "song.mp3"))
            .is_match());
        assert!(!f
            .check_update(&doc_update("video/mp4", "vid.mp4"))
            .is_match());
    }

    #[test]
    fn document_mime_type() {
        let f = DocumentMimeType::new("application/pdf");
        assert!(f
            .check_update(&doc_update("application/pdf", "doc.pdf"))
            .is_match());
        assert!(!f
            .check_update(&doc_update("application/zip", "arc.zip"))
            .is_match());
    }

    #[test]
    fn document_file_extension() {
        let f = DocumentFileExtension::new(Some("pdf"), false);
        assert!(f
            .check_update(&doc_update("application/pdf", "report.PDF"))
            .is_match());
        assert!(!f
            .check_update(&doc_update("application/pdf", "report.docx"))
            .is_match());
    }

    #[test]
    fn document_file_extension_case_sensitive() {
        let f = DocumentFileExtension::new(Some("PDF"), true);
        assert!(f
            .check_update(&doc_update("application/pdf", "report.PDF"))
            .is_match());
        assert!(!f
            .check_update(&doc_update("application/pdf", "report.pdf"))
            .is_match());
    }

    #[test]
    fn document_file_extension_none() {
        let f = DocumentFileExtension::new(None, false);
        assert!(f
            .check_update(&doc_update("application/octet-stream", "Dockerfile"))
            .is_match());
        assert!(!f
            .check_update(&doc_update("application/pdf", "test.pdf"))
            .is_match());
    }

    #[test]
    fn document_namespace_shortcuts() {
        let pdf = document::pdf();
        assert!(pdf
            .check_update(&doc_update("application/pdf", "x.pdf"))
            .is_match());
        let mp3 = document::mp3();
        assert!(mp3
            .check_update(&doc_update("audio/mpeg", "song.mp3"))
            .is_match());
    }
}
