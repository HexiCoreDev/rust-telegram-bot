//! Extended bot with convenience features.
//!
//! Ported from `python-telegram-bot/src/telegram/ext/_extbot.py`.
//!
//! [`ExtBot`] wraps the low-level [`rust_tg_bot_raw::bot::Bot`] and adds:
//!
//! * [`Defaults`](crate::defaults::Defaults) injection into API calls
//! * [`CallbackDataCache`](crate::callback_data_cache::CallbackDataCache) for arbitrary
//!   callback data
//! * A `rate_limiter` slot (placeholder -- the actual trait lives in `rate_limiter.rs`)
//!
//! # Construction
//!
//! Use [`ExtBot::builder`] for the full option set, or [`ExtBot::from_bot`] when you
//! only have a raw `Bot` and need no extras:
//!
//! ```rust,ignore
//! // Minimal:
//! let ext = ExtBot::from_bot(bot);
//!
//! // Full control:
//! let ext = ExtBot::builder("token", request)
//!     .defaults(defaults)
//!     .arbitrary_callback_data(512)
//!     .build();
//! ```
//!
//! # `Deref` to `Bot`
//!
//! `ExtBot` implements `Deref<Target = Bot>`, so all `Bot` methods are accessible
//! directly without calling `.inner()`:
//!
//! ```rust,ignore
//! // Instead of: ext_bot.inner().send_message(chat_id, text)
//! ext_bot.send_message(chat_id, text)
//! ```

use std::sync::Arc;

use tokio::sync::RwLock;

use rust_tg_bot_raw::bot::Bot;
use rust_tg_bot_raw::request::base::BaseRequest;

use crate::callback_data_cache::CallbackDataCache;
use crate::defaults::Defaults;

/// Extended bot that adds defaults, arbitrary callback data, and a rate-limiter slot on top
/// of the raw [`Bot`].
///
/// # Construction
///
/// Use [`ExtBot::builder`] for the full option set, or [`ExtBot::from_bot`] for the
/// simplest case (no defaults, no cache, no rate limiter).
///
/// # `Deref` to `Bot`
///
/// `ExtBot` implements [`Deref<Target = Bot>`](std::ops::Deref), making all `Bot` methods
/// accessible directly. This is a zero-cost abstraction -- no allocation or indirection
/// beyond what `Bot` already provides.
///
/// # Rate limiter
///
/// The `rate_limiter` field is an `Option<()>` placeholder.  The actual rate-limiter trait
/// and implementations live in [`crate::rate_limiter`] and will be wired by a separate agent.
pub struct ExtBot {
    /// The underlying raw bot.
    bot: Bot,

    /// User-defined defaults for API calls.
    defaults: Option<Defaults>,

    /// Cache for arbitrary inline keyboard callback data.
    callback_data_cache: Option<Arc<RwLock<CallbackDataCache>>>,

    /// Placeholder for the rate-limiter.  The actual trait is in `rate_limiter.rs`.
    rate_limiter: Option<()>,
}

// ---------------------------------------------------------------------------
// Deref<Target = Bot> -- zero-cost access to all Bot methods
// ---------------------------------------------------------------------------

impl std::ops::Deref for ExtBot {
    type Target = Bot;

    fn deref(&self) -> &Bot {
        &self.bot
    }
}

impl std::fmt::Debug for ExtBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtBot")
            .field("token", &self.bot.token())
            .field("defaults", &self.defaults)
            .field(
                "has_callback_data_cache",
                &self.callback_data_cache.is_some(),
            )
            .field("rate_limiter", &self.rate_limiter)
            .finish()
    }
}

impl ExtBot {
    /// Creates a new `ExtBot`.
    ///
    /// # Arguments
    ///
    /// * `bot` - The underlying raw bot.
    /// * `defaults` - Optional user-defined defaults for API calls.
    /// * `arbitrary_callback_data` - Pass `Some(maxsize)` to enable the callback data cache,
    ///   or `None` to disable it.  `Some(0)` uses the default maxsize of 1024.
    /// * `rate_limiter` - Placeholder.
    ///
    /// Prefer [`ExtBot::builder`] or [`ExtBot::from_bot`] for public construction.
    #[must_use]
    pub(crate) fn new(
        bot: Bot,
        defaults: Option<Defaults>,
        arbitrary_callback_data: Option<usize>,
        rate_limiter: Option<()>,
    ) -> Self {
        let callback_data_cache = arbitrary_callback_data.map(|maxsize| {
            let effective = if maxsize == 0 { 1024 } else { maxsize };
            Arc::new(RwLock::new(CallbackDataCache::new(effective)))
        });

        Self {
            bot,
            defaults,
            callback_data_cache,
            rate_limiter,
        }
    }

