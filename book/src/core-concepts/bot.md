# Bot

The `Bot` object is your interface to the Telegram Bot API. It wraps HTTP calls behind typed builder methods, so you never construct raw JSON payloads.

## Accessing the Bot

Inside a handler, get the bot through the context:

```rust
async fn my_handler(update: Arc<Update>, context: Context) -> HandlerResult {
    let bot = context.bot();
    // Use bot to call Telegram API methods
    Ok(())
}
```

`context.bot()` returns an `Arc<ExtBot>`, which wraps the raw `Bot` with framework extensions (defaults, callback data cache, rate limiting).

## Sending Messages

### Basic Text

```rust
let chat_id = update.effective_chat().map(|c| c.id).unwrap();

context.bot()
    .send_message(chat_id, "Hello, world!")
    .await?;
```

### Formatted Text

Use `ParseMode` constants -- never raw strings:

```rust
use rust_tg_bot::ext::prelude::ParseMode;

context.bot()
    .send_message(chat_id, "<b>Bold</b> and <i>italic</i>")
    .parse_mode(ParseMode::Html)
    .await?;
```

Available parse modes:
- `ParseMode::Html`
- `ParseMode::Markdown`
- `ParseMode::MarkdownV2`

### With Reply Markup

Attach keyboards or inline keyboards to messages:

```rust
use rust_tg_bot::ext::prelude::{InlineKeyboardButton, InlineKeyboardMarkup};

let keyboard = InlineKeyboardMarkup::new(vec![
    vec![
        InlineKeyboardButton::callback("Yes", "answer_yes"),
        InlineKeyboardButton::callback("No", "answer_no"),
    ],
]);

context.bot()
    .send_message(chat_id, "Do you agree?")
    .reply_markup(serde_json::to_value(keyboard).unwrap())
    .await?;
```

## Editing Messages

```rust
context.bot()
    .edit_message_text("Updated text")
    .chat_id(chat_id)
    .message_id(message_id)
    .await?;
```

## Answering Callback Queries

When a user presses an inline keyboard button, answer the callback query to dismiss the loading indicator:

```rust
let cq = update.callback_query().unwrap();

context.bot()
    .answer_callback_query(&cq.id)
    .await?;
```

Optionally show a notification:

```rust
context.bot()
    .answer_callback_query(&cq.id)
    .text("Selection recorded!")
    .show_alert(true)
    .await?;
```

## The Convenience Method: reply_text

For the common case of replying to whatever chat sent the update:

```rust
context.reply_text(&update, "Thanks for your message!").await?;
```

This extracts the chat ID from the update and calls `send_message` for you.

## Other API Methods

The bot exposes builder methods for every Telegram Bot API method. Here are some common ones:

```rust
// Send a photo -- media arguments take an InputFile, not a bare &str
context.bot()
    .send_photo(chat_id, InputFile::url("https://example.com/photo.jpg"))
    .caption("A nice photo")
    .await?;

// Send an invoice
context.bot()
    .send_invoice(chat_id, title, description, payload, currency, prices)
    .await?;

// Get chat member info
let member = context.bot()
    .get_chat_member(ChatId::Id(chat_id), user_id)
    .await?;

// Set webhook
context.bot()
    .set_webhook("https://example.com/webhook")
    .await?;

// Answer inline query
context.bot()
    .answer_inline_query(&query_id, results)
    .await?;
```

## Bot Data

The bot's own `User` record is fetched once during `initialize()` and cached
internally; there is no public accessor for it. Call `get_me()` yourself if you
need the fields:

```rust
let me = context.bot().get_me().await?;
let username = me.username.as_deref().unwrap_or("unknown");
let bot_id = me.id;
```

## Builder Pattern

All API method calls follow a consistent builder pattern:

```rust
context.bot()
    .method_name(required_args)     // Start the builder
    .optional_param(value)          // Chain optional parameters
    .another_optional(value)        // Chain more
    .await?;                        // Await the future (IntoFuture)
```

This pattern gives you IDE autocompletion for every parameter and catches typos at compile time.

## Rate Limiting

With the `rate-limiter` feature, API calls are throttled at the transport
layer — every request through `context.bot()` waits its turn, and nothing else
in your handler changes:

```rust,ignore
use std::time::Duration;
use rust_tg_bot::ext::rate_limiter::AioRateLimiter;

// 30 messages/second overall, 1/second per chat group, up to 3 retries.
// Pass 0 for a rate or period to disable that level.
let limiter = AioRateLimiter::new(
    30,
    Duration::from_secs(1),
    1,
    Duration::from_secs(1),
    3,
);

let app = ApplicationBuilder::new()
    .token(token)
    .rate_limiter(Arc::new(limiter))
    .build();
```

The limiter reacts to Telegram's own `RetryAfter` responses, so a 429 from the
server pauses every subsequent request rather than just the one that hit it.

You can also handle this yourself. `TelegramError::RetryAfter { retry_after }`
carries the duration Telegram asked for:

```rust,ignore
match context.bot().send_message(chat_id, "hi").await {
    Ok(_) => {}
    Err(TelegramError::RetryAfter { retry_after }) => {
        tracing::warn!("rate limited; retrying in {retry_after:?}");
    }
    Err(e) => return Err(e.into()),
}
```

Note that the error type does not expose a numeric `error_code`, and HTTP 401
and 404 are folded into a single `InvalidToken` variant — so "revoked token"
and "typo in the method name" are not distinguishable from the variant alone.

## Next Steps

Now learn how updates are routed to your handler functions in [Handlers](handlers.md).

## Runnable examples

The code above is abridged. These run in full:

| Example | What to look at |
|---|---|
| [`raw_api_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/raw_api_bot.rs) | calling the Bot API without the ext layer |

Every example is listed in the [example index](../examples.md).
