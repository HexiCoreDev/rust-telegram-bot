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
| rust-telegram-bot (RTB) | 1.0.0-beta.2 | Rust 1.93 | `rtb/bench_bot.rs` |

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

## Results (April 8, 2026)

**System:** x86_64, 4 cores, 15 GB RAM, Arch Linux
**Rust:** 1.93.0 | **Python:** 3.14.3

| Framework | Idle RSS | Under Load | Binary (stripped) |
|-----------|:--------:|:----------:|:-----------------:|
| PTB 22.7 (Python + Starlette + uvicorn) | 57 MB | 60 MB | N/A (needs runtime) |
| teloxide 0.17 (Rust) | **15 MB** | **17 MB** | 6.6 MB |
| RTB 1.0.0-beta.2 (Rust + axum) | **15 MB** | **17 MB** | **6.2 MB** |

**Test protocol:** Each bot received 21+ interactions: `/start`, inline keyboard button presses, `/help`, and text messages echoed with typing indicator. All bots ran in webhook mode on port 8000 behind the same zrok tunnel.

### Optimization history

| Version | Idle | Under Load | Binary | Key changes |
|---------|:----:|:----------:|:------:|------------|
| beta.1 (initial) | 20 MB | 32 MB | 12 MB | Baseline |
| beta.2 (P2-P6,T2) | 17 MB | 21 MB | 11 MB | Pool 8, typed filters, Arc\<str\> |
| beta.2 (S1-S3) | 17 MB | 20 MB | 9.6 MB | UpdateKind enum, Message boxing, direct serde |
| beta.2 (final) | **15 MB** | **17 MB** | **6.2 MB** | LTO, codegen-units=1, strip, size opts |

### Analysis

- **PTB** uses the most memory (57 MB idle, 60 MB under load) — Python's GC and interpreter overhead.
- **teloxide** is lean at 15/17 MB with a 6.6 MB binary. Its focused dispatcher design avoids framework overhead.
- **RTB** matches teloxide exactly (15/17 MB) while providing the full PTB-equivalent framework: 22 handler types, 44+ composable filters, ConversationHandler, persistence, job queue, 168 builders, 90+ type constructors. RTB's binary is **smaller** than teloxide at 6.2 MB.

### Why RTB matches teloxide despite the larger feature set

1. **Aggressive release profile**: LTO, `codegen-units = 1`, `strip = true`, and `opt-level = "z"` eliminate dead code and framework overhead that is unused at runtime.
2. **Feature-gated modules**: job-queue and persistence are not compiled in unless explicitly enabled. The benchmark bot runs with only the features it uses.
3. **Arc\<str\> and Arc\<Update\>**: Zero-copy dispatch reduces allocations under load to match teloxide's profile.
4. **Direct serde for text builders**: 21 text-only builders bypass the double serde pass, reducing per-request overhead.

### Value proposition

RTB matches teloxide on memory (15/17 MB) and beats it on binary size (6.2 vs 6.6 MB) while providing: ConversationHandler state machine, JSON/SQLite persistence, tokio-native job queue, 44+ composable filters with `&`/`|`/`!` operators, 168 directly-awaitable builders, 90+ type constructors, and a developer experience that mirrors python-telegram-bot. That is 3.5x less memory than PTB with no runtime required.
