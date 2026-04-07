# API Reference

The authoritative API reference is generated from doc-comments in the source code using
`cargo doc`. This page explains how to generate it locally and provides a navigable
overview of the most important public types.

---

## Generating the Docs

```sh
# In the repository root
cargo doc --open

# Include documentation for dependencies (useful for understanding tokio, serde, etc.)
cargo doc --open --document-private-items

# Generate without opening a browser
cargo doc --no-deps
```

The output is placed in `target/doc/`. Open `target/doc/telegram_bot/index.html` in a
browser. The facade crate re-exports everything, so starting there gives you a single
index.

---

## Crate Overview

### telegram_bot (facade)

Entry point for the public API. Re-exports from both raw and ext crates under two
namespaces:

- `telegram_bot::raw` -- all types and methods from `telegram-bot-raw`
- `telegram_bot::ext` -- application framework from `telegram-bot-ext`
- `telegram_bot::ext::prelude` -- convenient re-exports for typical bot code
- `telegram_bot::run(future)` -- tokio runtime entry point with proper stack sizing

### telegram_bot_raw

| Module | Key items |
|--------|-----------|
| `bot` | `Bot`, `ChatId`, `MessageOrBool`, `Defaults` |
| `bot_builders` | `SendMessageBuilder`, `SendPhotoBuilder`, `EditMessageTextBuilder`, `AnswerCallbackQueryBuilder`, etc. |
| `constants` | `ParseMode`, `ChatType`, `MessageEntityType`, `ChatAction`, `ChatMemberStatus` |
| `types::update` | `Update` |
| `types::message` | `Message`, `MessageEntity` |
| `types::user` | `User` |
| `types::chat` | `Chat` |
| `types::callback_query` | `CallbackQuery` |
| `types::inline_query` | `InlineQuery` |
| `types::poll` | `Poll`, `PollAnswer` |
| `error` | `TelegramError`, `Result` |
| `request::base` | `BaseRequest`, `TimeoutOverride` |

### telegram_bot_ext

| Module | Key items |
|--------|-----------|
| `prelude` | `Update`, `Context`, `HandlerResult`, `ApplicationBuilder`, `CommandHandler`, `MessageHandler`, `FnHandler`, `TEXT()`, `COMMAND()`, `ParseMode`, `ChatType`, `MessageEntityType`, `F`, `Filter` |
| `application` | `Application`, `HandlerError`, `ApplicationError` |
| `builder` | `ApplicationBuilder`, `NoToken`, `HasToken` |
| `context` | `CallbackContext`, `DataReadGuard`, `DataWriteGuard` |
| `ext_bot` | `ExtBot` (implements `Deref<Target = Bot>`) |
| `defaults` | `Defaults`, `DefaultsBuilder` |
| `handlers::base` | `Handler` (trait), `HandlerResult`, `MatchResult`, `FnHandler` |
| `handlers::command` | `CommandHandler`, `HasArgs` |
| `handlers::message` | `MessageHandler` |
| `handlers::callback_query` | `CallbackQueryHandler`, `CallbackPattern` |
| `handlers::conversation` | `ConversationHandler`, `ConversationResult`, `ConversationKey` |
| `handlers::inline_query` | `InlineQueryHandler` |
| `filters::base` | `Filter` (trait), `FilterResult`, `F`, `FnFilter`, `ALL` |
| `filters::text` | `TextAny`, `CAPTION`, `TextFilter`, `CaptionFilter`, `DiceFilter`, etc. |
| `filters::command` | `CommandFilter` |
| `filters::photo` | `PHOTO`, `STICKER`, `DOCUMENT`, `AUDIO`, `VOICE`, `VIDEO` |
| `filters::status_update` | `STATUS_UPDATE`, `NEW_CHAT_MEMBERS`, etc. |
| `filters::regex` | `RegexFilter` |
| `filters::entity` | `EntityFilter` |
| `filters::user` | `UserFilter` |
| `filters::chat` | `ChatFilter` |
| `filters::document` | `DocumentFilter` |
| `persistence::base` | `BasePersistence` (trait), `PersistenceInput`, `PersistenceError` |
| `persistence::json_file` | `JsonFilePersistence` |
| `persistence::sqlite` | `SqlitePersistence` |
| `persistence::dict` | `DictPersistence` |
| `job_queue` | `JobQueue`, `Job`, `JobContext`, `JobCallbackFn`, `RunOnceBuilder`, `RunRepeatingBuilder`, `RunDailyBuilder`, `RunMonthlyBuilder` |

---

## Core Types Reference

### Bot

The raw HTTP client. Wraps the Telegram Bot API. Every method returns a builder with
chainable setters for optional parameters. Builders implement `IntoFuture`.

```
Bot::new(token, request) -> Bot
Bot::get_me() -> Result<User>

// Builder pattern (preferred):
Bot::send_message(chat_id, text) -> SendMessageBuilder
Bot::send_photo(chat_id, photo) -> SendPhotoBuilder
Bot::edit_message_text(text) -> EditMessageTextBuilder
Bot::answer_callback_query(id) -> AnswerCallbackQueryBuilder
// ... builders for all major methods
```

Access via `context.bot()` in handler callbacks. `ExtBot` implements `Deref<Target = Bot>`,
so all `Bot` methods are callable directly.

### ExtBot

Wraps `Bot` with defaults, callback data caching, and rate limiting. Implements
`Deref<Target = Bot>` for zero-cost access to all `Bot` methods.

