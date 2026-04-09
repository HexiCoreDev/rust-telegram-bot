//! Default parameter values for bot methods.
//!
//! Ported from `python-telegram-bot/src/telegram/ext/_defaults.py`.
//!
//! [`Defaults`] gathers all user-defined default values that the [`ExtBot`](super::ext_bot::ExtBot)
//! and handlers consult when an explicit value is not provided by the caller.
//!
//! Once constructed, every field is immutable (no public setters).
//!
//! # Construction
//!
//! Use the builder pattern via [`Defaults::builder()`]:
//!
//! ```
//! # use rust_tg_bot_ext::defaults::Defaults;
//! let defaults = Defaults::builder()
//!     .parse_mode("HTML")
//!     .disable_notification(true)
//!     .build();
//!
//! assert_eq!(defaults.parse_mode(), Some("HTML"));
//! ```

use std::collections::HashMap;

use serde_json::Value;

use rust_tg_bot_raw::types::link_preview_options::LinkPreviewOptions;

/// Convenience struct to gather all parameters with a (user defined) default value.
///
/// Fields that are `None` indicate "no default was set" and will not be injected into API
/// calls.
#[derive(Debug, Clone)]
pub struct Defaults {
    parse_mode: Option<String>,
    disable_notification: Option<bool>,
    allow_sending_without_reply: Option<bool>,
    protect_content: Option<bool>,
    block: bool,
    link_preview_options: Option<LinkPreviewOptions>,
    do_quote: Option<bool>,
    /// Pre-computed map of non-`None` defaults keyed by the API parameter name.  Used by
    /// `ExtBot` to merge defaults into outgoing requests.
    api_defaults: HashMap<String, Value>,
}

impl Defaults {
    /// Creates a new `Defaults` instance.
    ///
    /// Only the values that are explicitly `Some(...)` will be injected into API calls.
    /// `block` defaults to `true` when `None` is passed.
    ///
    /// Prefer [`Defaults::builder()`] for public construction -- this avoids long
    /// `None, None, None` argument lists.
    #[must_use]
    pub(crate) fn new(
        parse_mode: Option<String>,
        disable_notification: Option<bool>,
        allow_sending_without_reply: Option<bool>,
        protect_content: Option<bool>,
        block: Option<bool>,
        link_preview_options: Option<LinkPreviewOptions>,
        do_quote: Option<bool>,
    ) -> Self {
        let block = block.unwrap_or(true);

        let mut api_defaults = HashMap::new();

        if let Some(ref pm) = parse_mode {
            let v = Value::String(pm.clone());
            api_defaults.insert("parse_mode".into(), v.clone());
            api_defaults.insert("explanation_parse_mode".into(), v.clone());
            api_defaults.insert("text_parse_mode".into(), v.clone());
            api_defaults.insert("question_parse_mode".into(), v);
        }
        if let Some(dn) = disable_notification {
            api_defaults.insert("disable_notification".into(), Value::Bool(dn));
        }
        if let Some(aswr) = allow_sending_without_reply {
            api_defaults.insert("allow_sending_without_reply".into(), Value::Bool(aswr));
        }
        if let Some(pc) = protect_content {
            api_defaults.insert("protect_content".into(), Value::Bool(pc));
        }
        if let Some(dq) = do_quote {
            api_defaults.insert("do_quote".into(), Value::Bool(dq));
        }
        if let Some(ref lpo) = link_preview_options {
            if let Ok(v) = serde_json::to_value(lpo) {
                api_defaults.insert("link_preview_options".into(), v);
            }
        }

        Self {
            parse_mode,
            disable_notification,
            allow_sending_without_reply,
            protect_content,
            block,
            link_preview_options,
            do_quote,
            api_defaults,
        }
    }

    /// Returns a new [`DefaultsBuilder`] for ergonomic construction.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_tg_bot_ext::defaults::Defaults;
    /// let defaults = Defaults::builder()
    ///     .parse_mode("HTML")
    ///     .protect_content(true)
    ///     .build();
    /// ```
    #[must_use]
    pub fn builder() -> DefaultsBuilder {
        DefaultsBuilder::new()
    }

