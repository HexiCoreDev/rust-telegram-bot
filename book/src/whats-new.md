# What's New

`rust-tg-bot` tracks the [Telegram Bot API](https://core.telegram.org/bots/api-changelog) release by
release. This page is the short history; the itemised record is
[CHANGELOG.md](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/CHANGELOG.md).
The coverage audit for the current spec is reproducible with `make tg-api-parity`
(scraped snapshot plus a method-by-method and field-by-field diff against the crate).

## Release history

| Release | Bot API | What landed |
| --- | --- | --- |
| **1.0.0** (current) | **10.4** | HTML widgets, TON wallet cards, chat share themes, ephemeral replies without `chat_id`, rich-message quotes |
| 1.0.0-rc.4 | 10.0 – 10.3 | live photos, guest mode, poll media, rich messages, ephemeral messages and parameters, communities, rich buttons |
| 1.0.0-beta | 9.6 | first complete surface: 169 methods, 301 types |
| earlier betas | 9.5 | port of python-telegram-bot v22: Application, handlers, filters, conversations, job queue, persistence |

## 1.0.0 — Bot API 10.4

This is the first stable release: the public API is now covered by semver. It is a drop-in upgrade
from `1.0.0-rc.4` apart from the short list under [Upgrade notes](#upgrade-notes). Every method and
type in Bot API 10.4 is implemented — 193 methods, 440 types — and `cargo audit` reports no
outstanding advisories against the lockfile.

### HTML widgets

Bots can send interactive HTML panels and receive queries from the JavaScript inside them.
Five methods (`sendWidget`, `sendSharedWidget`, `answerWidgetQuery`,
`editMessageWidgetData`, `editSharedWidgetData`), two update kinds (`widget_query`,
`shared_widget_query`, with the matching `WIDGET_QUERY`/`SHARED_WIDGET_QUERY` filters), the
`Widget`-bearing message fields, and inline-result widget content.

```rust
use rust_tg_bot::ext::prelude::*;

// The reply is delivered only to the widget instance that asked.
context
    .bot()
    .answer_widget_query(query_id)
    .data("{\"balance\":\"12\"}")
    .await?;
```

Guide: [HTML Widgets](guides/widgets.md) — sending, shared widgets, query types, editing data,
limits.

### Ephemeral replies without a `chat_id`

An ephemeral reply addressed by `callback_query_id` is routed by the callback query, so
`chat_id` is no longer required on the 14 `send*` methods that accept
`ephemeral_message_parameters`, and `EphemeralMessageParameters::receiver_user_id` is optional:

```rust
context
    .bot()
    .send_message_to_callback_query(cbq.id.as_str(), "Only you can see this")
    .await?;
```

Guide: [Ephemeral Messages](guides/ephemeral-messages.md) — the full flow, including the
`*_to_callback_query` constructors and the classic `receiver_user_id` path.

### Quotes from rich messages

`TextQuote` gained `rich_texts` (the quoted fragment as rich text) and `ReplyParameters` gained
`quote_rich_texts`, so a quote keeps its formatting:

```rust
use rust_tg_bot::raw::types::reply::ReplyParameters;

let mut reply = ReplyParameters::new(message_id);
if let Some(quote) = incoming.quote.as_ref() {
    reply.quote_rich_texts = quote.rich_texts.clone();
}

context
    .bot()
    .send_message(chat_id, "Replying to that")
    .reply_parameters(reply)
    .await?;
```

Setting `quote_rich_texts` makes Telegram ignore `quote`, `quote_parse_mode` and `quote_entities`.

### TON wallet and chat share themes

`ChatFullInfo` now carries `ton_wallet_card` (address, balance, NFT stickers) and `share_theme`
(the screen shown when someone shares a link to the user or chat). Transfers arrive on
`Message::ton_wallet_transfer`, TON addresses format as `RichTextTonAddress` and
`MessageEntityType::TonAddress`, and `getChatShareThemeBackdrops` lists the available backgrounds.
The position type formerly named `StoryAreaPosition` is now `AreaPosition`.

Guide: [TON Wallet and Share Themes](guides/ton-and-share-themes.md) — cards, stickers, transfers,
themes, backdrops.

### Gifts, payments and verification

- `getUniqueGift(name)` looks up a unique gift by its `t.me/nft` name.
- `deleteBusinessAccountGift(business_connection_id, owned_gift_id)` deletes a regular gift owned by
  a managed business account (requires the `can_convert_gifts_to_stars` right).
- `Message::paid_messages_waived` (`PaidMessagesWaived`) marks a service message about someone
  using a link that waives the paid-message fee.
- `verifyUser` and `verifyChat` accept `custom_description_parse_mode` and
  `custom_description_entities`; only `text_link` entities are supported.

### Platform changes in 1.0.0

- **MSRV is 1.94.** `sqlx` 0.9.1 — needed to reach the patched `libsqlite3-sys` — declares Rust 1.94.
- **All advisories cleared.** `rustls`, `rustls-webpki`, `webpki-roots`, `quinn-proto`,
  `quinn-udp`, `tokio` and `libsqlite3-sys` were bumped; `cargo audit` now exits 0 with only the
  pre-existing, documented `rsa` ignore for an unreachable dependency.
- **Dynamic SQL is asserted.** `sqlx` 0.9 requires `AssertSqlSafe` for non-`'static` statements;
  the Postgres persistence backend wraps each statement after validating that only the table prefix
  is interpolated.
- **Two wire-fidelity bugs fixed**: `ChatOwnerChanged` and `ChatOwnerLeft` were empty marker structs
  and silently dropped `new_owner` in both directions.

## Upgrade notes

```toml
[dependencies]
rust-tg-bot = "1.0.0"
```

| Change | Action |
| --- | --- |
| MSRV 1.88 → **1.94** | Update `rust-toolchain` if you pin one |
| `StoryAreaPosition` → **`AreaPosition`** | Import `types::area_position::AreaPosition`; the old name still resolves as a `#[deprecated]` alias |
| `EphemeralMessageParameters::receiver_user_id` is `Option<i64>` | `new(user_id)` is unchanged; only struct literals need `Some(..)` |
| `verify_user_raw` / `verify_chat_raw` gained two parameters | Pass the custom-description parse mode and entities |
| `Message::widget`, `Message::ton_wallet_transfer`, `Message::chat_owner_changed`/`_left` are boxed | `.as_ref()` usage is unchanged; only pattern matches that bind `Box<_>` need adjusting |
| `ChatOwnerChanged` / `ChatOwnerLeft` now carry `new_owner` | Code that constructed them as unit structs must pass the user |

## Earlier Bot API versions

Content and methods that arrived before 1.0.0, documented in the guides:

- **10.3** — `EphemeralMessageParameters` on all 14 ephemeral-capable sends, `DisabledButton`,
  `MessageGenerationStopped`, `can_send_welcome_messages`, compact tables and document blocks in
  rich messages.
- **10.2** — the five `editEphemeralMessage*`/`deleteEphemeralMessage` methods, communities
  (`Community`, `community_chat_*`), and input rich blocks for inline results.
- **10.1** — rich messages: `sendRichMessage`, `RichText` (26 members), `RichBlock` (20 blocks),
  guard bots, join-request queries. See [Rich Messages](guides/rich-messages.md).
- **10.0** — live photos (`sendLivePhoto`), guest mode (`answerGuestQuery`,
  `getUserPersonalChatMessages`), poll media, managed-bot access settings. See
  [Ephemeral Messages](guides/ephemeral-messages.md).

## See also

- [Bot](core-concepts/bot.md) — how builders and `IntoFuture` work for every method.
- [Update](core-concepts/update.md) — the update kinds added above.
- [HTML Widgets](guides/widgets.md), [TON Wallet and Share Themes](guides/ton-and-share-themes.md),
  [Rich Messages](guides/rich-messages.md), [Ephemeral Messages](guides/ephemeral-messages.md).
- [API Reference](api-reference.md) — where each type lives.
