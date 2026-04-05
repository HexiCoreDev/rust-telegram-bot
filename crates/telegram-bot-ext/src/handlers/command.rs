//! [`CommandHandler`] -- handles Telegram bot commands (`/command`).
//!
//! Ported from `python-telegram-bot`'s `CommandHandler`. Matches messages
//! whose first entity is `BOT_COMMAND` at offset 0, validates the command
//! against a set of known commands, and extracts arguments.
//!
//! ## Key behaviours matching the Python implementation
//!
//! - **C1 -- `@botname` validation**: When the command contains `@username`,
//!   the suffix is validated against `bot_username` (case-insensitive). If
//!   `bot_username` is `None` the `@` part is silently stripped (backwards
//!   compatible).
//! - **C2 -- Filter integration**: An optional `filter_fn` runs *before*
//!   command matching. The default filter accepts updates that have a
//!   `message` **or** `edited_message` field, matching Python's
//!   `filters.UpdateType.MESSAGES` default.

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use regex::Regex;
use telegram_bot_raw::types::update::Update;
use telegram_bot_raw::constants::MessageEntityType;

use crate::context::CallbackContext;
use super::base::{ContextCallback, Handler, HandlerCallback, HandlerResult, MatchResult};

/// Specifies how many arguments a command must have to match.
#[derive(Debug, Clone)]
pub enum HasArgs {
    /// Accept any number of arguments (including zero).
    Any,
    /// Require at least one argument.
    NonEmpty,
    /// Require zero arguments.
    None,
    /// Require exactly this many arguments.
    Exact(usize),
}

/// Type alias for the optional update filter closure.
///
/// Returns `true` if the update should be considered, `false` to reject it
/// before command matching even runs.
pub type UpdateFilter = Arc<dyn Fn(&Update) -> bool + Send + Sync>;

/// Handler for Telegram bot commands (messages starting with `/`).
///
/// The handler will only trigger on messages where the first entity is a
/// `bot_command` at offset 0. It validates the command text against the
/// provided set of commands (case-insensitive) and optionally checks the
/// argument count.
///
/// # Ergonomic constructor
///
/// ```rust,ignore
/// use telegram_bot_ext::prelude::*;
///
/// async fn start(update: Update, context: Context) -> HandlerResult {
///     context.reply_text(&update, "Hello!").await?;
///     Ok(())
/// }
///
/// CommandHandler::new("start", start);
/// ```
///
/// # Full-control constructor
///
/// ```rust,ignore
/// use telegram_bot_ext::handlers::command::CommandHandler;
/// use telegram_bot_ext::handlers::base::*;
/// use std::sync::Arc;
///
/// let handler = CommandHandler::with_options(
///     vec!["start".into(), "help".into()],
///     Arc::new(|update, match_result| Box::pin(async move {
///         HandlerResult::Continue
///     })),
///     None, // has_args
///     true, // block
/// );
/// ```
pub struct CommandHandler {
    /// Lowercased set of commands this handler responds to (without `/`).
    commands: HashSet<String>,
    callback: HandlerCallback,
    has_args: HasArgs,
    block: bool,
    /// C1: Optional bot username for `@botname` validation (stored lowercased).
    bot_username: Option<String>,
    /// C2: Optional filter applied before command matching. When `None` the
    /// default behaviour is to require that the update has a `message` or
    /// `edited_message` field, matching Python's `UpdateType.MESSAGES`.
    filter_fn: Option<UpdateFilter>,
    /// Optional context-aware callback for the ergonomic API.
    context_callback: Option<ContextCallback>,
}

/// Validation regex: commands must be 1-32 chars of `[a-z0-9_]`.
fn validate_command(cmd: &str) -> bool {
    lazy_static_regex().is_match(cmd)
}

fn lazy_static_regex() -> &'static Regex {
    use std::sync::OnceLock;
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[a-z0-9_]{1,32}$").expect("command regex is valid"))
}

/// The default update filter: accepts updates with a `message` **or**
/// `edited_message` field.
///
/// This mirrors Python's `filters.UpdateType.MESSAGES` default which accepts
/// both `message` and `edited_message`.
fn default_update_filter(update: &Update) -> bool {
    update.message.is_some() || update.edited_message.is_some()
}

