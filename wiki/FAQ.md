# Frequently Asked Questions

---

### General

**Q: What Rust version do I need?**

A: Rust 1.75 or later. The framework uses `async fn` in traits, which was stabilized in Rust 1.75. Run `rustup update` to get the latest stable release.

**Q: Do I need to use `#[tokio::main]`?**

A: No. Use `telegram_bot::run(async { ... })` instead. This creates a multi-threaded tokio runtime with 8MB thread stacks, which prevents stack overflow errors caused by the deeply nested async state machines in the Bot API call chain. If you prefer to manage your own runtime, configure it with `thread_stack_size(8 * 1024 * 1024)`.

**Q: Can I use this in production?**

A: Yes. The framework is designed for production use. Use release builds, webhook mode, and structured logging. See the [Production Deployment](../docs/ZERO_TO_HERO.md#part-5-production-deployment) section of the Zero to Hero guide.

**Q: How does this compare to other Rust Telegram libraries?**

A: See the [Comparison with Python](Comparison-with-Python) page for a detailed comparison with python-telegram-bot. Compared to other Rust libraries like `teloxide`, this framework takes a different approach: it mirrors the python-telegram-bot API almost exactly, making it easy for PTB users to transition. The handler/filter/context architecture is the same, just written in Rust.

---

### API and Handlers

**Q: How do I handle multiple commands?**

A: Register a `CommandHandler` for each command, all in the same group:

```rust
app.add_typed_handler(CommandHandler::new("start", start_fn), 0).await;
app.add_typed_handler(CommandHandler::new("help", help_fn), 0).await;
app.add_typed_handler(CommandHandler::new("settings", settings_fn), 0).await;
```

**Q: How do I access command arguments?**

A: The `CommandHandler` parses arguments and stores them in `context.args`:

```rust
// User sends: /set 30 hello
async fn set(update: Update, context: Context) -> HandlerResult {
    if let Some(args) = &context.args {
        // args = ["30", "hello"]
        let value = &args[0];
    }
    Ok(())
}
```

**Q: How do I handle callback queries (button presses)?**

A: Use `FnHandler::on_callback_query`:

```rust
async fn button(update: Update, context: Context) -> HandlerResult {
    let cq = update.callback_query.as_ref().unwrap();
    let data = cq.data.as_deref().unwrap_or("");

    context.bot().answer_callback_query(&cq.id).send().await?;
    // Process the button press...
    Ok(())
}

app.add_typed_handler(FnHandler::on_callback_query(button), 0).await;
```

Always call `answer_callback_query` to dismiss the loading indicator.

**Q: How do I send a message with formatting?**

A: Use the builder pattern with `parse_mode`:

```rust
context
    .bot()
    .send_message(chat_id, "<b>Bold</b> and <i>italic</i>")
    .parse_mode(ParseMode::Html)
    .send()
    .await?;
```

Supported parse modes: `ParseMode::Html`, `ParseMode::MarkdownV2`.

**Q: How do I reply with a keyboard?**

A: Pass a JSON keyboard to `reply_markup`:

```rust
use serde_json::json;

let keyboard = json!({
    "inline_keyboard": [
        [{"text": "Yes", "callback_data": "yes"}, {"text": "No", "callback_data": "no"}]
    ]
});

context
    .bot()
    .send_message(chat_id, "Choose:")
    .reply_markup(keyboard)
    .send()
    .await?;
```

For reply keyboards (below the text input):

```rust
let keyboard = json!({
    "keyboard": [
        [{"text": "Option A"}, {"text": "Option B"}]
    ],
    "resize_keyboard": true,
    "one_time_keyboard": true,
});
```

To remove a reply keyboard:

```rust
context
    .bot()
    .send_message(chat_id, "Keyboard removed.")
    .reply_markup(json!({"remove_keyboard": true}))
    .send()
    .await?;
```

**Q: How do I send files/photos?**

A: Use the appropriate Bot API method:

```rust
// Send by URL
context.bot().send_photo(chat_id, "https://example.com/photo.jpg").send().await?;

// Send by file_id (if you have a previously uploaded file ID)
context.bot().send_document(chat_id, file_id).send().await?;
```

**Q: What is the difference between `context.reply_text()` and `context.bot().send_message()`?**

A: `reply_text` is a convenience method that extracts the chat ID from the update and calls `send_message`. Use it for quick text-only replies. Use `bot().send_message()` directly when you need formatting, keyboards, or any other options:

```rust
// Quick reply
context.reply_text(&update, "Simple text").await?;

// Full control
context.bot().send_message(chat_id, "<b>Formatted</b>")
    .parse_mode(ParseMode::Html)
    .reply_markup(keyboard)
    .send()
    .await?;
```

---

### Data and Persistence

**Q: How do I store data that persists across bot restarts?**

A: Configure a persistence backend:

```rust
use telegram_bot::ext::persistence::json_file::JsonFilePersistence;

let persistence = JsonFilePersistence::new("my_bot", true, false);
let app = ApplicationBuilder::new()
    .token(token)
    .persistence(Box::new(persistence))
    .build();
```

After this, `context.user_data()`, `context.chat_data()`, and `context.bot_data()` will be automatically loaded from and saved to disk.

**Q: What is the difference between bot_data, user_data, and chat_data?**

A: They have different scopes:

| Store | Scope | Access |
|-------|-------|--------|
| `bot_data` | Global, shared across all users and chats | `context.bot_data()` / `context.bot_data_mut()` |
| `user_data` | Per user (keyed by user ID) | `context.user_data()` / `context.set_user_data()` |
| `chat_data` | Per chat (keyed by chat ID) | `context.chat_data()` / `context.set_chat_data()` |

Use `bot_data` for global state (configuration, counters). Use `user_data` for per-user preferences or progress. Use `chat_data` for per-group settings.

---

### Deployment

**Q: How do I switch from polling to webhooks?**

A: See the [Webhook Bot](../docs/ZERO_TO_HERO.md#webhook-bot-for-production-deployment) section. The key difference is that you manage the lifecycle manually (`initialize`, `start`, `stop`, `shutdown`) and run your own HTTPS server that feeds updates into the Application via `app.update_sender()`.

**Q: What hosting platforms work?**

A: Any platform that can run a binary and expose an HTTPS port works for webhooks:

- **Railway, Fly.io, Render**: Deploy directly from Git. TLS is automatic.
- **AWS (ECS, Lambda)**: Use the Docker image. For Lambda, compile with `musl`.
- **VPS (DigitalOcean, Hetzner)**: Use Docker or systemd. Put nginx/Caddy in front for TLS.
- **Local machine**: Use polling mode. Webhooks require a public HTTPS endpoint.

**Q: How do I run the bot as a systemd service?**

A: Create `/etc/systemd/system/my-bot.service`:

```ini
[Unit]
Description=My Telegram Bot
After=network.target

[Service]
Type=simple
User=bot
Environment=TELEGRAM_BOT_TOKEN=your-token
Environment=RUST_LOG=info
ExecStart=/usr/local/bin/my_bot
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Then:

```sh
sudo systemctl daemon-reload
sudo systemctl enable --now my-bot
sudo journalctl -u my-bot -f  # view logs
```

---

### Troubleshooting

See the dedicated [Troubleshooting](Troubleshooting) page for common issues and their solutions.
