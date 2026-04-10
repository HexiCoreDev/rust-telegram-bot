# API Reference

This page provides an overview of the crate structure, key types, and how to explore the full API documentation.

## Generating Documentation

The most detailed and always-up-to-date reference is the `cargo doc` output. Generate and open it with:

```sh
cargo doc --open -p rust-tg-bot --all-features
```

This builds HTML documentation for all public types, traits, and functions, including source links and cross-references.

To include the underlying crates:

```sh
cargo doc --open --workspace --all-features
```

## Online Documentation

- **Guide**: [rust-tg-bot-docs.vercel.app](https://rust-tg-bot-docs.vercel.app/) -- mdBook with tutorials, guides, and architecture docs
- **API reference**: [docs.rs/rust-tg-bot](https://docs.rs/rust-tg-bot) -- auto-generated from source (available after crates.io publish)

## Crate Structure

The framework is split into four crates:

| Crate | Purpose | You Use Directly? |
|---|---|---|
| `rust-tg-bot` | Facade crate -- re-exports all of the below | Yes |
| `rust-tg-bot-raw` | Low-level Bot API types, HTTP methods, request builders | Rarely |
| `rust-tg-bot-ext` | High-level Application, handlers, filters, context, persistence | Rarely |
| `rust-tg-bot-macros` | Proc macros (`#[derive(BotCommands)]`) | Rarely |

You almost always depend only on `rust-tg-bot` in your `Cargo.toml`. It re-exports everything you need through two module paths:

- `rust_tg_bot::ext::prelude` -- handlers, filters, context, application
- `rust_tg_bot::raw::types` -- low-level Telegram types when needed

## The Prelude

The `rust_tg_bot::ext::prelude` module re-exports the most commonly needed types:

### Core Application Types

| Type | Description |
|---|---|
| `Application` | The main application that manages handlers, dispatching, and lifecycle |
| `ApplicationBuilder` | Builder for constructing an `Application` with token, persistence, etc. |
| `HandlerResult` | Alias for `Result<(), HandlerError>` |
| `HandlerError` | Error type returned by handlers |
| `Context` | Alias for `CallbackContext` -- passed to every handler |
| `CallbackContext` | The full context type with bot, data access, and convenience methods |

### Handler Types

| Type | Description |
|---|---|
| `CommandHandler` | Matches `/command` messages |
| `MessageHandler` | Matches messages that pass a filter |
| `FnHandler` | Custom predicate-based handler for any update type |
| `CallbackQueryHandler` | Matches callback queries from inline keyboards |

### Filter Types

| Type | Description |
|---|---|
| `F` | Wrapper around `Arc<dyn Filter>` with `&`, `\|`, `!` operators |
| `Filter` | The trait every filter implements |
| `FilterResult` | `NoMatch`, `Match`, or `MatchWithData` |
| `TEXT()` | Function returning a filter for text messages |
| `COMMAND()` | Function returning a filter for bot commands |

### Telegram Types

| Type | Description |
|---|---|
| `Update` | A Telegram update (message, callback query, inline query, etc.) |
| `Arc` | Re-exported `std::sync::Arc` for wrapping `Update` |
| `Message` | A Telegram message |
| `User` | A Telegram user |
| `Chat` | A Telegram chat |
| `ChatId` | Enum for chat identifiers |
| `CallbackQuery` | Data from an inline keyboard button press |

### Keyboard Types

| Type | Description |
|---|---|
| `InlineKeyboardMarkup` | Layout for inline keyboard buttons |
| `InlineKeyboardButton` | A single inline keyboard button |
| `ReplyKeyboardMarkup` | Layout for reply keyboard buttons |
| `KeyboardButton` | A single reply keyboard button |
| `ReplyKeyboardRemove` | Removes the reply keyboard |
| `ForceReply` | Forces the user to reply |

### Constants

| Type | Description |
|---|---|
| `ParseMode` | `Html`, `Markdown`, `MarkdownV2` |
| `ChatAction` | `Typing`, `UploadPhoto`, etc. |
| `ChatType` | `Private`, `Group`, `Supergroup`, `Channel` |
| `MessageEntityType` | `BotCommand`, `Mention`, `Url`, etc. |
| `ChatMemberStatus` | `Creator`, `Administrator`, `Member`, etc. |

### Data Types

| Type | Description |
|---|---|
| `DataReadGuard` | Typed read guard for bot-wide data |
| `DataWriteGuard` | Typed write guard for bot-wide data |
| `JsonValue` | Re-exported `serde_json::Value` |
| `HashMap` | Re-exported `std::collections::HashMap` |
| `RwLock` | Re-exported `tokio::sync::RwLock` |

### Feature-Gated Types

| Type | Feature | Description |
|---|---|---|
| `WebhookConfig` | `webhooks` | Configuration for webhook mode |
| `WebhookHandler` | `webhooks` | Handles incoming webhook requests |
| `WebhookServer` | `webhooks` | Built-in webhook HTTP server |

## Raw Types Module

For types not in the prelude, import from `rust_tg_bot::raw::types`:

```rust
// Inline query types
use rust_tg_bot::raw::types::inline::inline_query_result_article::InlineQueryResultArticle;
use rust_tg_bot::raw::types::inline::input_message_content::InputMessageContent;
use rust_tg_bot::raw::types::inline::input_text_message_content::InputTextMessageContent;

// Payment types
use rust_tg_bot::raw::types::payment::labeled_price::LabeledPrice;
use rust_tg_bot::raw::types::payment::shipping_option::ShippingOption;

// Chat member types
use rust_tg_bot::raw::types::chat_member::ChatMember;
```

## Persistence Module

Persistence types live outside the prelude:

```rust
// The trait
use rust_tg_bot::ext::persistence::base::{
    BasePersistence, PersistenceError, PersistenceInput, PersistenceResult,
};

// Backends
use rust_tg_bot::ext::persistence::json_file::JsonFilePersistence;

#[cfg(feature = "persistence-sqlite")]
use rust_tg_bot::ext::persistence::sqlite::SqlitePersistence;
```

## Filters Module

Advanced filter types beyond `TEXT()` and `COMMAND()`:

```rust
use rust_tg_bot::ext::filters::base::{
    Filter, FilterResult, FnFilter, F, ALL,
    PHOTO, VIDEO, AUDIO, VOICE, LOCATION, CONTACT,
};

use rust_tg_bot::ext::filters::text::{TextFilter, CaptionFilter, CAPTION};
use rust_tg_bot::ext::filters::regex::RegexFilter;
use rust_tg_bot::ext::filters::user::UserFilter;
use rust_tg_bot::ext::filters::chat::{ChatTypePrivate, ChatTypeGroup};
use rust_tg_bot::ext::filters::update_type::{MESSAGE, EDITED_MESSAGE};
```

## Bot Methods

The `Bot` (accessed via `context.bot()`) provides methods mirroring the Telegram Bot API. Most methods return a builder that you finalise with `.await` (builders implement `IntoFuture`):

```rust
// Send a message
context.bot().send_message(chat_id, "text").await?;

// Send with options
context.bot().send_message(chat_id, "text")
    .parse_mode(ParseMode::Html)
    .reply_markup(keyboard)
    .await?;

// Edit a message
context.bot().edit_message_text("new text")
    .chat_id(chat_id)
    .message_id(msg_id)
    .await?;

// Answer a callback query
context.bot().answer_callback_query(&cq_id).await?;

// Answer an inline query
context.bot().answer_inline_query(&iq_id, results).await?;

// Send an invoice
context.bot().send_invoice(chat_id, title, desc, payload, currency, prices)
    .provider_token(&token)
    .await?;

// Set webhook
context.bot().set_webhook(&url).await?;

// Delete webhook
context.bot().delete_webhook(false).await?;
```

### Convenience Methods on Context

`Context` provides shortcuts that skip the builder pattern:

```rust
// Reply with text (extracts chat_id from update automatically)
context.reply_text(&update, "Hello!").await?;

// Reply with HTML
context.reply_html(&update, "<b>Bold</b>").await?;

// Reply with MarkdownV2
context.reply_markdown_v2(&update, "*Bold*").await?;
```

## Next Steps

For the complete API surface, run `cargo doc --open -p rust-tg-bot --all-features` in your project. The generated documentation includes every public type, method, and trait implementation with source links.
