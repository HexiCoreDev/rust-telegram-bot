# Update

Every interaction with your bot -- a message, a button press, an inline query, a payment -- arrives as an `Update`. Understanding the `Update` type is fundamental to working with this framework.

## The Update Struct

An `Update` is a strongly typed Rust struct that mirrors the [Telegram Bot API Update object](https://core.telegram.org/bots/api#update). It contains an `update_id` and exactly one of many optional fields indicating what type of event occurred.

```rust
use telegram_bot::ext::prelude::Update;
```

## Accessing Update Fields

The `Update` type provides convenience methods to avoid deeply nested `Option` chains:

### effective_message()

Returns the `Message` regardless of whether the update is a regular message, edited message, channel post, or edited channel post:

```rust
async fn handler(update: Arc<Update>, context: Context) -> HandlerResult {
    if let Some(msg) = update.effective_message() {
        let chat_id = msg.chat.id;
        let text = msg.text.as_deref().unwrap_or("");
        // ...
    }
    Ok(())
}
```

### effective_user()

Returns the `User` who triggered the update, regardless of the update type:

```rust
let user_name = update
    .effective_user()
    .map(|u| u.first_name.as_str())
    .unwrap_or("Unknown");
```

### effective_chat()

Returns the `Chat` the update originated from:

```rust
let chat_id = update
    .effective_chat()
    .map(|c| c.id)
    .expect("update must have a chat");
```

### Type-Specific Accessors

For updates that are not messages, use the specific accessors:

```rust
// Callback query (inline keyboard button press)
if let Some(cq) = update.callback_query() {
    let data = cq.data.as_deref().unwrap_or("");
    // ...
}

// Inline query
if let Some(iq) = update.inline_query() {
    let query_text = &iq.query;
    // ...
}

// Shipping query (payment flow)
if let Some(sq) = update.shipping_query() {
    // ...
}

// Pre-checkout query (payment flow)
if let Some(pcq) = update.pre_checkout_query() {
    // ...
}
```

## Update Types

The Telegram Bot API defines many update types. Here are the most common:

| Update Type | Accessor | When It Fires |
|---|---|---|
| Message | `effective_message()` | User sends a text, photo, sticker, etc. |
| Edited Message | `effective_message()` | User edits an existing message |
| Channel Post | `effective_message()` | New post in a channel the bot is in |
| Callback Query | `callback_query()` | User presses an inline keyboard button |
| Inline Query | `inline_query()` | User types `@yourbot query` in any chat |
| Chosen Inline Result | `chosen_inline_result()` | User selects an inline query result |
| Shipping Query | `shipping_query()` | Payment: user selected a shipping address |
| Pre-Checkout Query | `pre_checkout_query()` | Payment: final confirmation before charging |
| Poll | `poll()` | Poll state changes |
| Poll Answer | `poll_answer()` | User votes in a poll |
| Chat Member | `my_chat_member()` / `chat_member()` | Bot or user's membership status changes |
| Chat Join Request | `chat_join_request()` | User requests to join a chat |

## The Arc Wrapper

Handlers receive `Arc<Update>` rather than `Update` directly:

```rust
async fn my_handler(update: Arc<Update>, context: Context) -> HandlerResult {
    // ...
}
```

The `Arc` (atomic reference count) allows the update to be shared across multiple handler groups and async tasks without copying. You access it exactly like a regular reference -- all the methods above work through `Arc`'s `Deref` implementation.

## Message Fields

When you have a `Message` (from `effective_message()`), commonly used fields include:

```rust
if let Some(msg) = update.effective_message() {
    // Sender info
    let chat_id: i64 = msg.chat.id;
    let from: Option<&User> = msg.from.as_ref();

    // Content
    let text: Option<&str> = msg.text.as_deref();
    let entities: Option<&Vec<MessageEntity>> = msg.entities.as_ref();

    // Media
    let photo: Option<&Vec<PhotoSize>> = msg.photo.as_ref();
    let document: Option<&Document> = msg.document.as_ref();

    // Special messages
    let successful_payment = msg.successful_payment.as_ref();
    let new_chat_members = msg.new_chat_members.as_ref();
}
```

## Next Steps

Now that you understand `Update`, learn how to interact with the Telegram API through the [Bot](bot.md) object.