    /// Creates an `ExtBot` from a raw `Bot` with no extras.
    ///
    /// This is the simplest construction path -- no defaults, no callback data
    /// cache, and no rate limiter.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let bot = Bot::new("token", request);
    /// let ext = ExtBot::from_bot(bot);
    /// ```
    #[must_use]
    pub fn from_bot(bot: Bot) -> Self {
        Self::new(bot, None, None, None)
    }

    /// Returns a reference to the underlying raw bot.
    ///
    /// Note: With the `Deref<Target = Bot>` implementation, you can call `Bot`
    /// methods directly on `ExtBot` without using `.inner()`. This method is
    /// retained for backward compatibility.
    #[must_use]
    pub fn inner(&self) -> &Bot {
        &self.bot
    }

    /// Returns the bot token (delegates to the inner bot).
    #[must_use]
    pub fn token(&self) -> &str {
        self.bot.token()
    }

    /// Returns the user-defined defaults, if any.
    #[must_use]
    pub fn defaults(&self) -> Option<&Defaults> {
        self.defaults.as_ref()
    }

    /// Returns a reference to the callback data cache, if enabled.
    #[must_use]
    pub fn callback_data_cache(&self) -> Option<&Arc<RwLock<CallbackDataCache>>> {
        self.callback_data_cache.as_ref()
    }

    /// Returns `true` if arbitrary callback data is enabled.
    #[must_use]
    pub fn has_callback_data_cache(&self) -> bool {
        self.callback_data_cache.is_some()
    }

    /// Returns the rate-limiter placeholder.
    #[must_use]
    pub fn rate_limiter(&self) -> Option<()> {
        self.rate_limiter
    }

    /// Convenience builder entry point.
    #[must_use]
    pub fn builder(token: impl Into<String>, request: Arc<dyn BaseRequest>) -> ExtBotBuilder {
        ExtBotBuilder::new(token, request)
    }

    /// Initializes the bot.
    ///
    /// Currently a no-op.  If a rate-limiter is present it would be initialized here.
    pub async fn initialize(&self) -> rust_tg_bot_raw::error::Result<()> {
        Ok(())
    }

    /// Shuts down the bot.
    pub async fn shutdown(&self) -> rust_tg_bot_raw::error::Result<()> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// ExtBotBuilder
// ---------------------------------------------------------------------------

/// Builder for [`ExtBot`].
///
/// # Example
///
/// ```rust,ignore
/// let ext = ExtBot::builder("my_token", request)
///     .defaults(defaults)
///     .arbitrary_callback_data(256)
///     .build();
/// ```
pub struct ExtBotBuilder {
    token: String,
    request: Arc<dyn BaseRequest>,
    base_url: Option<String>,
    base_file_url: Option<String>,
    defaults: Option<Defaults>,
    arbitrary_callback_data: Option<usize>,
    rate_limiter: Option<()>,
}

impl ExtBotBuilder {
    /// Creates a new builder with the required token and HTTP request backend.
    #[must_use]
    pub fn new(token: impl Into<String>, request: Arc<dyn BaseRequest>) -> Self {
        Self {
            token: token.into(),
            request,
            base_url: None,
            base_file_url: None,
            defaults: None,
            arbitrary_callback_data: None,
            rate_limiter: None,
        }
    }

    /// Sets a custom base URL (e.g. for a local Bot API server).
    #[must_use]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Sets a custom base file URL.
    #[must_use]
    pub fn base_file_url(mut self, url: impl Into<String>) -> Self {
        self.base_file_url = Some(url.into());
        self
    }

    /// Sets the user-defined defaults.
    #[must_use]
    pub fn defaults(mut self, defaults: Defaults) -> Self {
        self.defaults = Some(defaults);
        self
    }

    /// Enables arbitrary callback data with the given cache size.
    ///
    /// Pass `0` to use the default maxsize of 1024.
    #[must_use]
    pub fn arbitrary_callback_data(mut self, maxsize: usize) -> Self {
        self.arbitrary_callback_data = Some(maxsize);
        self
    }

    /// Sets the rate-limiter placeholder.
    #[must_use]
    pub fn rate_limiter(mut self, rl: ()) -> Self {
        self.rate_limiter = Some(rl);
        self
    }

