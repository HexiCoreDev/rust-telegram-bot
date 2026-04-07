# Memory Benchmarks

Side-by-side comparison of three Telegram bot frameworks running identical webhook bots.

## Test Specification

All three bots implement exactly the same features:
- `/start` — greeting with user's first name + inline keyboard (3 buttons)
- `/help` — help text
- Callback query handler — responds to inline keyboard button presses
- Echo handler — echoes any text message back
- Chat action — sends "typing" indicator before echoing
- Webhook mode on port 8000

## Frameworks Tested

| Framework | Version | Language | File |
|-----------|---------|----------|------|
| python-telegram-bot (PTB) | 22.7 | Python 3.14 | `ptb/bench_bot.py` |
| teloxide | 0.17 | Rust 1.93 | `teloxide/` (Cargo project) |
| rust-telegram-bot (RTB) | 1.0.0-beta | Rust 1.93 | `rtb/bench_bot.rs` |

## How to Run

### PTB
```sh
cd ptb
pip install "python-telegram-bot[webhooks]" starlette uvicorn
TELEGRAM_BOT_TOKEN="..." WEBHOOK_URL="https://your.domain" python3 bench_bot.py
```

### teloxide
```sh
cd teloxide
TELEGRAM_BOT_TOKEN="..." WEBHOOK_URL="https://your.domain" cargo run --release
```

### RTB (this project)
```sh
cd rtb
TELEGRAM_BOT_TOKEN="..." WEBHOOK_URL="https://your.domain" ADMIN_CHAT_ID="..." cargo run --release
```

## How to Measure

After starting each bot and sending ~20 messages + button presses:
```sh
ps -eo pid,rss,vsz,comm | grep "<bot_process_name>"
```

RSS = Resident Set Size (actual RAM used).

## Results (April 7, 2026)

**System:** x86_64, 4 cores, 15 GB RAM, Arch Linux
**Rust:** 1.93.0 | **Python:** 3.14.3

| Framework | Idle RSS | Under Load | Binary (stripped) |
|-----------|:--------:|:----------:|:-----------------:|
| PTB 22.7 (Python + Starlette + uvicorn) | 57 MB | 57 MB | N/A (needs runtime) |
| teloxide 0.17 (Rust) | **15 MB** | **17 MB** | **6.6 MB** |
| RTB 1.0.0-beta (Rust + axum) | 20 MB | 32 MB | 12 MB |

**Test protocol:** Each bot received ~21 interactions: `/start`, inline keyboard button presses, `/help`, and text messages echoed with typing indicator. All bots ran in webhook mode on port 8000 behind the same zrok tunnel.

### Analysis

- **PTB** uses the most memory (57 MB) but stays flat — Python's GC pre-allocates and manages memory uniformly.
- **teloxide** is the leanest — 15 MB idle, minimal growth under load, smallest binary. Its webhook implementation is tightly integrated.
- **RTB** sits between the two — 20 MB idle, grows to 32 MB as reqwest connection pools and tokio task allocations warm up. The larger binary (12 MB vs 6.6 MB) reflects the inclusion of the full ext framework (handlers, filters, persistence, job queue) even when not all features are used.

### Why RTB uses more memory than teloxide

1. **Full framework overhead**: RTB includes the `telegram-bot-ext` application framework (handler dispatch, persistence layer, job queue, callback data cache) even in the benchmark bot. teloxide's benchmark uses only the core dispatcher.
2. **axum web server**: RTB uses a full axum router with custom routes. teloxide's webhook integration is lighter weight.
3. **serde_json::Value bridge**: RTB's filter system converts typed Updates to Values for backward compatibility. This creates temporary allocations.
4. **Connection pool sizing**: RTB's reqwest client defaults to a 256-connection pool (matching PTB's httpx settings). teloxide uses a smaller default.

### Value proposition

RTB's memory overhead buys a complete PTB-equivalent framework: 22 handler types, 44+ composable filters, ConversationHandler state machine, persistence backends, job queue, and a developer experience that mirrors python-telegram-bot. For bots that need these features, the ~15 MB premium over teloxide is a reasonable trade-off.
