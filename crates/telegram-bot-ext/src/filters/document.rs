//! Document filters -- the `filters.Document` namespace.

use crate::filters::base::{Filter, FilterResult, Update};

// ---------------------------------------------------------------------------
// Document.ALL
// ---------------------------------------------------------------------------

/// Matches any message that contains a `document` field.
pub struct DocumentAll;

impl Filter for DocumentAll {
    fn check_update(&self, update: &Update) -> FilterResult {
        if update
            .effective_message()
            .and_then(|m| m.document.as_ref())
            .is_some()
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

/// Matches documents whose MIME type starts with a given category prefix
/// (e.g. `"audio/"`, `"image/"`).
pub struct DocumentCategory {
    /// The MIME type category prefix to match against.
    category: String,
    /// Human-readable display name for debugging.
    display: String,
}

impl DocumentCategory {
    /// Create a new category filter for the given MIME type prefix.
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
        let matched = update
            .effective_message()
            .and_then(|m| m.document.as_ref())
            .and_then(|d| d.mime_type.as_deref())
            .map(|mime| mime.starts_with(&self.category))
            .unwrap_or(false);

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
// Document.FileExtension
// ---------------------------------------------------------------------------

/// Matches documents by file extension.
///
/// When `extension` is `None`, matches files with no extension. When set,
/// matches files whose name ends with the given extension. Comparison is
/// case-insensitive by default unless `case_sensitive` is `true`.
pub struct DocumentFileExtension {
    /// The file extension to match (including leading dot), or `None` to match
    /// files without an extension.
    extension: Option<String>,
    /// Whether the extension comparison is case-sensitive.
    case_sensitive: bool,
    /// Human-readable display name for debugging.
    display: String,
}

impl DocumentFileExtension {
    /// Create a new file extension filter.
    ///
    /// Pass `None` to match documents without a file extension. When
    /// `case_sensitive` is `false`, comparison is done in lowercase.
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
        let doc = match update.effective_message().and_then(|m| m.document.as_ref()) {
            Some(d) => d,
            None => return FilterResult::NoMatch,
        };
        let file_name = match doc.file_name.as_deref() {
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

/// Matches documents whose MIME type exactly equals a given string.
pub struct DocumentMimeType {
    /// The exact MIME type to match.
    mimetype: String,
    /// Human-readable display name for debugging.
    display: String,
}

impl DocumentMimeType {
    /// Create a new MIME type filter for the given type string.
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
        let matched = update
            .effective_message()
            .and_then(|m| m.document.as_ref())
            .and_then(|d| d.mime_type.as_deref())
            .map(|mime| mime == self.mimetype)
            .unwrap_or(false);

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
// Pre-built constants (namespace module)
// ---------------------------------------------------------------------------

/// Pre-built document filter constants and convenience constructors.
pub mod presets {
    use super::*;

    /// Matches any message containing a document.
    pub const ALL: DocumentAll = DocumentAll;

    /// Create a filter matching documents with an `application/*` MIME type.
    pub fn application() -> DocumentCategory {
        DocumentCategory::new("application/")
    }
    /// Create a filter matching documents with an `audio/*` MIME type.
    pub fn audio() -> DocumentCategory {
        DocumentCategory::new("audio/")
    }
    /// Create a filter matching documents with an `image/*` MIME type.
    pub fn image() -> DocumentCategory {
        DocumentCategory::new("image/")
    }
    /// Create a filter matching documents with a `video/*` MIME type.
    pub fn video() -> DocumentCategory {
        DocumentCategory::new("video/")
    }
    /// Create a filter matching documents with a `text/*` MIME type.
    pub fn text() -> DocumentCategory {
        DocumentCategory::new("text/")
    }

    /// Create a filter matching APK files (`application/vnd.android.package-archive`).
    pub fn apk() -> DocumentMimeType {
        DocumentMimeType::new("application/vnd.android.package-archive")
    }
    /// Create a filter matching DOC files (`application/msword`).
    pub fn doc() -> DocumentMimeType {
        DocumentMimeType::new("application/msword")
    }
    /// Create a filter matching DOCX files.
    pub fn docx() -> DocumentMimeType {
        DocumentMimeType::new(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        )
    }
    /// Create a filter matching EXE files (`application/x-msdos-program`).
    pub fn exe() -> DocumentMimeType {
        DocumentMimeType::new("application/x-msdos-program")
    }
    /// Create a filter matching GIF files (`image/gif`).
    pub fn gif() -> DocumentMimeType {
        DocumentMimeType::new("image/gif")
    }
    /// Create a filter matching JPG files (`image/jpeg`).
    pub fn jpg() -> DocumentMimeType {
        DocumentMimeType::new("image/jpeg")
    }
    /// Create a filter matching MP3 files (`audio/mpeg`).
    pub fn mp3() -> DocumentMimeType {
        DocumentMimeType::new("audio/mpeg")
    }
    /// Create a filter matching MP4 files (`video/mp4`).
    pub fn mp4() -> DocumentMimeType {
        DocumentMimeType::new("video/mp4")
    }
    /// Create a filter matching PDF files (`application/pdf`).
    pub fn pdf() -> DocumentMimeType {
        DocumentMimeType::new("application/pdf")
    }
    /// Create a filter matching Python files (`text/x-python`).
    pub fn py() -> DocumentMimeType {
        DocumentMimeType::new("text/x-python")
    }
    /// Create a filter matching SVG files (`image/svg+xml`).
    pub fn svg() -> DocumentMimeType {
        DocumentMimeType::new("image/svg+xml")
    }
    /// Create a filter matching plain text files (`text/plain`).
    pub fn txt() -> DocumentMimeType {
        DocumentMimeType::new("text/plain")
    }
    /// Create a filter matching tar.gz files (`application/x-compressed-tar`).
    pub fn targz() -> DocumentMimeType {
        DocumentMimeType::new("application/x-compressed-tar")
    }
    /// Create a filter matching WAV files (`audio/x-wav`).
    pub fn wav() -> DocumentMimeType {
        DocumentMimeType::new("audio/x-wav")
    }
    /// Create a filter matching XML files (`text/xml`).
    pub fn xml() -> DocumentMimeType {
        DocumentMimeType::new("text/xml")
    }
    /// Create a filter matching ZIP files (`application/zip`).
    pub fn zip() -> DocumentMimeType {
        DocumentMimeType::new("application/zip")
    }
}

/// Re-export of [`presets`] for ergonomic `document::ALL` access.
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
