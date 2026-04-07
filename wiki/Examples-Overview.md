# Examples Overview

The repository includes 20 example bots demonstrating every major feature. All examples are located in `crates/telegram-bot/examples/`.

## Running Any Example

```sh
TELEGRAM_BOT_TOKEN="your-token" cargo run -p telegram-bot --example <example_name>
```

For examples requiring additional environment variables or feature flags, the specific requirements are noted below.

---

## Quick Reference

| # | Example | Difficulty | Key Concepts |
|---|---------|-----------|--------------|
| 1 | `echo_bot` | Beginner | CommandHandler, MessageHandler, filters, reply_text |
| 2 | `inline_keyboard` | Beginner | InlineKeyboardMarkup, callback queries, edit_message_text |
| 3 | `inline_keyboard2` | Beginner | Alternative keyboard patterns, multiple button rows |
| 4 | `inline_bot` | Beginner | Inline queries, InlineQueryResultArticle, answer_inline_query |
| 5 | `timer_bot` | Intermediate | JobQueue, scheduled tasks, job cancellation |
| 6 | `conversation_bot` | Intermediate | Multi-step state machine, RwLock state tracking |
| 7 | `conversation_bot2` | Intermediate | Alternative conversation pattern |
| 8 | `persistent_conversation_bot` | Intermediate | JsonFilePersistence, user_data, surviving restarts |
| 9 | `nested_conversation_bot` | Advanced | Nested ConversationHandler, state delegation |
| 10 | `error_handler_bot` | Intermediate | Error handler registration, error reporting to admin |
| 11 | `context_types_bot` | Intermediate | Custom context types, typed data access |
| 12 | `deep_linking` | Intermediate | Deep link parameters via /start payload |
| 13 | `payment_bot` | Intermediate | Invoices, shipping queries, pre-checkout, successful payment |
| 14 | `poll_bot` | Intermediate | Creating polls, handling poll answers |
| 15 | `passport_bot` | Advanced | Telegram Passport data handling |
| 16 | `chat_member_bot` | Intermediate | Tracking join/leave events, member status changes |
| 17 | `custom_webhook_bot` | Advanced | Custom axum server, webhook integration, manual lifecycle |
| 18 | `webapp_bot` | Advanced | Telegram Web Apps integration |
| 19 | `raw_api_bot` | Advanced | Direct raw API access bypassing the handler framework |
| 20 | `arbitrary_callback_data_bot` | Advanced | Arbitrary callback data cache for complex keyboard state |

---

## Detailed Descriptions

### 1. echo_bot

**The "Hello World" of Telegram bots.** Responds to `/start` and `/help` commands and echoes back any text message.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example echo_bot
```

**What you learn:**
- The basic handler signature: `async fn(Update, Context) -> HandlerResult`
- `CommandHandler::new("command", fn)` for matching commands
- `MessageHandler::new(TEXT() & !COMMAND(), fn)` for filtering text messages
- `context.reply_text(&update, text)` convenience method
- `update.effective_user()` for typed access to the sender
- `telegram_bot::run(async { ... })` entry point

---

### 2. inline_keyboard

**Interactive inline buttons.** Sends a message with buttons; handles button presses.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example inline_keyboard
```

**What you learn:**
- Building `InlineKeyboardMarkup` as a JSON value with `serde_json::json!`
- `bot.send_message(chat_id, text).reply_markup(keyboard).send().await?`
- `FnHandler::on_callback_query(fn)` for handling button presses
- `bot.answer_callback_query(&id).send().await?` to dismiss the loading indicator
- `bot.edit_message_text(text).chat_id(id).message_id(id).send().await?` to update messages

---

### 3. inline_keyboard2

**Extended keyboard example.** Demonstrates more complex keyboard layouts and interaction patterns.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example inline_keyboard2
```

---

### 4. inline_bot

**Inline mode.** Users type `@botname query` in any chat and the bot offers text transformations (CAPS, Bold, Italic).

**Prerequisite:** Enable inline mode with `@BotFather` by sending `/setinline`.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example inline_bot
```

**What you learn:**
- Handling `update.inline_query` updates
- Building `InlineQueryResultArticle` JSON objects
- `bot.answer_inline_query(&id, results).send().await?`
- HTML escaping for safe embedding in messages
- `ParseMode::Html` for formatted results

---

### 5. timer_bot

**Scheduled tasks.** Implements `/set <seconds>` to schedule a delayed message and `/unset` to cancel it.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example timer_bot
```

**What you learn:**
- Creating a `JobQueue` and passing it to `ApplicationBuilder::job_queue()`
- `jq.once(callback, duration).name("...").chat_id(id).start().await`
- `job.schedule_removal()` for cancellation
- `context.job_queue.as_ref()` to access the queue in handlers
- Sharing state between handlers with `Arc<RwLock<HashMap>>`

---

### 6. conversation_bot

**Multi-step conversations.** Asks the user for name, age, location, and bio in sequence.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example conversation_bot
```

**What you learn:**
- Manual conversation state tracking with `RwLock<HashMap<i64, ConvState>>`
- Handler groups: commands in group 0, state handlers in group 1
- `try_read()` for non-blocking state checks in predicates
- State transitions by updating the HashMap
- The `Arc::clone` pattern for sharing state across closures

---

### 7. conversation_bot2

**Alternative conversation pattern.** A variation on the conversation bot with different state management.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example conversation_bot2
```

---

### 8. persistent_conversation_bot

**Persistent data collection.** Acts as "Doctor Botter" collecting facts about users. Data survives restarts via JSON file persistence.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example persistent_conversation_bot
```

