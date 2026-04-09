# Telegram Bot API Overview

A quick primer on how the Telegram Bot API works, tailored for `rust-tg-bot` developers.

---

## How Bots Communicate

Telegram bots communicate through an HTTPS-based API hosted at `https://api.telegram.org/bot<TOKEN>/`. Every interaction follows a request-response cycle:

1. **Telegram sends updates to your bot** -- either via long polling (your bot asks "any new updates?") or via webhooks (Telegram pushes them to your server).
2. **Your bot processes the update** and optionally responds by calling Bot API methods (send a message, edit a message, answer a query, etc.).

All data is transferred as JSON. The `rust-tg-bot` framework handles serialization, deserialization, HTTP transport, and retry logic for you.

---

## Updates

An Update is the atomic unit of communication from Telegram to your bot. Each update contains exactly one of these fields:

| Update Type | Field | When It Arrives |
|-------------|-------|-----------------|
| Message | `message` | User sends a message (text, photo, sticker, etc.) |
| Edited Message | `edited_message` | User edits an existing message |
| Channel Post | `channel_post` | New post in a channel where the bot is admin |
| Edited Channel Post | `edited_channel_post` | Channel post was edited |
| Inline Query | `inline_query` | User types `@botname query` in any chat |
| Chosen Inline Result | `chosen_inline_result` | User selected an inline query result |
| Callback Query | `callback_query` | User pressed an inline keyboard button |
| Shipping Query | `shipping_query` | Payment: shipping address step |
| Pre-Checkout Query | `pre_checkout_query` | Payment: confirmation step |
| Poll | `poll` | Poll state changed |
| Poll Answer | `poll_answer` | User voted in a poll |
| Chat Member | `chat_member` | Someone's status in a chat changed |
| My Chat Member | `my_chat_member` | The bot's own status changed |
| Chat Join Request | `chat_join_request` | Someone asked to join a chat |

In `rust-tg-bot`, the `Update` struct provides these as `Option<T>` fields. Convenience methods search across types:

```rust
update.effective_user()     // works for messages, callbacks, inline queries, etc.
update.effective_chat()     // works for messages, callbacks, channel posts
update.effective_message()  // works for message, edited_message, channel_post, etc.
```

---

## Messages

A Message is the richest type in the Bot API. It can contain:

| Content | Message Field | Description |
|---------|---------------|-------------|
| Text | `text` | Plain or formatted text |
| Entities | `entities` | Bold, italic, links, commands, etc. |
| Photo | `photo` | One or more photo sizes |
| Document | `document` | Any file |
| Audio | `audio` | Audio file |
| Video | `video` | Video file |
| Location | `location` | GPS coordinates |
| Contact | `contact` | Phone contact |
| Sticker | `sticker` | A sticker |
| Poll | `poll` | An attached poll |
| Invoice | `invoice` | A payment invoice |
| Successful Payment | `successful_payment` | Payment completion confirmation |

A message also has metadata:
- `chat` -- which chat it belongs to (with `chat.id` and `chat.type`)
- `from` -- who sent it (with `from.id` and `from.first_name`)
- `date` -- Unix timestamp
- `message_id` -- unique within the chat
- `reply_to_message` -- if it is a reply

---

## Bot API Methods

The Bot API provides methods your bot calls to interact with Telegram. In `rust-tg-bot`, every method is a builder on `context.bot()`:

### Messaging

| Method | Rust API | Description |
|--------|----------|-------------|
| `sendMessage` | `bot.send_message(chat_id, text)` | Send a text message |
| `editMessageText` | `bot.edit_message_text(text).chat_id(id).message_id(id)` | Edit an existing message |
| `deleteMessage` | `bot.delete_message(chat_id, message_id)` | Delete a message |
| `forwardMessage` | `bot.forward_message(chat_id, from_chat_id, message_id)` | Forward a message |
| `sendPhoto` | `bot.send_photo(chat_id, photo)` | Send a photo |
| `sendDocument` | `bot.send_document(chat_id, document)` | Send a file |
| `sendLocation` | `bot.send_location(chat_id, lat, lon)` | Send a location |