impl CommandHandler {
    /// Ergonomic constructor matching python-telegram-bot's
    /// `CommandHandler("cmd", callback)`.
    ///
    /// Accepts a single command name (string) and an async handler function
    /// with signature `async fn(Update, Context) -> HandlerResult`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use telegram_bot_ext::prelude::*;
    ///
    /// async fn start(update: Update, context: Context) -> HandlerResult {
    ///     context.reply_text(&update, "Hello!").await?;
    ///     Ok(())
    /// }
    ///
    /// CommandHandler::new("start", start);
    /// ```
    pub fn new<Cb, Fut>(command: impl Into<String>, callback: Cb) -> Self
    where
        Cb: Fn(Update, CallbackContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), crate::application::HandlerError>> + Send + 'static,
    {
        let cmd = command.into();
        let cb = Arc::new(callback);
        let context_cb: ContextCallback = Arc::new(move |update, ctx| {
            let fut = cb(update, ctx);
            Box::pin(fut) as Pin<Box<dyn Future<Output = Result<(), crate::application::HandlerError>> + Send>>
        });

        // The raw callback is a no-op; handle_update_with_context is used instead.
        let noop_callback: HandlerCallback = Arc::new(|_update, _mr| {
            Box::pin(async { HandlerResult::Continue })
        });

        let commands: HashSet<String> = {
            let lower = cmd.to_lowercase();
            assert!(
                validate_command(&lower),
                "Command `{lower}` is not a valid bot command"
            );
            let mut set = HashSet::new();
            set.insert(lower);
            set
        };

        Self {
            commands,
            callback: noop_callback,
            has_args: HasArgs::Any,
            block: true,
            bot_username: None,
            filter_fn: None,
            context_callback: Some(context_cb),
        }
    }

    /// Full-control constructor for advanced use cases.
    ///
    /// # Panics
    ///
    /// Panics if any command string does not match `[a-z0-9_]{1,32}`.
    pub fn with_options(
        commands: Vec<String>,
        callback: HandlerCallback,
        has_args: Option<HasArgs>,
        block: bool,
    ) -> Self {
        let commands: HashSet<String> = commands.into_iter().map(|c| c.to_lowercase()).collect();
        for cmd in &commands {
            assert!(
                validate_command(cmd),
                "Command `{cmd}` is not a valid bot command"
            );
        }
        Self {
            commands,
            callback,
            has_args: has_args.unwrap_or(HasArgs::Any),
            block,
            bot_username: None,
            filter_fn: None,
            context_callback: None,
        }
    }

    /// Set the bot username for `@botname` validation (C1).
    ///
    /// When a command like `/start@MyBot` is received, the `@MyBot` suffix
    /// will be compared case-insensitively against this value. If they do not
    /// match the update is rejected.
    ///
    /// If no bot username is configured, the `@` suffix is silently ignored
    /// (backwards compatible).
    pub fn with_bot_username(mut self, username: impl Into<String>) -> Self {
        self.bot_username = Some(username.into().to_lowercase());
        self
    }

    /// Set a custom update filter (C2).
    ///
    /// The filter runs *before* any command matching. If it returns `false`
    /// the update is immediately rejected.
    ///
    /// When no custom filter is supplied the default behaviour is to require
    /// `update.message` or `update.edited_message` to be `Some`, matching
    /// Python's `UpdateType.MESSAGES`.
    pub fn with_filter(mut self, filter: UpdateFilter) -> Self {
        self.filter_fn = Some(filter);
        self
    }

    /// Check whether the argument count satisfies the `has_args` constraint.
    fn check_args(&self, args: &[String]) -> bool {
        match &self.has_args {
            HasArgs::Any => true,
            HasArgs::NonEmpty => !args.is_empty(),
            HasArgs::None => args.is_empty(),
            HasArgs::Exact(n) => args.len() == *n,
        }
    }
}

impl Handler for CommandHandler {
    fn check_update(&self, update: &Update) -> Option<MatchResult> {
        // -- C2: Apply filter first -----------------------------------------------
        let passes_filter = match &self.filter_fn {
            Some(f) => f(update),
            None => default_update_filter(update),
        };
        if !passes_filter {
            return None;
        }

        let message = update.effective_message()?;
        let text = message.text.as_ref()?;
        let entities = message.entities.as_ref()?;

        // First entity must be a bot_command at offset 0.
        let first_entity = entities.first()?;
        if first_entity.entity_type != MessageEntityType::BotCommand {
            return None;
        }
        if first_entity.offset != 0 {
            return None;
        }
        let length = first_entity.length as usize;

        // Extract command (strip leading `/`) and optional `@botname`.
        let raw_command = &text[1..length];
        let command_parts: Vec<&str> = raw_command.splitn(2, '@').collect();
        let command_name = command_parts[0];

        // -- C1: Validate @botname suffix -----------------------------------------
        if command_parts.len() > 1 {
            // Command has an `@suffix`.
            let at_suffix = command_parts[1];
            if let Some(ref expected) = self.bot_username {
                if at_suffix.to_lowercase() != *expected {
                    return None;
                }
            }
            // When bot_username is None we silently strip the suffix
            // (backwards compatible).
        }
        // When there is no @suffix the command is accepted regardless of
        // bot_username (matches Python behaviour where the bot appends its
        // own username and the comparison trivially passes).

        if !self.commands.contains(&command_name.to_lowercase()) {
            return None;
        }

        // Extract arguments: everything after the command, split on whitespace.
        let args: Vec<String> = text
            .split_whitespace()
            .skip(1)
            .map(String::from)
            .collect();

        if !self.check_args(&args) {
            return None;
        }

        Some(MatchResult::Args(args))
    }

