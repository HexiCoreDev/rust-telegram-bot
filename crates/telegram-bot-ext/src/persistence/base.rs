//! Base persistence trait and configuration types.
//!
//! Port of `telegram.ext._basepersistence.BasePersistence`.
//! Implementations must be `Send + Sync` so they can live behind an `Arc`.

use crate::utils::types::{CdcData, ConversationDict, ConversationKey, JsonMap};
use std::collections::HashMap;
use std::fmt;

// ---------------------------------------------------------------------------
// PersistenceInput — controls which data categories are persisted
// ---------------------------------------------------------------------------

/// Configuration for which data categories a persistence back-end should store.
///
/// All flags default to `true`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersistenceInput {
    pub bot_data: bool,
    pub chat_data: bool,
    pub user_data: bool,
    pub callback_data: bool,
}

impl Default for PersistenceInput {
    fn default() -> Self {
        Self {
            bot_data: true,
            chat_data: true,
            user_data: true,
            callback_data: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Error type for persistence operations
// ---------------------------------------------------------------------------

/// Errors that a persistence back-end may produce.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum PersistenceError {
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Custom(String),

    #[cfg(feature = "persistence-sqlite")]
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[cfg(feature = "persistence-redis")]
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[cfg(feature = "persistence-postgres")]
    #[error("PostgreSQL error: {0}")]
    Postgres(#[from] sqlx::Error),
}

pub type PersistenceResult<T> = Result<T, PersistenceError>;

/// Validate a table/key prefix supplied via `with_prefix`.
///
/// Prefixes are interpolated into SQL DDL (PostgreSQL) and key patterns
/// (Redis `SCAN MATCH`, where `*?[]` are glob metacharacters), so only
/// `^[A-Za-z_][A-Za-z0-9_]*$` is accepted. Callers that cannot fail (the
/// `with_prefix` setters) enforce the same rule with `debug_assert!`; fallible
/// constructors (`build` / `new`) return this error instead.
pub(crate) fn validate_table_prefix(prefix: &str) -> PersistenceResult<()> {
    let mut chars = prefix.chars();
    let valid = match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(PersistenceError::Custom(format!(
            "invalid table/key prefix {prefix:?}: must match ^[A-Za-z_][A-Za-z0-9_]*$"
        )))
    }
}

/// Restrict a persistence file to owner-only access.
///
/// Persistence files hold bot/user/chat data that may be sensitive. On Unix
/// this sets `0o600` so group/other users cannot read them regardless of the
/// process umask. On other platforms it is a no-op.
#[cfg(unix)]
pub(crate) fn restrict_file_permissions(path: &std::path::Path) -> PersistenceResult<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    Ok(())
}

/// Restrict a persistence file to owner-only access.
///
/// No-op on non-Unix platforms (see the Unix variant for details).
#[cfg(not(unix))]
pub(crate) fn restrict_file_permissions(_path: &std::path::Path) -> PersistenceResult<()> {
    Ok(())
}

// ---------------------------------------------------------------------------
// BasePersistence trait
// ---------------------------------------------------------------------------

/// The abstract persistence interface.
///
/// Implementations are accessed exclusively by the `Application`.
/// Calling methods manually may interfere with the application's own
/// persistence bookkeeping.
///
/// Uses native `async fn` in traits (stabilised in Rust 1.75).
pub trait BasePersistence: Send + Sync + fmt::Debug {
    // -- read -----------------------------------------------------------------

    /// Return all stored user data, keyed by user ID.
    fn get_user_data(
        &self,
    ) -> impl std::future::Future<Output = PersistenceResult<HashMap<i64, JsonMap>>> + Send;

    /// Return all stored chat data, keyed by chat ID.
    fn get_chat_data(
        &self,
    ) -> impl std::future::Future<Output = PersistenceResult<HashMap<i64, JsonMap>>> + Send;

    /// Return the stored bot-wide data.
    fn get_bot_data(&self) -> impl std::future::Future<Output = PersistenceResult<JsonMap>> + Send;

