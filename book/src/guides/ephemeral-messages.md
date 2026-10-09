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
