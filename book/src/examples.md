# Examples

Every example lives in
[`crates/telegram-bot/examples/`](https://github.com/HexiCoreDev/rust-telegram-bot/tree/main/crates/telegram-bot/examples)
and is runnable. They are the same code that appears throughout this book, so
what you read here is exactly what compiles.

## Running an example

Every bot needs a token. Get one from [@BotFather](https://t.me/BotFather).

```sh
export TELEGRAM_BOT_TOKEN="123456:ABC-your-token"
cargo run -p rust-tg-bot --example echo_bot
```

Examples that need a feature must be run with it:

```sh
cargo run -p rust-tg-bot --example webhook_bot --features webhooks
```

The table below lists the exact command for each one.

## Verifying them

The repository ships a harness that starts every example against the live Bot
API and asserts it reaches a steady state, so a broken example fails rather
than quietly exiting:

```sh
TELEGRAM_BOT_TOKEN=… tools/examples_smoke.py
```

It runs the examples sequentially — two bots polling one token at the same time
make Telegram answer `409 Conflict`. Examples that register a webhook are put
behind a real public tunnel (Cloudflare Tunnel, falling back to
[zrok](https://github.com/openziti/zrok)) and checked against `getWebhookInfo`,
because a bot that merely starts proves nothing about whether Telegram can
reach it. Pass `--no-tunnel` to skip those.

Three tunnel gotchas are worth knowing, because each one produced a
false failure before it was handled:

- `zrok share public` re-execs itself as a background child. Killing the
  foreground command leaves the share running until you exceed your account's
  share limit, so delete it explicitly.
- `cloudflared` prints its URL *before* the hostname resolves in DNS, so a
  probe fired immediately fails with "Name or service not known". Wait until
  the tunnel answers with any status.
- Telegram verifies a registration by calling the URL and gives up on a slow
  first response, so warming the tunnel before registering is what makes the
  test deterministic.

## The examples

| Example | Shows | Features | Extra environment |
|---|---|---|---|
| [`echo_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/echo_bot.rs) | The smallest useful bot: commands, filters, replies | — | — |
| [`raw_api_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/raw_api_bot.rs) | Calling the Bot API directly, without the ext layer | — | — |
| [`commands_derive_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/commands_derive_bot.rs) | The `BotCommands` derive macro | `macros` | — (no token) |
| [`error_handler_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/error_handler_bot.rs) | Catching handler errors and forwarding them to a dev chat | — | `DEVELOPER_CHAT_ID` |
| [`poll_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/poll_bot.rs) | Sending a poll | — | — |
| [`chat_member_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/chat_member_bot.rs) | Querying and reacting to chat member changes | — | — |
| [`deep_linking`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/deep_linking.rs) | `start` payloads and deep links into the bot | — | — |
| [`context_types_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/context_types_bot.rs) | Supplying your own types through `Context` | — | — |
| [`arbitrary_callback_data_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/arbitrary_callback_data_bot.rs) | Storing real objects behind inline buttons instead of short strings | — | — |
| [`rich_message_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/rich_message_bot.rs) | Formatting replies with HTML and Markdown | — | — |
| [`ephemeral_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/ephemeral_bot.rs) | `answer_ephemeral`, which deletes itself after a few seconds | — | — |
| [`inline_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/inline_bot.rs) | Answering inline queries | — | — |
| [`inline_keyboard`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/inline_keyboard.rs) | Building and handling inline keyboards | — | — |
| [`inline_keyboard2`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/inline_keyboard2.rs) | Keyboard layouts and callback routing at larger scale | — | — |
| [`conversation_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/conversation_bot.rs) | Multi-step `ConversationHandler` state machines | — | — |
| [`conversation_bot2`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/conversation_bot2.rs) | A larger conversation with branching states | — | — |
| [`nested_conversation_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/nested_conversation_bot.rs) | Conversations nested inside other conversations | — | — |
| [`persistent_conversation_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/persistent_conversation_bot.rs) | Surviving restarts by persisting conversation state | `persistence-json` | — |
| [`timer_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/timer_bot.rs) | The job queue, for periodic work | `job-queue` | — |
| [`webhook_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/webhook_bot.rs) | The simplest webhook deployment | `webhooks` | `WEBHOOK_URL`, `PORT` |
| [`webhook_echo_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/webhook_echo_bot.rs) | A webhook echo bot with a secret token, end to end | `webhooks` | `WEBHOOK_URL`, `WEBHOOK_SECRET`, `PORT`, `WEBHOOK_PATH` |
| [`custom_webhook_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/custom_webhook_bot.rs) | Your own axum routes alongside the Telegram webhook | `webhooks` | `WEBHOOK_URL`, `ADMIN_CHAT_ID`, `PORT` |
| [`bench_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/bench_bot.rs) | A webhook bot for measuring update latency; `TELEGRAM_API_URL` points it at a stub API for offline runs | `webhooks` | `WEBHOOK_URL`, `PORT`, `TELEGRAM_API_URL` |
| [`payment_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/payment_bot.rs) | Star payments and pre-checkout queries | — | `PAYMENT_PROVIDER_TOKEN` |
| [`passport_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/passport_bot.rs) | Telegram Passport decryption | — | — |
| [`webapp_bot`](https://github.com/HexiCoreDev/rust-telegram-bot/blob/main/crates/telegram-bot/examples/webapp_bot.rs) | Launching a Mini App and answering `web_app_data` | — | — |

## Examples that need more than a token

Three examples cannot do anything meaningful without setup beyond a bot token:

- **`payment_bot`** needs a payment provider token from [@BotFather](https://t.me/BotFather) → `/myprofile` → Payments. Without it `/pay` fails at the `createInvoiceLink` call.
- **`passport_bot`** needs Telegram Passport configured for your bot, plus the RSA private key used to decrypt the returned credentials. The example shows where to load it.
- **The webhook examples** need a public HTTPS URL. See [Webhooks → Getting a Public HTTPS URL](guides/webhooks.md#getting-a-public-https-url) for ngrok, zrok, Cloudflare Tunnel, and localhost.run.

## Where to go next

Each guide references the example it is built on, so you can read the
explanation and the working code side by side. If you are starting from
nothing, [Your First Bot](getting-started/first-bot.md) is the shortest path.
