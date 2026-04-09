# Inline Keyboards

Inline keyboards are buttons that appear directly below a message. They can trigger callback queries, open URLs, or switch to inline mode.

## Building a Keyboard

Use the typed constructors -- never raw JSON:

```rust
use rust_tg_bot::ext::prelude::{InlineKeyboardButton, InlineKeyboardMarkup};

fn build_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Option 1", "1"),
            InlineKeyboardButton::callback("Option 2", "2"),
        ],
        vec![
            InlineKeyboardButton::callback("Option 3", "3"),
        ],
    ])
}
```

Each inner `Vec` is one row of buttons. Each `InlineKeyboardButton::callback(text, data)` creates a button that sends `data` as a callback query when pressed.

### Single-Row Shortcut

```rust
let keyboard = InlineKeyboardMarkup::from_row(vec![
    InlineKeyboardButton::callback("Yes", "yes"),
    InlineKeyboardButton::callback("No", "no"),
]);
```

### Single-Button Shortcut

```rust
let keyboard = InlineKeyboardMarkup::from_button(
    InlineKeyboardButton::callback("Confirm", "confirm"),
);
```

## Sending a Keyboard

```rust
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap();
    let keyboard = serde_json::to_value(build_menu()).unwrap();

    context.bot()
        .send_message(chat_id, "Please choose an option:")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}
```

## Handling Button Presses

When a user presses an inline keyboard button, Telegram sends a callback query. Handle it with `FnHandler::on_callback_query`:

```rust
use rust_tg_bot::ext::prelude::FnHandler;

async fn button_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let cq = update.callback_query()
        .expect("callback query handler received update without callback_query");

    let data = cq.data.as_deref().unwrap_or("unknown");

    // Answer the callback query (removes the loading indicator)
    context.bot().answer_callback_query(&cq.id).await?;

    // Edit the original message to show the selection
    if let Some(msg) = cq.message.as_deref() {
        let response_text = format!("You selected: Option {data}");
        context.bot()
            .edit_message_text(&response_text)
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .await?;
    }

    Ok(())
}

// Register the callback handler
app.add_handler(FnHandler::on_callback_query(button_callback), 0).await;
```

### Always Answer Callback Queries

If you do not call `answer_callback_query`, the user sees a perpetual loading spinner on the button. Always answer, even if you have nothing to show:

```rust
context.bot().answer_callback_query(&cq.id).await?;
```

You can also show a notification:

```rust
context.bot()
    .answer_callback_query(&cq.id)
    .text("Saved!")
    .show_alert(true)  // Shows a modal alert instead of a toast
    .await?;
```

## Button Types

`InlineKeyboardButton` supports several types:

```rust
// Callback button -- sends data back to your bot
InlineKeyboardButton::callback("Click me", "callback_data")

// URL button -- opens a link
InlineKeyboardButton::url("Visit website", "https://example.com")
```

## Editing Keyboards

Update the keyboard on an existing message:

```rust
let new_keyboard = InlineKeyboardMarkup::new(vec![
    vec![InlineKeyboardButton::callback("Updated Option", "new_data")],
]);

context.bot()
    .edit_message_text("Updated message")
    .chat_id(chat_id)
    .message_id(message_id)
    .reply_markup(serde_json::to_value(new_keyboard).unwrap())
    .await?;
```

## Complete Example

```rust
use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, FnHandler,
    HandlerResult, InlineKeyboardButton, InlineKeyboardMarkup, Update,
};

fn build_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Option 1", "1"),
            InlineKeyboardButton::callback("Option 2", "2"),
        ],
        vec![InlineKeyboardButton::callback("Option 3", "3")],
    ])
}

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap();
    let keyboard = serde_json::to_value(build_keyboard()).unwrap();

    context.bot()
        .send_message(chat_id, "Please choose an option:")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

async fn button_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let cq = update.callback_query().unwrap();
    let data = cq.data.as_deref().unwrap_or("unknown");

    context.bot().answer_callback_query(&cq.id).await?;

    if let Some(msg) = cq.message.as_deref() {
        context.bot()
            .edit_message_text(&format!("You selected: Option {data}"))
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(CommandHandler::new("start", start), 0).await;
    app.add_handler(FnHandler::on_callback_query(button_callback), 0).await;

    app.run_polling().await.unwrap();
}
```

## Next Steps

- [Conversations](conversations.md) -- build multi-step flows using state and keyboards.
- [Payments](payments.md) -- inline keyboards are also used in payment flows.