    fn handle_update(
        &self,
        update: Update,
        match_result: MatchResult,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        (self.callback)(update, match_result)
    }

    fn block(&self) -> bool {
        self.block
    }

    /// Merge command arguments into `context.args`.
    ///
    /// Mirrors Python's `CommandHandler.collect_additional_context` which
    /// populates `context.args` from the parsed argument list produced by
    /// [`check_update`](Handler::check_update).
    fn collect_additional_context(
        &self,
        context: &mut CallbackContext,
        match_result: &MatchResult,
    ) {
        if let MatchResult::Args(args) = match_result {
            context.args = Some(args.clone());
        }
    }

    fn handle_update_with_context(
        &self,
        update: Update,
        match_result: MatchResult,
        context: CallbackContext,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        if let Some(ref cb) = self.context_callback {
            let fut = cb(update, context);
            Box::pin(async move {
                match fut.await {
                    Ok(()) => HandlerResult::Continue,
                    Err(crate::application::HandlerError::HandlerStop { .. }) => HandlerResult::Stop,
                    Err(crate::application::HandlerError::Other(e)) => HandlerResult::Error(e),
                }
            })
        } else {
            (self.callback)(update, match_result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::Arc;

    fn noop_callback() -> HandlerCallback {
        Arc::new(|_update, _mr| Box::pin(async { HandlerResult::Continue }))
    }

    /// Build a minimal `Update` with a command message via JSON deserialization.
    fn make_command_update(text: &str) -> Update {
        let cmd_part = text.split_whitespace().next().unwrap_or(text);
        let entity_len = cmd_part.len();
        serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": text,
                "entities": [{"type": "bot_command", "offset": 0, "length": entity_len}]
            }
        }))
        .expect("test update JSON must be valid")
    }

    /// Build an `Update` with an `edited_message` containing a command.
    fn make_edited_command_update(text: &str) -> Update {
        let cmd_part = text.split_whitespace().next().unwrap_or(text);
        let entity_len = cmd_part.len();
        serde_json::from_value(json!({
            "update_id": 1,
            "edited_message": {
                "message_id": 1,
                "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": text,
                "entities": [{"type": "bot_command", "offset": 0, "length": entity_len}]
            }
        }))
        .expect("test update JSON must be valid")
    }

    #[test]
    fn valid_commands_accepted() {
        let h = CommandHandler::with_options(
            vec!["start".into(), "help".into()],
            noop_callback(),
            None,
            true,
        );
        assert!(h.commands.contains("start"));
        assert!(h.commands.contains("help"));
    }

    #[test]
    #[should_panic(expected = "not a valid bot command")]
    fn invalid_command_panics() {
        CommandHandler::with_options(
            vec!["invalid command!".into()],
            noop_callback(),
            None,
            true,
        );
    }

    #[test]
    fn check_args_variants() {
        let h = CommandHandler::with_options(vec!["test".into()], noop_callback(), None, true);
        assert!(h.check_args(&[]));
        assert!(h.check_args(&["a".into()]));

        let h2 = CommandHandler::with_options(
            vec!["test".into()],
            noop_callback(),
            Some(HasArgs::Exact(2)),
            true,
        );
        assert!(!h2.check_args(&["a".into()]));
        assert!(h2.check_args(&["a".into(), "b".into()]));
    }

    // -- C1 tests ---------------------------------------------------------

    #[test]
    fn c1_bot_username_matching_accepted() {
        let h = CommandHandler::with_options(vec!["start".into()], noop_callback(), None, true)
            .with_bot_username("MyBot");
        let update = make_command_update("/start@MyBot");
        assert!(h.check_update(&update).is_some());
    }

    #[test]
    fn c1_bot_username_case_insensitive() {
        let h = CommandHandler::with_options(vec!["start".into()], noop_callback(), None, true)
            .with_bot_username("mybot");
        let update = make_command_update("/start@MYBOT");
        assert!(h.check_update(&update).is_some());
    }

    #[test]
    fn c1_wrong_bot_username_rejected() {
        let h = CommandHandler::with_options(vec!["start".into()], noop_callback(), None, true)
            .with_bot_username("MyBot");
        let update = make_command_update("/start@OtherBot");
        assert!(h.check_update(&update).is_none());
    }

    #[test]
    fn c1_no_at_suffix_still_accepted() {
        let h = CommandHandler::with_options(vec!["start".into()], noop_callback(), None, true)
            .with_bot_username("MyBot");
        let update = make_command_update("/start");
        assert!(h.check_update(&update).is_some());
    }

    #[test]
    fn c1_no_bot_username_configured_strips_suffix() {
        let h = CommandHandler::with_options(vec!["start".into()], noop_callback(), None, true);
        let update = make_command_update("/start@AnyBot");
        assert!(h.check_update(&update).is_some());
    }

    // -- C2 tests ---------------------------------------------------------

    #[test]
    fn c2_default_filter_accepts_edited_message() {
        let update = make_edited_command_update("/start");
        let h = CommandHandler::with_options(vec!["start".into()], noop_callback(), None, true);
        // Default filter now matches edited_message (like Python's UpdateType.MESSAGES).
        assert!(h.check_update(&update).is_some());
    }

    #[test]
    fn c2_default_filter_rejects_channel_post() {
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "channel_post": {
                "message_id": 1,
                "date": 0,
                "chat": {"id": -100, "type": "channel"},
                "text": "/start",
                "entities": [{"type": "bot_command", "offset": 0, "length": 6}]
            }
        }))
        .expect("valid");

        let h = CommandHandler::with_options(vec!["start".into()], noop_callback(), None, true);
        assert!(h.check_update(&update).is_none());
    }

    #[test]
    fn c2_custom_filter_allows_edited() {
        let update = make_edited_command_update("/start");

        let h = CommandHandler::with_options(vec!["start".into()], noop_callback(), None, true)
            .with_filter(Arc::new(|u| {
                u.message.is_some() || u.edited_message.is_some()
            }));
        assert!(h.check_update(&update).is_some());
    }

    #[test]
    fn c2_custom_filter_rejects() {
        let update = make_command_update("/start");
        let h = CommandHandler::with_options(vec!["start".into()], noop_callback(), None, true)
            .with_filter(Arc::new(|_u| false));
        assert!(h.check_update(&update).is_none());
    }

    // -- collect_additional_context tests ---------------------------------

    #[test]
    fn collect_context_populates_args() {
        use crate::context::CallbackContext;
        use crate::ext_bot::test_support::mock_request;
        use std::collections::HashMap;
        use telegram_bot_raw::bot::Bot;

        let bot = Arc::new(crate::ext_bot::ExtBot::from_bot(
            Bot::new("test", mock_request()),
        ));
        let stores = (
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        );
        let mut ctx = CallbackContext::new(bot, None, None, stores.0, stores.1, stores.2);

        let h = CommandHandler::with_options(vec!["start".into()], noop_callback(), None, true);
        let mr = MatchResult::Args(vec!["foo".into(), "bar".into()]);
        h.collect_additional_context(&mut ctx, &mr);

        assert_eq!(ctx.args, Some(vec!["foo".into(), "bar".into()]));
    }

    #[test]
    fn collect_context_no_op_for_empty() {
        use crate::context::CallbackContext;
        use crate::ext_bot::test_support::mock_request;
        use std::collections::HashMap;
        use telegram_bot_raw::bot::Bot;

        let bot = Arc::new(crate::ext_bot::ExtBot::from_bot(
            Bot::new("test", mock_request()),
        ));
        let stores = (
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        );
        let mut ctx = CallbackContext::new(bot, None, None, stores.0, stores.1, stores.2);

        let h = CommandHandler::with_options(vec!["start".into()], noop_callback(), None, true);
        h.collect_additional_context(&mut ctx, &MatchResult::Empty);

        assert!(ctx.args.is_none());
    }

    // -- Ergonomic constructor tests --------------------------------------

    #[test]
    fn ergonomic_new_check_update_works() {
        async fn dummy(_update: Update, _ctx: CallbackContext) -> Result<(), crate::application::HandlerError> {
            Ok(())
        }
        let h = CommandHandler::new("start", dummy);
        let update = make_command_update("/start");
        assert!(h.check_update(&update).is_some());
    }

    #[test]
    fn ergonomic_new_rejects_wrong_command() {
        async fn dummy(_update: Update, _ctx: CallbackContext) -> Result<(), crate::application::HandlerError> {
            Ok(())
        }
        let h = CommandHandler::new("start", dummy);
        let update = make_command_update("/help");
        assert!(h.check_update(&update).is_none());
    }
}
