use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Represents a file to be uploaded to the Telegram API.
///
/// - `FileId` — reference an already-uploaded file by its `file_id`.
/// - `Url` — let Telegram fetch the file from a URL directly.
/// - `Bytes` — upload raw bytes with an explicit filename.
/// - `Path` — upload a file from the local filesystem.
#[derive(Debug, Clone, PartialEq)]
pub enum InputFile {
    /// An already-uploaded Telegram file referenced by its file identifier.
    FileId(String),

    /// A URL that Telegram will fetch on the sender's behalf.
    Url(String),

    /// Raw bytes to upload, with an explicit filename used for MIME-type inference.
    Bytes {
        /// Filename sent to Telegram (used for MIME inference).
        filename: String,
        /// Raw file content.
        data: Vec<u8>,
    },

    /// A path on the local filesystem to be read and uploaded.
    Path(PathBuf),
}

impl Serialize for InputFile {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            InputFile::FileId(id) => s.serialize_str(id),
            InputFile::Url(url) => s.serialize_str(url),
            InputFile::Bytes { filename, .. } => {
                s.serialize_str(&format!("attach://{filename}"))
            }
            InputFile::Path(p) => {
                s.serialize_str(&format!("attach://{}", p.display()))
            }
        }
    }
}

impl<'de> Deserialize<'de> for InputFile {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Ok(InputFile::FileId(s))
    }
}