```
ExtBot::from_bot(bot) -> ExtBot
ExtBot::builder(token, request) -> ExtBotBuilder
ExtBot::defaults() -> Option<&Defaults>
ExtBot::token() -> &str
ExtBot::has_callback_data_cache() -> bool
```

### Defaults

Bot-wide default parameter values. Constructed via builder:

```rust
let defaults = Defaults::builder()
    .parse_mode(ParseMode::Html)
    .disable_notification(true)
    .build();
```

### Application

The central dispatcher.

```
Application::new(...) -> Arc<Application>
app.add_typed_handler(handler, group) -> impl Future
app.run_polling() -> impl Future<Output = Result<()>>
app.polling() -> PollingBuilder  // configurable polling
app.initialize() -> impl Future<Output = Result<()>>
app.start() -> impl Future<Output = Result<()>>
app.stop() -> impl Future<Output = Result<()>>
app.shutdown() -> impl Future<Output = Result<()>>
app.bot() -> &Arc<ExtBot>
app.job_queue() -> Option<Arc<JobQueue>>
app.concurrent_updates() -> usize
```

### ApplicationBuilder

Typestate builder. `.token()` transitions from `NoToken` to `HasToken`. `.build()` is
only available on `ApplicationBuilder<HasToken>`.

```
ApplicationBuilder::new() -> ApplicationBuilder<NoToken>
builder.token(token) -> ApplicationBuilder<HasToken>
builder.defaults(defaults) -> Self
builder.concurrent_updates(n) -> Self
builder.persistence(p) -> Self
builder.job_queue(jq) -> Self
builder.post_init(hook) -> Self
builder.post_stop(hook) -> Self
builder.post_shutdown(hook) -> Self
builder.arbitrary_callback_data(maxsize) -> Self
builder.base_url(url) -> Self
builder.build() -> Arc<Application>  // HasToken only
```

### Context (CallbackContext)

Passed to every handler callback. Aliased as `Context` in the prelude.

```
context.bot() -> &Arc<ExtBot>
context.bot_data().await -> DataReadGuard        // typed read guard
context.bot_data_mut().await -> DataWriteGuard   // typed write guard
context.user_data().await -> Option<DefaultData> // cloned snapshot
context.chat_data().await -> Option<DefaultData> // cloned snapshot
context.set_user_data(key, value).await -> bool
context.set_chat_data(key, value).await -> bool
context.reply_text(&update, text).await -> Result<Message, TelegramError>
context.chat_id() -> Option<i64>
context.user_id() -> Option<i64>
context.args        -- Option<Vec<String>>   (set by CommandHandler)
context.matches     -- Option<Vec<String>>   (set by regex handlers)
context.named_matches -- Option<HashMap<String, String>>  (named regex groups)
context.job_queue   -- Option<Arc<JobQueue>>
context.error       -- Option<Arc<dyn Error + Send + Sync>>
```

### DataReadGuard

Typed read access to a `HashMap<String, Value>`:

```
guard.get_str(key) -> Option<&str>
guard.get_i64(key) -> Option<i64>
guard.get_f64(key) -> Option<f64>
guard.get_bool(key) -> Option<bool>
guard.get(key) -> Option<&Value>
guard.get_id_set(key) -> HashSet<i64>
guard.raw() -> &HashMap<String, Value>
guard.len() -> usize
guard.is_empty() -> bool
```

### DataWriteGuard

Typed write access to a `HashMap<String, Value>`:

```
guard.set_str(key, value)
guard.set_i64(key, value)
guard.set_bool(key, value)
guard.insert(key, value) -> Option<Value>
guard.add_to_id_set(key, id)
guard.remove_from_id_set(key, id)
guard.remove(key) -> Option<Value>
guard.entry(key) -> Entry
guard.get_mut(key) -> Option<&mut Value>
// Plus all DataReadGuard methods
```

### Handler trait

```
check_update(&self, update: &Update) -> Option<MatchResult>
handle_update(&self, update, match_result) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>>
handle_update_with_context(&self, update, match_result, context) -> Pin<Box<...>>
block(&self) -> bool
collect_additional_context(&self, context: &mut CallbackContext, match_result: &MatchResult)
```

### Filter trait

```
check_update(&self, update: &Update) -> FilterResult
name(&self) -> &str
```

### BasePersistence trait

See [persistence.md](persistence.md) for the full method list.

### JobQueue

Scheduling uses builder methods:

```
jq.once(callback, delay) -> RunOnceBuilder
jq.repeating(callback, interval) -> RunRepeatingBuilder
jq.daily(callback, time, days) -> RunDailyBuilder
jq.monthly(callback, time, day) -> RunMonthlyBuilder
jq.run_custom(callback, trigger, name, data, chat_id, user_id) -> Job
jq.jobs() -> impl Future<Output = Vec<Job>>
jq.get_jobs_by_name(name) -> impl Future<Output = Vec<Job>>
jq.jobs_by_pattern(regex) -> impl Future<Output = Vec<Job>>
jq.start() -> impl Future
jq.stop() -> impl Future
```

---

## Feature Flags Reference

| Flag | Adds |
|------|------|
| (none / default) | Polling, all handlers, all filters, in-memory data |
| `job-queue` | `JobQueue` and scheduling methods |
| `persistence-json` | `JsonFilePersistence` |
| `persistence-sqlite` | `SqlitePersistence`, `rusqlite` dependency |
| `webhooks` | axum-based webhook server |
| `rate-limiter` | Rate limiting middleware hook |
| `full` | All of the above |

```toml
[dependencies]
telegram-bot = { git = "https://github.com/HexiCoreDev/rust-telegram-bot", features = ["full"] }
```
