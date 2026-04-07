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

### telegram_bot_raw

| Module | Key items |
|--------|-----------|
| `bot` | `Bot`, `ChatId`, `MessageOrBool`, `Defaults` |
| `bot_builders` | `SendMessageBuilder`, `EditMessageTextBuilder`, `AnswerCallbackQueryBuilder`, etc. |
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
| `application` | `Application`, `HandlerError`, `ApplicationError`, `HandlerCallback` |
| `builder` | `ApplicationBuilder`, `NoToken`, `HasToken` |
| `context` | `CallbackContext` |
| `ext_bot` | `ExtBot` |
| `defaults` | `Defaults`, `DefaultsBuilder` |
| `handlers::base` | `Handler` (trait), `HandlerResult`, `MatchResult` |
| `handlers::command` | `CommandHandler`, `HasArgs` |
| `handlers::message` | `MessageHandler` |
| `handlers::callback_query` | `CallbackQueryHandler`, `CallbackPattern` |
| `handlers::conversation` | `ConversationHandler`, `ConversationResult`, `ConversationKey` |
| `handlers::inline_query` | `InlineQueryHandler` |
| `filters::base` | `Filter` (trait), `FilterResult`, `F`, `FnFilter`, `ALL` |
| `filters::text` | `TEXT`, `CAPTION`, `TextFilter`, `CaptionFilter`, `DiceFilter`, etc. |
| `filters::command` | `COMMAND`, `CommandFilter` |
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

The raw HTTP client. Wraps the Telegram Bot API. Every method has both a positional-args
variant and a builder variant (prefixed with `build_`).

```
Bot::new(token, request) -> Bot
Bot::get_me() -> Result<User>

// Positional-args (all optional params explicit):
Bot::send_message(chat_id, text, parse_mode, ...) -> Result<Message>

// Builder (preferred):
Bot::build_send_message(chat_id, text) -> SendMessageBuilder
Bot::build_send_photo(chat_id, photo) -> SendPhotoBuilder
Bot::build_edit_message_text(chat_id, message_id, text) -> EditMessageTextBuilder
Bot::build_answer_callback_query(id) -> AnswerCallbackQueryBuilder
// ... builders for all major methods
```

Access via `context.bot().inner()` in handler callbacks.

### ExtBot

Wraps `Bot` with defaults, callback data caching, and rate limiting.

```
ExtBot::new(bot, defaults, arbitrary_callback_data, rate_limiter) -> ExtBot
ExtBot::inner() -> &Bot
ExtBot::defaults() -> Option<&Defaults>
ExtBot::token() -> &str
ExtBot::has_callback_data_cache() -> bool
```

### Defaults

Bot-wide default parameter values. Constructed via builder:

```rust
let defaults = Defaults::builder()
    .parse_mode("HTML")
    .disable_notification(true)
    .build();
```

### Application

The central dispatcher.

```
Application::new(...) -> Arc<Application>
app.add_handler(handler, group) -> impl Future
app.remove_handler(handler_id, group) -> impl Future
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

### CallbackContext

Passed to every handler callback.

```
context.bot() -> &Arc<ExtBot>
context.user_data() -> Arc<RwLock<HashMap<i64, JsonMap>>>
context.chat_data() -> Arc<RwLock<HashMap<i64, JsonMap>>>
context.bot_data() -> Arc<RwLock<JsonMap>>
context.job_queue() -> Option<Arc<JobQueue>>
context.args        -- Option<Vec<String>>   (set by CommandHandler)
context.matches     -- Option<Vec<String>>   (set by regex handlers)
context.named_matches -- Option<HashMap<String, String>>  (named regex groups)
```

### Handler trait

```
check_update(&self, update: &Update) -> Option<MatchResult>
handle_update(&self, update, match_result) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>>
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
telegram-bot = { git = "...", features = ["full"] }
```
