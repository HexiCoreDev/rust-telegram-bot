# Command Bots

Commands are the primary way users interact with Telegram bots. This guide covers everything from simple command handlers to commands with arguments and deep linking.

## Basic Commands

Use `CommandHandler` for straightforward command matching:

```rust
use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, HandlerResult, Update,
};

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    context.reply_text(&update, "Welcome! Use /help to see what I can do.").await?;
    Ok(())
}

async fn help(update: Arc<Update>, context: Context) -> HandlerResult {
    context.reply_text(&update, 
        "Available commands:\n\
         /start - Start the bot\n\
         /help - Show this message\n\
         /settings - Bot settings"
    ).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(CommandHandler::new("start", start), 0).await;
    app.add_handler(CommandHandler::new("help", help), 0).await;

    app.run_polling().await.unwrap();
}
```

`CommandHandler::new("start", start)` matches both `/start` and `/start@yourbotname`, which is important for group chats where commands are addressed to specific bots.

## Commands with Arguments

Parse arguments from the message text directly:

```rust
async fn set_timer(update: Arc<Update>, context: Context) -> HandlerResult {
    let msg = update.effective_message().expect("must have a message");
    let chat_id = msg.chat.id;
    let text = msg.text.as_deref().unwrap_or("");

    // Split on whitespace, skip the command itself
    let args: Vec<&str> = text.split_whitespace().skip(1).collect();

    let seconds: u64 = match args.first() {
        Some(s) => match s.parse() {
            Ok(n) if n > 0 => n,
            _ => {
                context.bot()
                    .send_message(chat_id, "Usage: /set <seconds>\nPlease provide a positive number.")
                    .send().await
                    .map_err(|e| HandlerError::Other(Box::new(e)))?;
                return Ok(());
            }
        },
        None => {
            context.bot()
                .send_message(chat_id, "Usage: /set <seconds>")
                .send().await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
            return Ok(());
        }
    };

    context.bot()
        .send_message(chat_id, &format!("Timer set for {seconds} seconds!"))
        .send().await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}
```

## Deep Linking

Deep linking lets you pass parameters through `/start` commands via special URLs. When a user clicks `https://t.me/yourbot?start=my-payload`, Telegram sends `/start my-payload` to your bot.

### Generating Deep Links

```rust
fn create_deep_linked_url(bot_username: &str, payload: &str, group: bool) -> String {
    let param = if group { "startgroup" } else { "start" };
    format!("https://t.me/{bot_username}?{param}={payload}")
}
```

### Routing by Payload

Register deep-link handlers **before** the plain `/start` handler. The first matching handler wins within its group:

```rust
use rust_tg_bot::ext::prelude::{FnHandler, MessageEntityType};

fn is_start_with_payload(update: &Update, payload: &str) -> bool {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return false,
    };
    let text = match msg.text.as_deref() {
        Some(t) => t,
        None => return false,
    };
    let entities = match msg.entities.as_ref() {
        Some(e) => e,
        None => return false,
    };
    let is_cmd = entities.first().map_or(false, |e| {
        e.entity_type == MessageEntityType::BotCommand && e.offset == 0
    });
    is_cmd && text.starts_with("/start") && text.contains(payload)
}

// Register deep link handlers first
app.add_handler(
    FnHandler::new(
        |u| is_start_with_payload(u, "referral-code"),
        handle_referral,
    ), 0,
).await;

// Then the plain /start handler
app.add_handler(
    FnHandler::new(
        |u| { /* matches bare /start only */ },
        start,
    ), 0,
).await;
```

### Sending Deep Links with Inline Keyboards

```rust
use rust_tg_bot::ext::prelude::{InlineKeyboardButton, InlineKeyboardMarkup};

let bot_username = context.bot().bot_data()
    .and_then(|d| d.username.clone())
    .unwrap_or_default();

let url = create_deep_linked_url(&bot_username, "my-payload", false);
let keyboard = InlineKeyboardMarkup::from_button(
    InlineKeyboardButton::url("Open in private chat", url),
);

context.bot()
    .send_message(chat_id, "Click below to continue:")
    .reply_markup(serde_json::to_value(keyboard).unwrap())
    .send().await?;
```

## Registering Commands with BotFather

For the best user experience, register your commands with @BotFather so Telegram shows them in the command menu:

1. Open @BotFather.
2. Send `/setcommands`.
3. Select your bot.
4. Send a list like:

```
start - Start the bot
help - Show help message
settings - Configure settings
set - Set a timer
```

## Next Steps

- [Inline Keyboards](inline-keyboards.md) -- add interactive buttons to your messages.
- [Conversations](conversations.md) -- build multi-step command flows.