    // -- Read-only accessors (mirrors Python @property with no setter) --

    /// Send Markdown or HTML -- if you want Telegram apps to show bold, italic, fixed-width text
    /// or URLs in your bot's message.
    #[must_use]
    pub fn parse_mode(&self) -> Option<&str> {
        self.parse_mode.as_deref()
    }

    /// Alias for [`parse_mode`](Self::parse_mode), used for the corresponding parameter of
    /// `Bot::send_poll`.
    #[must_use]
    pub fn explanation_parse_mode(&self) -> Option<&str> {
        self.parse_mode.as_deref()
    }

    /// Alias for [`parse_mode`](Self::parse_mode), used for `InputPollOption` and
    /// `Bot::send_gift`.
    #[must_use]
    pub fn text_parse_mode(&self) -> Option<&str> {
        self.parse_mode.as_deref()
    }

    /// Alias for [`parse_mode`](Self::parse_mode), used for `Bot::send_poll`.
    #[must_use]
    pub fn question_parse_mode(&self) -> Option<&str> {
        self.parse_mode.as_deref()
    }

    /// Alias for [`parse_mode`](Self::parse_mode), used for `ReplyParameters`.
    #[must_use]
    pub fn quote_parse_mode(&self) -> Option<&str> {
        self.parse_mode.as_deref()
    }

    /// Sends the message silently.
    #[must_use]
    pub fn disable_notification(&self) -> Option<bool> {
        self.disable_notification
    }

    /// Pass `true` if the message should be sent even if the specified replied-to message is not
    /// found.
    #[must_use]
    pub fn allow_sending_without_reply(&self) -> Option<bool> {
        self.allow_sending_without_reply
    }

    /// Default setting for `BaseHandler.block` and error handlers.
    #[must_use]
    pub fn block(&self) -> bool {
        self.block
    }

    /// Protects the contents of the sent message from forwarding and saving.
    #[must_use]
    pub fn protect_content(&self) -> Option<bool> {
        self.protect_content
    }

    /// Link preview generation options for all outgoing messages.
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&LinkPreviewOptions> {
        self.link_preview_options.as_ref()
    }

    /// Whether the bot should quote the replied-to message.
    #[must_use]
    pub fn do_quote(&self) -> Option<bool> {
        self.do_quote
    }

    /// Pre-computed mapping of non-`None` defaults keyed by API parameter name.
    #[must_use]
    pub fn api_defaults(&self) -> &HashMap<String, Value> {
        &self.api_defaults
    }
}

impl PartialEq for Defaults {
    fn eq(&self, other: &Self) -> bool {
        self.parse_mode == other.parse_mode
            && self.disable_notification == other.disable_notification
            && self.allow_sending_without_reply == other.allow_sending_without_reply
            && self.protect_content == other.protect_content
            && self.block == other.block
            && self.link_preview_options == other.link_preview_options
            && self.do_quote == other.do_quote
    }
}

impl Eq for Defaults {}

