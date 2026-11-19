# Ephemeral Messages and Guest Mode

Some conversations should not clutter the group: a bot answering one member,
or a bot summoned into a chat it never joined. Ephemeral messages are visible
only to one user and the bot, and delivery is best-effort (offline users may
never receive them).

See the runnable `ephemeral_bot` example
(`crates/telegram-bot/examples/ephemeral_bot.rs`).

## Sending an ephemeral reply

The `reply_ephemeral` shortcut targets one user in the update's chat:

```rust
let user_id = update.effective_user().map(|u| u.id).unwrap();
context.reply_ephemeral(&update, user_id, "Only you can see this.").await?;
```

Underneath it builds `EphemeralMessageParameters` for `sendMessage`. For
manual control, set the parameter object on any send builder:

```rust
use rust_tg_bot::raw::types::ephemeral::EphemeralMessageParameters;

context
    .bot()
    .send_message(chat_id, "Only you can see this.")
    .ephemeral_message_parameters(EphemeralMessageParameters::new(user_id))
    .await?;
```

## Answering from a button press

When the update carries a callback query, `reply_ephemeral` attaches its id
automatically so the message routes to the pressing client. To replace the
original message instead, use `.replace_original()` — except for callback
queries that themselves came from ephemeral messages, which must be edited
with the `editEphemeralMessage*` methods.

## Replying without a `chat_id` (Bot API 10.4)

When the reply is addressed by a callback query, the Bot API routes it without
any `chat_id`: `receiver_user_id` is optional in `EphemeralMessageParameters`,
and Telegram ignores `chat_id` when it is omitted. Every send method that takes
`ephemeral_message_parameters` therefore has a dedicated constructor:

```rust
use rust_tg_bot::raw::types::ephemeral::EphemeralMessageParameters;

if let Some(cbq) = update.callback_query() {
    // No chat_id, no manual parameter object.
    context
        .bot()
        .send_message_to_callback_query(cbq.id.as_str(), "Only you can see this")
        .await?;

    // Same thing, with the full builder (parse_mode, reply_markup, ...).
    let params = EphemeralMessageParameters::reply_to_callback_query(cbq.id.as_str());
    let chat_id = update.effective_chat().map(|chat| chat.id).unwrap_or_default();
    context
        .bot()
        .send_message(chat_id, "Only you can see this")
        .parse_mode("HTML")
        .ephemeral_message_parameters(params)
        .await?;
}
```

The `*_to_callback_query` constructors exist for all fourteen send methods
(`sendMessage`, `sendRichMessage`, `sendAnimation`, `sendAudio`, `sendContact`,
`sendDocument`, `sendLivePhoto`, `sendLocation`, `sendPhoto`, `sendSticker`,
`sendVenue`, `sendVideo`, `sendVideoNote`, `sendVoice`). Passing a `chat_id`
to the ordinary constructors remains correct — it is simply ignored while
`receiver_user_id` is unset — so existing code keeps working.

The callback query still has to be answered with `answerCallbackQuery`.

## Guest mode

A bot with guest support can be summoned into chats it is not a member of.
Such messages arrive as the `guest_message` update kind carrying a
`guest_query_id`. Match them with the `GUEST_MESSAGE` filter and answer with
an inline result:

```rust
use rust_tg_bot::ext::filters::update_type::GUEST_MESSAGE;
use rust_tg_bot::ext::prelude::{F, MessageHandler};

app.add_handler(
    MessageHandler::new(F::new(GUEST_MESSAGE), answer_guest),
    0,
)
.await;

async fn answer_guest(update: Arc<Update>, context: Context) -> HandlerResult {
    let query_id = match &update.kind {
        UpdateKind::GuestMessage(msg) => msg.guest_query_id.clone().unwrap_or_default(),
        _ => return Ok(()),
    };
    let result = json!({
        "type": "article",
        "id": "1",
        "title": "Hello, guest",
        "input_message_content": {"message_text": "Thanks for summoning me!"},
    });
    context.bot().answer_guest_query(query_id, result).await?;
    Ok(())
}
```

## Delivery guarantees

Both ephemeral and guest flows are at-least-once on Telegram's side and
best-effort on delivery. The framework deduplicates redeliveries (including
across restarts when persistence is configured), but handlers with
side effects should still key on `update_id`.

## Next Steps

Ephemeral replies are just Bot API calls, so everything in
[Bot](../core-concepts/bot.md) applies. For the structured message format, see
[Rich Messages](rich-messages.md); for the exact delivery semantics of each
field, the [Bot API reference](https://core.telegram.org/bots/api#sendmessage)
is authoritative.

## Runnable examples

The code above is abridged. These run in full:

| Example | What to look at |
|---|---|
| [`ephemeral_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/ephemeral_bot.rs) | messages that delete themselves |

Every example is listed in the [example index](../examples.md).
