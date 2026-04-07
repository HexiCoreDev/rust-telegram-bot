# Architecture

---

## Two-Layer Design

The library is a Cargo workspace with three crates:

```
rust-telegram-bot/
  crates/
    telegram-bot-raw/     # Layer 1: pure API types and HTTP client
    telegram-bot-ext/     # Layer 2: application framework
    telegram-bot/         # Facade: re-exports both layers
```

This mirrors the design of python-telegram-bot where `telegram` (raw types) and
`telegram.ext` (application framework) are separate but ship together.

### telegram-bot-raw

Contains every type defined by Bot API 9.6 and the `Bot` struct that calls the API.

Responsibilities:
- All API types: `Message`, `Update`, `User`, `Chat`, `CallbackQuery`, etc.
- All API methods on `Bot`: `send_message`, `get_updates`, `answer_callback_query`, etc.
- Builder variants for common methods: `build_send_message`, `build_edit_message_text`, etc.
- `ChatId` enum (numeric ID or `@username`)
- `Defaults` struct for bot-wide default parameters
- HTTP transport abstraction (`BaseRequest` trait, `reqwest` implementation)

The `Bot` struct caches the result of `getMe` after `initialize()` is called so the
application can look up the bot's own username for command validation without a network
round-trip.

`telegram-bot-raw` depends only on `serde`, `reqwest`, and `tokio`. It has no knowledge
of handlers, filters, or persistence.

### telegram-bot-ext

Provides the application framework.

Responsibilities:
- `ApplicationBuilder` -- typestate builder that produces an `Application`
- `Application` -- the central dispatcher (handler groups, polling, webhooks)
- 21 handler types
- 50+ composable filters
- `ConversationHandler` state machine
- `JobQueue` with tokio-based scheduling
- `BasePersistence` trait and JSON/SQLite backends
- `ExtBot` -- wraps `Bot` with defaults, callback data caching, and rate limiting
- `CallbackContext` -- context object passed to every handler callback

### telegram-bot (facade)

Re-exports both crates so users only add one dependency:

```rust
use telegram_bot::ext::application::Application;    // from telegram-bot-ext
use telegram_bot::raw::types::message::Message;     // from telegram-bot-raw
```

---

## Crate Dependency Graph

```
telegram-bot  ──depends on──>  telegram-bot-ext
                                     │
                               depends on
                                     │
                                     v
                              telegram-bot-raw
```

`telegram-bot-ext` depends on `telegram-bot-raw`. The facade `telegram-bot` depends on
both.

---

## Update Processing Pipeline

When `run_polling` (or the webhook equivalent) is active, updates flow through this
pipeline:

```
Telegram servers
      │
      │  HTTP long-poll (getUpdates)
      v
  UpdateFetcher  (inside Application::run_polling)
      │
      │  Vec<Update>
      v
  UpdateProcessor  (SimpleUpdateProcessor or concurrent variant)
      │
      │  individual Update
      v
  Application::process_update
      │
      ├─ Persistence refresh (refresh_user_data, refresh_chat_data)
      │
      ├─ Group 0  ──>  check_update on each Handler
      │                  first match wins  ──>  handle_update
      │                  if block=true: await the future
      │                  if HandlerResult::Stop: stop here
      │
      ├─ Group 1  (skipped if group 0 returned Stop)
      │
      ├─ Group N ...
      │
      └─ Persistence write (update_user_data, update_chat_data, update_bot_data)
```

Key points:

- Groups are stored in a `BTreeMap<i32, Vec<Handler>>`, iterated in ascending key order.
- Within a group, the first handler whose `check_update` returns `Some` wins. Others in
  the same group are not called.
- A handler returning `HandlerResult::Stop` prevents all subsequent groups from running.
- `HandlerResult::Continue` lets processing fall through to the next group.
- `block: true` means the application awaits the handler's future before moving to the
  next group. `block: false` spawns the future on a tokio task (non-blocking).

---

## Handler Dispatch Detail

The `Handler` trait has two phases:

```rust
pub trait Handler: Send + Sync {
    fn check_update(&self, update: &Update) -> Option<MatchResult>;

    fn handle_update(
        &self,
        update: Update,
        match_result: MatchResult,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>>;

    fn block(&self) -> bool { true }

    fn collect_additional_context(
        &self,
        context: &mut CallbackContext,
        match_result: &MatchResult,
    ) {}
}
```