    /// Builds the [`ExtBot`].
    #[must_use]
    pub fn build(self) -> ExtBot {
        let bot = Bot::new(&self.token, self.request);

        ExtBot::new(
            bot,
            self.defaults,
            self.arbitrary_callback_data,
            self.rate_limiter,
        )
    }
}

impl std::fmt::Debug for ExtBotBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtBotBuilder")
            .field("token", &"[REDACTED]")
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Test-only mock request
// ---------------------------------------------------------------------------

/// A minimal mock [`BaseRequest`] for use in tests throughout the ext crate.
///
/// Returns `{"ok": true, "result": []}` for every request.
#[cfg(test)]
pub(crate) mod test_support {
    use std::time::Duration;

    use rust_tg_bot_raw::request::base::{HttpMethod, TimeoutOverride};
    use rust_tg_bot_raw::request::request_data::RequestData;

    use super::*;

    #[derive(Debug)]
    pub struct MockRequest;

    #[async_trait::async_trait]
    impl BaseRequest for MockRequest {
        async fn initialize(&self) -> rust_tg_bot_raw::error::Result<()> {
            Ok(())
        }

        async fn shutdown(&self) -> rust_tg_bot_raw::error::Result<()> {
            Ok(())
        }

        fn default_read_timeout(&self) -> Option<Duration> {
            Some(Duration::from_secs(5))
        }

        async fn do_request(
            &self,
            _url: &str,
            _method: HttpMethod,
            _request_data: Option<&RequestData>,
            _timeouts: TimeoutOverride,
        ) -> rust_tg_bot_raw::error::Result<(u16, bytes::Bytes)> {
            let body = br#"{"ok":true,"result":[]}"#;
            Ok((200, bytes::Bytes::from_static(body)))
        }

        async fn do_request_json_bytes(
            &self,
            _url: &str,
            _body: &[u8],
            _timeouts: TimeoutOverride,
        ) -> rust_tg_bot_raw::error::Result<(u16, bytes::Bytes)> {
            let body = br#"{"ok":true,"result":[]}"#;
            Ok((200, bytes::Bytes::from_static(body)))
        }
    }

    pub fn mock_request() -> Arc<dyn BaseRequest> {
        Arc::new(MockRequest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::*;

    #[test]
    fn ext_bot_creation() {
        let bot = Bot::new("test_token", mock_request());
        let ext = ExtBot::from_bot(bot);

        assert_eq!(ext.token(), "test_token");
        assert!(ext.defaults().is_none());
        assert!(!ext.has_callback_data_cache());
        assert!(ext.rate_limiter().is_none());
    }

    #[test]
    fn ext_bot_with_callback_cache() {
        let bot = Bot::new("token", mock_request());
        let ext = ExtBot::new(bot, None, Some(512), None);

        assert!(ext.has_callback_data_cache());
    }

    #[test]
    fn ext_bot_with_defaults() {
        let defaults = Defaults::builder().parse_mode("HTML").build();
        let bot = Bot::new("token", mock_request());
        let ext = ExtBot::new(bot, Some(defaults), None, None);

        assert_eq!(ext.defaults().unwrap().parse_mode(), Some("HTML"));
    }

    #[test]
    fn ext_bot_builder() {
        let ext = ExtBot::builder("my_token", mock_request())
            .arbitrary_callback_data(256)
            .build();

        assert_eq!(ext.token(), "my_token");
        assert!(ext.has_callback_data_cache());
    }

    #[tokio::test]
    async fn ext_bot_lifecycle() {
        let bot = Bot::new("token", mock_request());
        let ext = ExtBot::from_bot(bot);
        assert!(ext.initialize().await.is_ok());
        assert!(ext.shutdown().await.is_ok());
    }

    #[test]
    fn ext_bot_debug() {
        let bot = Bot::new("token", mock_request());
        let ext = ExtBot::from_bot(bot);
        let s = format!("{ext:?}");
        assert!(s.contains("ExtBot"));
        assert!(s.contains("token"));
    }

    #[test]
    fn ext_bot_from_bot_convenience() {
        let bot = Bot::new("tk", mock_request());
        let ext = ExtBot::from_bot(bot);
        assert_eq!(ext.token(), "tk");
        assert!(ext.defaults().is_none());
        assert!(!ext.has_callback_data_cache());
        assert!(ext.rate_limiter().is_none());
    }

    #[test]
    fn ext_bot_deref_provides_bot_methods() {
        let bot = Bot::new("deref_token", mock_request());
        let ext = ExtBot::from_bot(bot);

        // token() is available on Bot via Deref (same as ext.inner().token())
        let deref_token: &str = (*ext).token();
        assert_eq!(deref_token, "deref_token");
        assert_eq!(ext.token(), deref_token);
    }
}