impl std::hash::Hash for Defaults {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parse_mode.hash(state);
        self.disable_notification.hash(state);
        self.allow_sending_without_reply.hash(state);
        self.protect_content.hash(state);
        self.block.hash(state);
        self.do_quote.hash(state);
        // LinkPreviewOptions does not implement Hash, so we hash its JSON representation.
        if let Some(ref lpo) = self.link_preview_options {
            if let Ok(v) = serde_json::to_string(lpo) {
                v.hash(state);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// DefaultsBuilder
// ---------------------------------------------------------------------------

/// Builder for [`Defaults`].
///
/// Provides ergonomic construction without `None, None, None` argument lists.
///
/// # Example
///
/// ```
/// # use rust_tg_bot_ext::defaults::Defaults;
/// let defaults = Defaults::builder()
///     .parse_mode("HTML")
///     .disable_notification(true)
///     .do_quote(true)
///     .build();
///
/// assert_eq!(defaults.parse_mode(), Some("HTML"));
/// assert_eq!(defaults.disable_notification(), Some(true));
/// assert!(defaults.block()); // defaults to true
/// ```
#[derive(Debug)]
pub struct DefaultsBuilder {
    parse_mode: Option<String>,
    disable_notification: Option<bool>,
    allow_sending_without_reply: Option<bool>,
    protect_content: Option<bool>,
    block: Option<bool>,
    link_preview_options: Option<LinkPreviewOptions>,
    do_quote: Option<bool>,
}

impl Default for DefaultsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultsBuilder {
    /// Creates a new builder with all fields unset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            parse_mode: None,
            disable_notification: None,
            allow_sending_without_reply: None,
            protect_content: None,
            block: None,
            link_preview_options: None,
            do_quote: None,
        }
    }

    /// Sets the default parse mode (e.g. `"HTML"`, `"MarkdownV2"`).
    #[must_use]
    pub fn parse_mode(mut self, mode: impl Into<String>) -> Self {
        self.parse_mode = Some(mode.into());
        self
    }

    /// Sets whether messages are sent silently by default.
    #[must_use]
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets whether messages should be sent even if the replied-to message is not found.
    #[must_use]
    pub fn allow_sending_without_reply(mut self, value: bool) -> Self {
        self.allow_sending_without_reply = Some(value);
        self
    }

    /// Sets whether message contents are protected from forwarding and saving.
    #[must_use]
    pub fn protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }

    /// Sets the default `block` value for handlers. Defaults to `true` if not set.
    #[must_use]
    pub fn block(mut self, value: bool) -> Self {
        self.block = Some(value);
        self
    }

    /// Sets the default link preview options.
    #[must_use]
    pub fn link_preview_options(mut self, options: LinkPreviewOptions) -> Self {
        self.link_preview_options = Some(options);
        self
    }

    /// Sets whether the bot should quote the replied-to message.
    #[must_use]
    pub fn do_quote(mut self, value: bool) -> Self {
        self.do_quote = Some(value);
        self
    }

    /// Builds the [`Defaults`] instance.
    #[must_use]
    pub fn build(self) -> Defaults {
        Defaults::new(
            self.parse_mode,
            self.disable_notification,
            self.allow_sending_without_reply,
            self.protect_content,
            self.block,
            self.link_preview_options,
            self.do_quote,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_immutable_accessors() {
        let d = Defaults::builder()
            .parse_mode("HTML")
            .disable_notification(true)
            .allow_sending_without_reply(false)
            .do_quote(true)
            .build();

        assert_eq!(d.parse_mode(), Some("HTML"));
        assert_eq!(d.explanation_parse_mode(), Some("HTML"));
        assert_eq!(d.text_parse_mode(), Some("HTML"));
        assert_eq!(d.question_parse_mode(), Some("HTML"));
        assert_eq!(d.disable_notification(), Some(true));
        assert_eq!(d.allow_sending_without_reply(), Some(false));
        assert!(d.block());
        assert_eq!(d.protect_content(), None);
        assert_eq!(d.do_quote(), Some(true));
    }

    #[test]
    fn defaults_api_defaults_map() {
        let d = Defaults::builder()
            .parse_mode("MarkdownV2")
            .protect_content(true)
            .build();

        let m = d.api_defaults();
        assert!(m.contains_key("parse_mode"));
        assert!(m.contains_key("explanation_parse_mode"));
        assert!(m.contains_key("protect_content"));
        assert!(!m.contains_key("disable_notification"));
    }

    #[test]
    fn defaults_equality() {
        let a = Defaults::builder().parse_mode("HTML").build();
        let b = Defaults::builder().parse_mode("HTML").build();
        let c = Defaults::builder().build();
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn block_defaults_to_true() {
        let d = Defaults::builder().build();
        assert!(d.block());
    }

    #[test]
    fn block_can_be_set_to_false() {
        let d = Defaults::builder().block(false).build();
        assert!(!d.block());
    }

    #[test]
    fn builder_default_trait() {
        let b = DefaultsBuilder::default();
        let d = b.build();
        assert!(d.parse_mode().is_none());
        assert!(d.block());
    }
}