`check_update` is synchronous and cheap -- it tests a filter, checks a regex, or inspects
a field. It returns `None` (skip) or `Some(MatchResult)` (handle).

`handle_update` is async. It receives the update and the `MatchResult` produced by
`check_update`. The match result carries handler-specific data: argument lists for
`CommandHandler`, regex captures for `CallbackQueryHandler`, etc.

`collect_additional_context` runs between `check_update` and the callback invocation. It
injects handler-specific data into the `CallbackContext` (e.g., `context.args` for
command handlers, `context.matches` for regex handlers).

---

## ExtBot vs Bot

User-facing code works with `ExtBot`, not `Bot` directly. `ExtBot` wraps `Bot` and adds:

- **Defaults**: bot-wide parse mode, notification settings, link preview options. When
  set, these are merged into every outgoing API call where the caller has not supplied an
  explicit value.
- **Callback data cache**: stores arbitrary data associated with inline keyboard buttons.
  The cache key is a short string; the actual data lives in the application's memory.
- **Rate limiter**: pluggable rate-limiting middleware (implementation is currently
  a stub; the hook point is present).

Access the underlying `Bot` via `ext_bot.inner()`.

---

## Comparison with python-telegram-bot Architecture

| Concept | python-telegram-bot | rust-telegram-bot |
|---------|---------------------|-------------------|
| Raw types + HTTP | `telegram` package | `telegram-bot-raw` crate |
| Application framework | `telegram.ext` package | `telegram-bot-ext` crate |
| Main entry point | `Application` class | `Application` struct (`Arc<Application>`) |
| Builder | `ApplicationBuilder` class | `ApplicationBuilder<State>` (typestate) |
| Handler base class | `BaseHandler` | `Handler` trait |
| Handler check | `check_update(update) -> bool` | `check_update(update) -> Option<MatchResult>` |
| Context | `CallbackContext` dataclass | `CallbackContext` struct |
| Bot on types | `message.reply_text(...)` | Not on types; use builder API (see below) |
| Persistence | `PicklePersistence`, custom | `JsonFilePersistence`, `SqlitePersistence`, custom trait |
| Job queue | APScheduler | tokio timers |
| Update delivery | asyncio + requests | tokio + reqwest |

---

## Design Decisions

### Why no `Bot` on types?

In python-telegram-bot, `Message`, `User`, and `Chat` objects carry a reference to the
`Bot` instance, allowing you to call `message.reply_text("hi")`. This is convenient but
creates a circular reference problem and complicates serialisation and cloning.

In the Rust port, types are plain data structures (serialisable, cloneable, `Send + Sync`).
The bot is accessed through the `CallbackContext` that handlers receive:

```rust
context.bot().inner()
    .build_send_message(chat_id.into(), "hi")
    .send()
    .await?;
```

This keeps the type model clean and makes the data types easy to persist, log, and test.

### Why `FilterResult` instead of `bool`?

Python filters return `bool | DataDict`. The Rust port uses an enum:

```rust
pub enum FilterResult {
    NoMatch,
    Match,
    MatchWithData(HashMap<String, Vec<String>>),
}
```

This encodes the three possible outcomes (no match, simple match, match with extracted
data) in the type system. When an `AndFilter` combines two data filters, the data maps
are merged automatically via `FilterResult::merge`. There is no ambiguity between a
filter that matched but produced no data and one that has not been evaluated yet.

### Why a typestate builder?

`ApplicationBuilder` uses phantom type parameters to enforce that `.token()` is called
before `.build()`. The compiler rejects code that calls `.build()` on a builder without a
token -- no runtime check needed:

```rust
// Compile error: no `.build()` method on ApplicationBuilder<NoToken>
let app = ApplicationBuilder::new().build();

// Compiles:
let app = ApplicationBuilder::new().token("...").build();
```

### Why `Arc<Application>`?

`Application` is wrapped in `Arc` so it can be shared safely across tokio tasks: the
polling loop, handler callbacks, and the job queue all need a handle to the application
to look up bot credentials and dispatch updates.
