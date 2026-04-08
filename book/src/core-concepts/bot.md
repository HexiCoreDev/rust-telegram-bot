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
    .send()
    .await?;
```

### Formatted Text

Use `ParseMode` constants -- never raw strings:

```rust
use telegram_bot::ext::prelude::ParseMode;

context.bot()
    .send_message(chat_id, "<b>Bold</b> and <i>italic</i>")
    .parse_mode(ParseMode::Html)
    .send()
    .await?;
```

Available parse modes:
- `ParseMode::Html`
- `ParseMode::Markdown`
- `ParseMode::MarkdownV2`

### With Reply Markup

Attach keyboards or inline keyboards to messages:

```rust
use telegram_bot::ext::prelude::{InlineKeyboardButton, InlineKeyboardMarkup};

let keyboard = InlineKeyboardMarkup::new(vec![
    vec![
        InlineKeyboardButton::callback("Yes", "answer_yes"),
        InlineKeyboardButton::callback("No", "answer_no"),
    ],
]);

context.bot()
    .send_message(chat_id, "Do you agree?")
    .reply_markup(serde_json::to_value(keyboard).unwrap())
    .send()
    .await?;
```

## Editing Messages

```rust
context.bot()
    .edit_message_text("Updated text")
    .chat_id(chat_id)
    .message_id(message_id)
    .send()
    .await?;
```

## Answering Callback Queries

When a user presses an inline keyboard button, answer the callback query to dismiss the loading indicator:

```rust
let cq = update.callback_query().unwrap();

context.bot()
    .answer_callback_query(&cq.id)
    .send()
    .await?;
```

Optionally show a notification:

```rust
context.bot()
    .answer_callback_query(&cq.id)
    .text("Selection recorded!")
    .show_alert(true)
    .send()
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
// Send a photo
context.bot()
    .send_photo(chat_id, "https://example.com/photo.jpg")
    .caption("A nice photo")
    .send()
    .await?;

// Send an invoice
context.bot()
    .send_invoice(chat_id, title, description, payload, currency, prices)
    .send()
    .await?;

// Get chat member info
let member = context.bot()
    .get_chat_member(ChatId::Id(chat_id), user_id)
    .await?;

// Set webhook
context.bot()
    .set_webhook("https://example.com/webhook")
    .send()
    .await?;

// Answer inline query
context.bot()
    .answer_inline_query(&query_id, results)
    .send()
    .await?;
```

## Bot Data

After the application initializes (calls `getMe`), you can access the bot's own information:

```rust
if let Some(bot_data) = context.bot().bot_data() {
    let username = bot_data.username.as_deref().unwrap_or("unknown");
    let bot_id = bot_data.id;
}
```

## Builder Pattern

All API method calls follow a consistent builder pattern:

```rust
context.bot()
    .method_name(required_args)     // Start the builder
    .optional_param(value)          // Chain optional parameters
    .another_optional(value)        // Chain more
    .send()                         // Execute the request
    .await?;                        // Await the future
```

This pattern gives you IDE autocompletion for every parameter and catches typos at compile time.

## Next Steps

Now learn how updates are routed to your handler functions in [Handlers](handlers.md).