**What you learn:**
- `JsonFilePersistence::new("prefix", single_file, update_on_flush)`
- `ApplicationBuilder::persistence(Box::new(persistence))`
- `context.user_data().await` to read persisted per-user data
- `context.set_user_data(key, value).await` to write per-user data
- Reply keyboard markup with `json!({"keyboard": [...], "resize_keyboard": true})`
- Removing reply keyboards with `json!({"remove_keyboard": true})`

---

### 9. nested_conversation_bot

**Nested state machines.** Demonstrates how one conversation can delegate to a child conversation and receive state back.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example nested_conversation_bot
```

**What you learn:**
- `ConversationHandler::builder()` with `.entry_point()`, `.state()`, `.fallback()`
- `ConversationResult::NextState`, `ConversationResult::End`, `ConversationResult::Stay`
- `map_to_parent` for returning from nested conversations
- Complex handler hierarchies

---

### 10. error_handler_bot

**Error handling.** Registers a custom error handler that logs errors and sends diagnostics to a developer chat.

```sh
TELEGRAM_BOT_TOKEN="..." DEVELOPER_CHAT_ID="..." cargo run -p telegram-bot --example error_handler_bot
```

**What you learn:**
- `app.add_error_handler(callback, block).await`
- `context.error` inside error handlers
- Returning `HandlerError::Other(Box::new(e))` from handlers
- Error message truncation for the Telegram 4096-character limit

---

### 11. context_types_bot

**Custom context types.** Shows how to configure custom data types for the application context.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example context_types_bot
```

---

### 12. deep_linking

**Deep links.** Handles `/start <payload>` where the payload is a deep link parameter.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example deep_linking
```

**What you learn:**
- Deep link URLs: `https://t.me/botusername?start=payload`
- Extracting the payload from `context.args`

---

### 13. payment_bot

**Telegram Payments.** Full payment flow with invoices, shipping queries, and pre-checkout queries.

**Prerequisite:** Get a payment provider token from `@BotFather`.

```sh
TELEGRAM_BOT_TOKEN="..." PAYMENT_PROVIDER_TOKEN="..." cargo run -p telegram-bot --example payment_bot
```

**What you learn:**
- `bot.send_invoice(chat_id, title, desc, payload, currency, prices).provider_token(&t)`
- `.need_name(true).need_email(true).is_flexible(true)` for shipping
- `FnHandler::on_shipping_query(fn)` and `FnHandler::on_pre_checkout_query(fn)`
- `bot.answer_shipping_query(&id, ok).shipping_options(opts)`
- `bot.answer_pre_checkout_query(&id, ok)`
- Detecting successful payments via `message.successful_payment`

---

### 14. poll_bot

**Polls.** Creates polls and handles poll answers.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example poll_bot
```

**What you learn:**
- `FnHandler::on_poll(fn)` and `FnHandler::on_poll_answer(fn)`

---

### 15. passport_bot

**Telegram Passport.** Handles encrypted identity data from Telegram Passport.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example passport_bot
```

---

### 16. chat_member_bot

**Member tracking.** Monitors join/leave events in groups.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example chat_member_bot
```

**What you learn:**
- `FnHandler::on_chat_member(fn)` and `FnHandler::on_my_chat_member(fn)`
- `ChatMemberUpdated` type with `old_chat_member` and `new_chat_member`

---

### 17. custom_webhook_bot

**Custom web server.** Runs an axum web server alongside the bot with custom endpoints.

**Requires the `webhooks` feature.**

```sh
TELEGRAM_BOT_TOKEN="..." WEBHOOK_URL="https://..." ADMIN_CHAT_ID="..." PORT="8000" \
  cargo run -p telegram-bot --example custom_webhook_bot --features webhooks
```

**What you learn:**
- Manual lifecycle: `app.initialize()`, `app.start()`, `app.stop()`, `app.shutdown()`
- `app.update_sender()` to feed updates from a custom endpoint
- `bot.set_webhook(&url).send().await` to register the webhook
- Running axum alongside the bot with `tokio::spawn` and graceful shutdown
- `bot.get_chat_member(chat_id, user_id)` for resolving user info
- HTML escaping for safe message construction

---

### 18. webapp_bot

**Web Apps.** Integrates with Telegram Web Apps (Mini Apps).

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example webapp_bot
```

---

### 19. raw_api_bot

**Raw API access.** Bypasses the handler framework and makes direct Bot API calls.

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example raw_api_bot
```

**What you learn:**
- Using `telegram_bot::raw::bot::Bot` directly
- Making arbitrary API calls without the Application framework

---

### 20. arbitrary_callback_data_bot

**Complex callback data.** Uses the arbitrary callback data cache to store rich data behind inline keyboard buttons (bypassing the 64-byte callback_data limit).

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p telegram-bot --example arbitrary_callback_data_bot
```

**What you learn:**
- `ApplicationBuilder::arbitrary_callback_data(maxsize)`
- Storing complex data structures behind button callbacks
- `context.drop_callback_data(callback_query_id)` for cache cleanup

---

## Progression Path

If you are new to the framework, work through the examples in this order:

1. **echo_bot** -- Learn the basics
2. **inline_keyboard** -- Add interactivity
3. **conversation_bot** -- Multi-step flows
4. **timer_bot** -- Scheduled tasks
5. **error_handler_bot** -- Robust error handling
6. **persistent_conversation_bot** -- Data persistence
7. **payment_bot** -- Payments
8. **custom_webhook_bot** -- Production deployment