### Queries

| Method | Rust API | Description |
|--------|----------|-------------|
| `answerCallbackQuery` | `bot.answer_callback_query(id)` | Acknowledge a button press |
| `answerInlineQuery` | `bot.answer_inline_query(id, results)` | Respond to an inline query |

### Payments

| Method | Rust API | Description |
|--------|----------|-------------|
| `sendInvoice` | `bot.send_invoice(chat_id, title, desc, payload, currency, prices)` | Send a payment invoice |
| `answerShippingQuery` | `bot.answer_shipping_query(id, ok)` | Respond to shipping step |
| `answerPreCheckoutQuery` | `bot.answer_pre_checkout_query(id, ok)` | Approve or reject payment |

### Chat Management

| Method | Rust API | Description |
|--------|----------|-------------|
| `getChatMember` | `bot.get_chat_member(chat_id, user_id)` | Get a member's status |
| `setChatTitle` | `bot.set_chat_title(chat_id, title)` | Change group title |
| `banChatMember` | `bot.ban_chat_member(chat_id, user_id)` | Ban a user |
| `unbanChatMember` | `bot.unban_chat_member(chat_id, user_id)` | Unban a user |

### Webhooks

| Method | Rust API | Description |
|--------|----------|-------------|
| `setWebhook` | `bot.set_webhook(url)` | Tell Telegram where to send updates |
| `deleteWebhook` | `bot.delete_webhook()` | Remove the webhook |
| `getWebhookInfo` | `bot.get_webhook_info()` | Check current webhook status |

All builders follow the same pattern: call the method, chain optional parameters, and finish with `.send().await?`.

---

## Common Builder Parameters

Most send methods accept these optional parameters:

| Parameter | Description | Example |
|-----------|-------------|---------|
| `parse_mode` | Text formatting mode | `.parse_mode(ParseMode::Html)` |
| `reply_markup` | Keyboard (inline or reply) | `.reply_markup(json!({...}))` |
| `disable_notification` | Silent message | `.disable_notification(true)` |
| `reply_to_message_id` | Reply to a specific message | `.reply_to_message_id(msg_id)` |
| `protect_content` | Prevent forwarding/saving | `.protect_content(true)` |

---

## Typed Constants

The framework provides typed Rust enums for common string constants in the Bot API:

```rust
// Parse modes
ParseMode::Html
ParseMode::MarkdownV2

// Chat types
ChatType::Private
ChatType::Group
ChatType::Supergroup
ChatType::Channel

// Message entity types
MessageEntityType::BotCommand
MessageEntityType::Bold
MessageEntityType::Italic
MessageEntityType::Url
MessageEntityType::Code
MessageEntityType::Pre
MessageEntityType::Mention
MessageEntityType::Hashtag

// Chat actions (typing indicators)
ChatAction::Typing
ChatAction::UploadPhoto
ChatAction::RecordVideo

// Chat member status
ChatMemberStatus::Creator
ChatMemberStatus::Administrator
ChatMemberStatus::Member
ChatMemberStatus::Restricted
ChatMemberStatus::Left
ChatMemberStatus::Kicked
```

Using typed constants prevents typos and provides compile-time verification.

---

## Rate Limits

Telegram enforces rate limits on Bot API calls:

- **Per chat**: No more than 1 message per second per chat.
- **Per bot**: No more than 30 messages per second overall.
- **Group messages**: No more than 20 messages per minute to the same group.
- **Bulk notifications**: For sending messages to many users, Telegram recommends spreading them over time (about 30 messages per second).

If you exceed these limits, the API returns a 429 error with a `retry_after` value. The framework's HTTP layer handles basic retries, but for high-volume bots you should implement your own queue.

---

## Further Reading

- [Official Telegram Bot API documentation](https://core.telegram.org/bots/api)
- [Telegram Bot FAQ](https://core.telegram.org/bots/faq)
- [Bot API changelog](https://core.telegram.org/bots/api-changelog)
