# Inline Mode

Inline mode lets users interact with your bot directly from the message input field of any chat. When a user types `@yourbotname query`, Telegram sends an inline query to your bot, and you respond with a list of results the user can pick from.

## Prerequisites

Before your bot can receive inline queries, you must enable inline mode through [@BotFather](https://t.me/BotFather):

1. Send `/mybots` and select your bot.
2. Choose "Bot Settings" then "Inline Mode".
3. Turn it on.

## Handling Inline Queries

Register an inline query handler with `FnHandler::on_inline_query`:

```rust
use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, FnHandler,
    HandlerResult, Update,
};
use rust_tg_bot::raw::types::inline::inline_query_result_article::InlineQueryResultArticle;
use rust_tg_bot::raw::types::inline::input_message_content::InputMessageContent;
use rust_tg_bot::raw::types::inline::input_text_message_content::InputTextMessageContent;

async fn inline_query_handler(update: Arc<Update>, context: Context) -> HandlerResult {
    let iq = match update.inline_query() {
        Some(q) => q,
        None => return Ok(()),
    };

    if iq.query.is_empty() {
        return Ok(());
    }

    let content = InputTextMessageContent::new(iq.query.to_uppercase());
    let article = InlineQueryResultArticle::new(
        format!("caps-{}", iq.id),
        "CAPS",
        InputMessageContent::Text(content),
    );

    let results = vec![
        serde_json::to_value(article).expect("article serialization"),
    ];

    context
        .bot()
        .answer_inline_query(&iq.id, results)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(FnHandler::on_inline_query(inline_query_handler), 0)
        .await;

    app.run_polling().await.unwrap();
}
```

## Building Results

### InlineQueryResultArticle

The most common result type. It shows a title and optional description, and sends a message when the user taps it.

```rust
use rust_tg_bot::raw::types::inline::inline_query_result_article::InlineQueryResultArticle;
use rust_tg_bot::raw::types::inline::input_message_content::InputMessageContent;
use rust_tg_bot::raw::types::inline::input_text_message_content::InputTextMessageContent;

let content = InputTextMessageContent::new("Hello, world!");
let article = InlineQueryResultArticle::new(
    "unique-id-1",          // Unique result ID
    "Say Hello",            // Title shown to the user
    InputMessageContent::Text(content),
);
```

### InputTextMessageContent

Controls the message that gets sent when the user selects a result.

```rust
// Plain text
let plain = InputTextMessageContent::new("Just plain text");

// HTML formatted
let html = InputTextMessageContent::new("<b>Bold</b> and <i>italic</i>")
    .parse_mode("HTML");

// MarkdownV2 formatted
let markdown = InputTextMessageContent::new("*Bold* and _italic_")
    .parse_mode("MarkdownV2");
```

## Multiple Results

Most inline bots offer several options. Each result needs a unique ID within the response:

```rust
async fn inline_query_handler(update: Arc<Update>, context: Context) -> HandlerResult {
    let iq = match update.inline_query() {
        Some(q) => q,
        None => return Ok(()),
    };

    let query = &iq.query;
    if query.is_empty() {
        return Ok(());
    }

    // Escape HTML special characters
    let escaped = query
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");

    let caps_content = InputTextMessageContent::new(query.to_uppercase());
    let bold_content = InputTextMessageContent::new(format!("<b>{escaped}</b>"))
        .parse_mode("HTML");
    let italic_content = InputTextMessageContent::new(format!("<i>{escaped}</i>"))
        .parse_mode("HTML");

    let results = vec![
        serde_json::to_value(InlineQueryResultArticle::new(
            format!("caps-{}", iq.id),
            "Caps",
            InputMessageContent::Text(caps_content),
        )).expect("serialization"),
        serde_json::to_value(InlineQueryResultArticle::new(
            format!("bold-{}", iq.id),
            "Bold",
            InputMessageContent::Text(bold_content),
        )).expect("serialization"),
        serde_json::to_value(InlineQueryResultArticle::new(
            format!("italic-{}", iq.id),
            "Italic",
            InputMessageContent::Text(italic_content),
        )).expect("serialization"),
    ];

    context
        .bot()
        .answer_inline_query(&iq.id, results)
        .await?;

    Ok(())
}
```

## Combining with Commands

Inline bots typically also respond to direct messages. Register both command handlers and the inline query handler:

```rust
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Hi! Use me inline by typing @botusername <query> in any chat.",
        )
        .await?;
    Ok(())
}

async fn help_command(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Type @botusername <text> in any chat. I will offer \
             CAPS, Bold, and Italic transformations.",
        )
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(CommandHandler::new("start", start), 0).await;
    app.add_handler(CommandHandler::new("help", help_command), 0).await;
    app.add_handler(FnHandler::on_inline_query(inline_query_handler), 0).await;

    println!("Inline bot is running. Press Ctrl+C to stop.");
    println!("Remember to enable inline mode with @BotFather!");

    app.run_polling().await.unwrap();
}
```

## Tips

- **Empty queries.** Always check if the query is empty. Users often trigger inline mode accidentally by typing `@botname` without a query.
- **Result limits.** Telegram allows up to 50 results per `answer_inline_query` call.
- **Caching.** Telegram caches results for 300 seconds by default. Use the `cache_time` builder method on `answer_inline_query` to adjust.
- **Unique IDs.** Every result in a single response must have a unique `id`. Using the inline query's own `id` as a prefix (e.g., `format!("caps-{}", iq.id)`) is a reliable pattern.
- **HTML escaping.** Always escape user input before embedding it in HTML-formatted content to prevent injection.
- **Serialization.** Each result must be serialized to `serde_json::Value` via `serde_json::to_value()` before passing to `answer_inline_query`.
- **10-second deadline.** You must call `answer_inline_query` within 10 seconds of receiving the query, or Telegram will show an error to the user.

## Next Steps

- [Payments](payments.md) -- accept payments directly through Telegram.
- [Inline Keyboards](inline-keyboards.md) -- the related feature for buttons within messages.
