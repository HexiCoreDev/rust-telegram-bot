use std::time::Duration;

/// Base error type for all Telegram Bot API errors.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TelegramError {
    /// The bot doesn't have enough rights to perform the requested action.
    #[error("{0}")]
    Forbidden(String),

    /// The provided token is invalid.
    #[error("{0}")]
    InvalidToken(String),

    /// The requested API endpoint was not found.
    #[error("{0}")]
    EndPointNotFound(String),

    /// A network-level error occurred.
    #[error("{0}")]
    Network(String),

    /// Telegram returned a 400 Bad Request response.
    #[error("{0}")]
    BadRequest(String),

    /// The request timed out.
    #[error("{0}")]
    TimedOut(String),

    /// The chat was migrated to a supergroup with a new ID.
    #[error("Group migrated to supergroup. New chat id: {new_chat_id}")]
    ChatMigrated {
        /// The new chat ID of the supergroup.
        new_chat_id: i64,
    },

    /// Flood control -- must wait before retrying.
    #[error("Flood control exceeded. Retry in {retry_after:?}")]
    RetryAfter {
        /// Duration to wait before retrying the request.
        retry_after: Duration,
    },

    /// A long-poll or webhook conflicts with another one.
    #[error("{0}")]
    Conflict(String),

    /// Passport decryption failed.
    #[error("PassportDecryptionError: {0}")]
    PassportDecryption(String),

    /// HTTP-level error from reqwest.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// I/O error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl TelegramError {
    /// Cleans the error prefix from API error messages, matching python-telegram-bot behavior.
    pub fn from_api_message(message: &str) -> String {
        let msg = message
            .strip_prefix("Error: ")
            .or_else(|| message.strip_prefix("[Error]: "))
            .or_else(|| message.strip_prefix("Bad Request: "));

        match msg {
            Some(stripped) => {
                let mut chars = stripped.chars();
                match chars.next() {
                    Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                    None => String::new(),
                }
            }
            None => message.to_string(),
        }
    }

    /// Attach the calling API method name to this error, if it carries a
    /// free-form message.
    ///
    /// String-payload variants (`Forbidden`, `BadRequest`, …) gain a
    /// `"[method] "` prefix so logs and user-facing errors identify the
    /// failing call. Structured variants (`ChatMigrated`, `RetryAfter`,
    /// `Http`, `Json`, `Io`) already carry their own context and are
    /// returned unchanged.
    #[must_use]
    pub fn with_method(self, method: &str) -> Self {
        let prefix = format!("[{method}] ");
        match self {
            Self::Forbidden(m) => Self::Forbidden(prefix + &m),
            Self::InvalidToken(m) => Self::InvalidToken(prefix + &m),
            Self::EndPointNotFound(m) => Self::EndPointNotFound(prefix + &m),
            Self::Network(m) => Self::Network(prefix + &m),
            Self::BadRequest(m) => Self::BadRequest(prefix + &m),
            Self::TimedOut(m) => Self::TimedOut(prefix + &m),
            Self::Conflict(m) => Self::Conflict(prefix + &m),
            Self::PassportDecryption(m) => Self::PassportDecryption(prefix + &m),
            other => other,
        }
    }
}

/// A type alias for `Result<T, TelegramError>`.
pub type Result<T> = std::result::Result<T, TelegramError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_method_prefixes_string_variants() {
        let err = TelegramError::BadRequest("chat not found".to_owned()).with_method("sendMessage");
        assert_eq!(format!("{err}"), "[sendMessage] chat not found");
    }

    #[test]
    fn with_method_leaves_structured_variants_untouched() {
        let err = TelegramError::ChatMigrated { new_chat_id: 42 }.with_method("sendMessage");
        assert!(matches!(
            err,
            TelegramError::ChatMigrated { new_chat_id: 42 }
        ));
    }
}
