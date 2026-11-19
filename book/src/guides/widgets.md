# HTML Widgets

A widget is an HTML panel the bot sends as a message and whose JavaScript can talk
back -- a small interactive view inside the chat. Bot API 10.4 adds the five
methods, the two update kinds, the inline-result content types, and the field that
carries a widget on an incoming message.

Three constraints shape the design: sent widgets have protected content (they cannot
be forwarded or copied), widgets reach private chats only in the production
environment, and a widget query must be answered within **15 seconds**, with at most
**5 queries per 5 seconds** per user across all of the bot's widgets.

> There is no runnable example for widgets yet: they need a client that renders HTML,
> so this interface is easier to drive by hand than in a loop.

## Sending a widget

```rust
use rust_tg_bot::ext::prelude::*;

let sent = context
    .bot()
    .send_widget(chat_id, "<div id=\"app\">Loading…</div>")
    .width(320)          // logical pixels; clamped to 200..bubble width
    .height(180)         // clamped to 50..600; omit for auto height
    .public_data("{\"version\":1}")  // same everywhere the widget is shown
    .chat_data("{\"draft\":true}")   // specific to this message
    .caption("Your balance")
    .parse_mode("HTML")
    .allow_queries(true) // let the widget talk back
    .await?;
```

The remaining parameters are the usual send options: `disable_notification`,
`protect_content`, `allow_paid_broadcast`, `message_effect_id`,
`reply_parameters`, `reply_markup` and `message_thread_id`.

## Shared widgets

Pass `is_shared(true)` and Telegram creates a *shared* widget: every message sent
from that call gets its own instance, and the bot learns the instance id only from
queries (or from the message's `Widget::id`). A shared widget can be sent again
elsewhere:

```rust
let sent = context
    .bot()
    .send_widget(chat_id, "<div id=\"app\"></div>")
    .is_shared(true)
    .await?;

if let Some(widget_id) = sent.widget.as_ref().and_then(|widget| widget.id.as_deref()) {
    // Later, or in another chat:
    context
        .bot()
        .send_shared_widget(other_chat_id, widget_id)
        .caption("Same widget, different chat")
        .await?;
}
```

## Answering queries

Queries arrive as the `widget_query` update kind (widget in a message the bot sent)
or `shared_widget_query` (widget in a forwarded message). Match them with the
`WIDGET_QUERY` and `SHARED_WIDGET_QUERY` filters and answer with
`answerWidgetQuery`:

```rust
use rust_tg_bot::ext::prelude::*;
use rust_tg_bot::raw::types::widget_query::WidgetQueryType;

app.add_handler(
    MessageHandler::new(F::new(WIDGET_QUERY), on_widget_query),
    0,
)
.await;

async fn on_widget_query(update: Arc<Update>, context: Context) -> HandlerResult {
    let Some(query) = update.widget_query() else {
        return Ok(());
    };

    // `data` is whatever the widget's JavaScript sent, 1-64 bytes.
    // The answer goes only to the widget instance that asked; it does not
    // change the widget's stored data (use the edit methods for that).
    let reply = match &query.query_type {
        WidgetQueryType::Data(_) => "{\"balance\":\"12\"}",
        _ => "{}",
    };

    context
        .bot()
        .answer_widget_query(query.id.as_str())
        .data(reply)
        .await?;
    Ok(())
}
```

`WidgetQuery` carries `id`, `from`, `data`, `widget_id`, `chat_instance`, plus
either `chat` + `message_id` (message widget) or `inline_message_id` (inline
message), and `chat_type` for inline messages.

### Query types

`WidgetQueryType` tells you what the widget sent:

| Variant | Payload | Meaning |
| --- | --- | --- |
| `Data` | `WidgetQueryTypeData` (`wallet_card`) | Plain data for the bot, optionally with the user's TON wallet card |
| `SignData` | `WidgetQueryTypeSignData` (`wallet`, `sign_data`) | Data signed by a TON wallet of the user |
| `SentTransaction` | `WidgetQueryTypeSentTransaction` (`wallet`, `msg_hash`) | Notification that a TON transaction was sent |

`sign_data` is signed for the domain `bot<bot_id>`. The wallet shape
(`WidgetQueryWallet` with `address`, `amount`, `public_key`, `state_init`) is the
same one the [TON wallet](ton-and-share-themes.md) page describes.

## Editing widget data

`editMessageWidgetData` changes data without re-sending the message; `public_data`
is shared by every copy of a shared widget, `chat_data` is per message. Pass an
empty string to clear a field, or omit it to leave it unchanged:

```rust
// Identify the message with chat_id + message_id ...
context
    .bot()
    .edit_message_widget_data()
    .chat_id(chat_id)
    .message_id(message_id)
    .public_data("{\"price\":\"2\"}")
    .await?;

// ... or with inline_message_id.
context
    .bot()
    .edit_message_widget_data()
    .inline_message_id("inline-id")
    .chat_data("{\"draft\":false}")
    .await?;
```

`editSharedWidgetData` changes a shared widget for all of its copies. A shared
widget's public data can be changed at most once per second, and `cache_time`
(5-3600) controls how long clients may cache it:

```rust
context
    .bot()
    .edit_shared_widget_data(widget_id, "{\"price\":\"2\"}")
    .cache_time(60)
    .await?;
```

## Receiving widgets

Incoming widgets surface as `Message::widget` (boxed) and
`ExternalReplyInfo::widget`:

```rust
if let Some(widget) = update.effective_message().and_then(|m| m.widget.as_deref()) {
    // widget.id     -- Some(..) only for shared widgets
    // widget.html   -- the markup Telegram rendered
    // widget.width  -- None when the widget spans the bubble
    let _ = (&widget.id, &widget.html, widget.width);
}
```

Widget messages cannot be copied with `copyMessage` or `copyMessages`.

## Inline results

Inline-query results can carry widgets through two new `InputMessageContent`
variants: `InputWidgetMessageContent` (raw HTML plus the same sizing/caption
options) and `InputSharedWidgetMessageContent` (`widget_id` plus per-message
`chat_data`). Production clients do not render widgets from inline results yet.

```rust
use rust_tg_bot::raw::types::inline::input_message_content::InputMessageContent;
use rust_tg_bot::raw::types::inline::input_widget_message_content::InputWidgetMessageContent;

let content = InputMessageContent::Widget(
    InputWidgetMessageContent::new("<div id=\"app\"></div>")
        .is_shared(true)
        .allow_queries(true),
);
```

## Limits

- `html`: 1-32768 bytes, and at most 4096 bytes when compressed with zlib.
- `width`: clamped to 200 .. message bubble width. `height`: clamped to 50-600.
- `public_data` / `chat_data`: 0-1024 bytes each. `caption`: up to 1024 characters.
- Query `data`: 1-64 bytes; 5 queries per 5 seconds per user; answer within 15 s.
- Shared widget public data: at most one change per second.

## Next Steps

Widget queries can carry a TON wallet card; see
[TON Wallet and Share Themes](ton-and-share-themes.md). Widget updates are handled
like any other update kind -- see [Update](../core-concepts/update.md) and
[Filters](../core-concepts/filters.md). The authoritative field-by-field reference
is the [Bot API page on widgets](https://core.telegram.org/bots/widgets).