    /// Return the stored callback-data cache, if any.
    fn get_callback_data(
        &self,
    ) -> impl std::future::Future<Output = PersistenceResult<Option<CdcData>>> + Send;

    /// Return the conversation state map for the handler identified by `name`.
    fn get_conversations(
        &self,
        name: &str,
    ) -> impl std::future::Future<Output = PersistenceResult<ConversationDict>> + Send;

    // -- write ----------------------------------------------------------------

    /// Persist updated user data for a single user.
    fn update_user_data(
        &self,
        user_id: i64,
        data: &JsonMap,
    ) -> impl std::future::Future<Output = PersistenceResult<()>> + Send;

    /// Persist updated chat data for a single chat.
    fn update_chat_data(
        &self,
        chat_id: i64,
        data: &JsonMap,
    ) -> impl std::future::Future<Output = PersistenceResult<()>> + Send;

    /// Persist updated bot-wide data.
    fn update_bot_data(
        &self,
        data: &JsonMap,
    ) -> impl std::future::Future<Output = PersistenceResult<()>> + Send;

    /// Persist the callback-data cache.
    fn update_callback_data(
        &self,
        data: &CdcData,
    ) -> impl std::future::Future<Output = PersistenceResult<()>> + Send;

    /// Persist a single conversation state change.
    fn update_conversation(
        &self,
        name: &str,
        key: &ConversationKey,
        new_state: Option<&serde_json::Value>,
    ) -> impl std::future::Future<Output = PersistenceResult<()>> + Send;

    // -- delete ---------------------------------------------------------------

    /// Drop all data for a specific chat.
    fn drop_chat_data(
        &self,
        chat_id: i64,
    ) -> impl std::future::Future<Output = PersistenceResult<()>> + Send;

    /// Drop all data for a specific user.
    fn drop_user_data(
        &self,
        user_id: i64,
    ) -> impl std::future::Future<Output = PersistenceResult<()>> + Send;

    // -- refresh (optional hook) ----------------------------------------------

    /// Called before dispatching to give the back-end a chance to refresh
    /// user data from an external source. Default implementation is a no-op.
    fn refresh_user_data(
        &self,
        _user_id: i64,
        _user_data: &mut JsonMap,
    ) -> impl std::future::Future<Output = PersistenceResult<()>> + Send {
        async { Ok(()) }
    }

    /// Called before dispatching to refresh chat data. Default: no-op.
    fn refresh_chat_data(
        &self,
        _chat_id: i64,
        _chat_data: &mut JsonMap,
    ) -> impl std::future::Future<Output = PersistenceResult<()>> + Send {
        async { Ok(()) }
    }

    /// Called before dispatching to refresh bot data. Default: no-op.
    fn refresh_bot_data(
        &self,
        _bot_data: &mut JsonMap,
    ) -> impl std::future::Future<Output = PersistenceResult<()>> + Send {
        async { Ok(()) }
    }

    // -- lifecycle ------------------------------------------------------------

    /// Called by `Application::stop`. Gives the back-end a chance to flush
    /// pending writes or close a database connection.
    fn flush(&self) -> impl std::future::Future<Output = PersistenceResult<()>> + Send;

    /// The interval in seconds at which the `Application` should trigger
    /// a persistence update cycle. Defaults to 60 seconds.
    fn update_interval(&self) -> f64 {
        60.0
    }

    /// Which data categories this persistence stores.
    fn store_data(&self) -> PersistenceInput {
        PersistenceInput::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_prefixes_accepted() {
        for prefix in ["rtb", "mybot", "_x", "A1_", "a"] {
            assert!(
                validate_table_prefix(prefix).is_ok(),
                "{prefix} should be valid"
            );
        }
    }

    #[test]
    fn invalid_prefixes_rejected() {
        for prefix in [
            "",
            "has space",
            "has-dash",
            "has:colon",
            "1leading-digit",
            "semi;colon",
            "quote\"",
            "star*",
            "dot.name",
        ] {
            assert!(
                validate_table_prefix(prefix).is_err(),
                "{prefix:?} should be rejected"
            );
        }
    }
}
