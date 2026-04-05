//! Customisable context type factories.
//!
//! Ported from `python-telegram-bot/src/telegram/ext/_contexttypes.py`.
//!
//! [`ContextTypes`] lets users swap the concrete types used for `bot_data`, `chat_data`, and
//! `user_data` by supplying factory functions that produce the initial (empty) values.
//!
//! In Rust, Python's "pass a type that supports calling with zero args" maps to
//! `Fn() -> T` closures stored behind `Arc`.

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

/// Type alias for the default data stores -- `HashMap<String, Value>`.
pub type DefaultData = HashMap<String, Value>;

/// A factory function that produces a fresh data store instance.
///
/// Equivalent to Python's `type[dict]` default in `ContextTypes.__init__`.
pub type DataFactory<T> = Arc<dyn Fn() -> T + Send + Sync>;

/// Convenience struct that gathers the factory functions for the three per-scope data stores
/// and the context constructor strategy.
///
/// The generic defaults are `HashMap<String, Value>` for all three stores, matching the
/// Python library's `dict[Any, Any]` defaults.
///
/// # Type parameters
///
/// * `UD` -- user data store type (one per user id)
/// * `CD` -- chat data store type (one per chat id)
/// * `BD` -- bot data store type (single instance)
#[derive(Clone)]
pub struct ContextTypes<UD = DefaultData, CD = DefaultData, BD = DefaultData> {
    user_data_factory: DataFactory<UD>,
    chat_data_factory: DataFactory<CD>,
    bot_data_factory: DataFactory<BD>,
}

impl Default for ContextTypes<DefaultData, DefaultData, DefaultData> {
    fn default() -> Self {
        Self {
            user_data_factory: Arc::new(HashMap::<String, serde_json::Value>::new),
            chat_data_factory: Arc::new(HashMap::<String, serde_json::Value>::new),
            bot_data_factory: Arc::new(HashMap::<String, serde_json::Value>::new),
        }
    }
}

impl<UD, CD, BD> ContextTypes<UD, CD, BD>
where
    UD: Send + Sync + 'static,
    CD: Send + Sync + 'static,
    BD: Send + Sync + 'static,
{
    /// Creates a new `ContextTypes` with custom factory functions.
    #[must_use]
    pub fn new(
        user_data_factory: DataFactory<UD>,
        chat_data_factory: DataFactory<CD>,
        bot_data_factory: DataFactory<BD>,
    ) -> Self {
        Self {
            user_data_factory,
            chat_data_factory,
            bot_data_factory,
        }
    }

    /// Produces a fresh user-data store.
    #[must_use]
    pub fn user_data(&self) -> UD {
        (self.user_data_factory)()
    }

    /// Produces a fresh chat-data store.
    #[must_use]
    pub fn chat_data(&self) -> CD {
        (self.chat_data_factory)()
    }

    /// Produces a fresh bot-data store.
    #[must_use]
    pub fn bot_data(&self) -> BD {
        (self.bot_data_factory)()
    }

    /// Returns a reference to the user-data factory.
    #[must_use]
    pub fn user_data_factory(&self) -> &DataFactory<UD> {
        &self.user_data_factory
    }

    /// Returns a reference to the chat-data factory.
    #[must_use]
    pub fn chat_data_factory(&self) -> &DataFactory<CD> {
        &self.chat_data_factory
    }

    /// Returns a reference to the bot-data factory.
    #[must_use]
    pub fn bot_data_factory(&self) -> &DataFactory<BD> {
        &self.bot_data_factory
    }
}

impl<UD, CD, BD> std::fmt::Debug for ContextTypes<UD, CD, BD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextTypes")
            .field("user_data_factory", &"<fn>")
            .field("chat_data_factory", &"<fn>")
            .field("bot_data_factory", &"<fn>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_context_types_produce_empty_hashmaps() {
        let ct = ContextTypes::default();
        let ud: DefaultData = ct.user_data();
        let cd: DefaultData = ct.chat_data();
        let bd: DefaultData = ct.bot_data();

        assert!(ud.is_empty());
        assert!(cd.is_empty());
        assert!(bd.is_empty());
    }

    #[test]
    fn custom_context_types() {
        #[derive(Debug, Default)]
        struct MyUserData {
            score: i32,
        }

        let ct = ContextTypes::new(
            Arc::new(|| MyUserData { score: 100 }),
            Arc::new(HashMap::<String, serde_json::Value>::new),
            Arc::new(HashMap::<String, serde_json::Value>::new),
        );

        let ud = ct.user_data();
        assert_eq!(ud.score, 100);
    }

    #[test]
    fn factory_is_repeatable() {
        let ct = ContextTypes::default();
        let a: DefaultData = ct.user_data();
        let b: DefaultData = ct.user_data();
        // Each call produces a distinct empty map.
        assert!(a.is_empty());
        assert!(b.is_empty());
    }

    #[test]
    fn debug_impl() {
        let ct = ContextTypes::default();
        let s = format!("{ct:?}");
        assert!(s.contains("ContextTypes"));
    }
}
