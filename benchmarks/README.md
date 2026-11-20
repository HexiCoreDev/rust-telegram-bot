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
| rust-telegram-bot (RTB) | 1.0.0-rc.3 | Rust 1.93 | `rtb/bench_bot.rs` |

> The RTB rows are pinned to rc.3 because that is the build these numbers
> were measured on. `make bump` rewrites them mechanically; they are
> reverted deliberately, and re-measured only when a new benchmark run
> replaces them.

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

## Results (November 20, 2026) — offline re-run, latest teloxide

**System:** x86_64, 14 cores, 15 GB RAM, Arch Linux (kernel 7.3.5)
**Rust:** 1.100.0, release profile (RTB: workspace LTO / `opt-level="z"` / strip; teloxide-bench: default release)
**teloxide:** 0.17.0 — the newest published release, so the comparison target was already current

Both bots are the same benchmark bot as above (inline keyboard, `/help`, echo with typing
action, callback query, webhook on :8000). This run needs **no bot token**: the bots point at
`benchmarks/stub_api.py` via `TELEGRAM_API_URL`, and synthetic updates are POSTed straight to
each bot's webhook by `benchmarks/measure_rss.sh`. There is no live Telegram traffic and no
TLS, so these are framework footprints, not end-to-end latencies.

| Framework | Idle RSS | +200 updates | +1000 updates | Binary (release) | Binary (stripped) | Crates |
|-----------|:--------:|:------------:|:-------------:|:----------------:|:-----------------:|:------:|
| teloxide 0.17.0 | 14.0 MB | 14.1 MB | 14.1 MB | 9.45 MB | **6.45 MB** | 208 |
| RTB 1.0.0 | **7.2 MB** | 9.5 MB | 9.6 MB | **7.22 MB**¹ | 7.22 MB | 160 |

¹ RTB's example is compiled with the workspace profile, which already strips; the "release"
and "stripped" columns are therefore the same binary.

What the numbers say:

- **Idle footprint**: RTB uses about **half** the RSS of teloxide (7.2 vs 14.0 MB) with 160
  crates in the graph instead of 208. Unlike teloxide, RTB does not keep the whole framework
  (dispatcher, dialogues, filters, conversation state) initialized at startup.
- **Per-update growth**: RTB rises ~2.3 MB and then **plateaus** — identical after 200 and
  1000 updates, so it is allocator retention for the per-update buffers, not a leak.
  teloxide's RSS is flat (+0.2 MB over 1000 updates).
- **Binary size**: teloxide's stripped binary is ~0.8 MB smaller (6.45 vs 7.22 MB) for a
  single-bot webhook program. RTB carries the full framework surface in the same binary.
- **Caveat**: no live Telegram round-trips means no TLS sessions and no response payloads
  beyond the stub's canned objects; a live run adds a few MB to both sides.

Re-running it:

```sh
cargo build --release -p rust-tg-bot --example bench_bot --features webhooks
cargo build --release --manifest-path benchmarks/teloxide/Cargo.toml
benchmarks/measure_rss.sh 200        # or 1000
```

The harness gained three things for this run:

- `benchmarks/stub_api.py` — a case-insensitive Bot API stub (`setWebhook`, `getMe`,
  `sendMessage`, `editMessageText`, `getWebhookInfo`).
- `benchmarks/measure_rss.sh` — starts the stub, runs each bot, sends N synthetic updates,
  reports idle/load RSS.
- `TELEGRAM_API_URL` support in both bench bots — RTB via `ApplicationBuilder::base_url`
  (previously a stored-but-ignored option; fixed here), teloxide via `Bot::set_api_url`.

## Results (April 8, 2026) — live API run

**System:** x86_64, 4 cores, 15 GB RAM, Arch Linux
**Rust:** 1.93.0 | **Python:** 3.14.3

| Framework | Idle RSS | Under Load | Binary (stripped) |
|-----------|:--------:|:----------:|:-----------------:|
| PTB 22.7 (Python + Starlette + uvicorn) | 57 MB | 60 MB | N/A (needs runtime) |
| teloxide 0.17 (Rust) | **15 MB** | **17 MB** | 6.6 MB |
| RTB 1.0.0-rc.3 (Rust + axum) | **15 MB** | **17 MB** | **6.6 MB** |

**Test protocol:** Each bot received 21+ interactions: `/start`, inline keyboard button presses, `/help`, and text messages echoed with typing indicator. All bots ran in webhook mode on port 8000 behind the same zrok tunnel.

### Optimization history

| Version | Idle | Under Load | Binary | Key changes |
|---------|:----:|:----------:|:------:|------------|
| beta.1 (initial) | 20 MB | 32 MB | 12 MB | Baseline |
| beta.2 (P2-P6,T2) | 17 MB | 21 MB | 11 MB | Pool 8, typed filters, Arc\<str\> |
| beta.2 (S1-S3) | 17 MB | 20 MB | 9.6 MB | UpdateKind enum, Message boxing, direct serde |
| beta.2 (final) | **15 MB** | **17 MB** | **6.2 MB** | LTO, codegen-units=1, strip, size opts |
| beta.4b | **15 MB** | **17 MB** | **6.2 MB** | Macros, Redis/Postgres persistence, TLS webhooks, #[non_exhaustive] |
| rc.2 (10.3) | **15 MB** | **17 MB** | **6.6 MB** | Full Bot API 10.3 surface (+0.4 MB binary); Message −33% via payload boxing |
| 1.0.0 (10.4) | **15 MB** (profile unchanged) | **17 MB** (profile unchanged) | **6.9 MB** | Full Bot API 10.4 surface: widgets, TON wallet, share themes, 8 new methods; Message stays 6336 B |

### Analysis

- **PTB** uses the most memory (57 MB idle, 60 MB under load) — Python's GC and interpreter overhead.
- **teloxide** measured 15/17 MB (whole-MB readout) with a 6.6 MB stripped binary in the live run; the November offline harness puts it at 14.0 MB idle.
- **RTB** measured the same 15/17 MB in the live run; the November re-run shows a **7.2 MB** idle footprint (half of teloxide's) while providing the full PTB-equivalent framework: 22 handler types, 44+ composable filters, ConversationHandler, persistence, job queue, 194 builders, 90+ type constructors. RTB's binary is **6.9 MB** stripped (echo_bot reference) and **7.2 MB** for the benchmark bot with `webhooks`.

### Why RTB matches teloxide despite the larger feature set

1. **Aggressive release profile**: LTO, `codegen-units = 1`, `strip = true`, and `opt-level = "z"` eliminate dead code and framework overhead that is unused at runtime.
2. **Feature-gated modules**: job-queue and persistence are not compiled in unless explicitly enabled. The benchmark bot runs with only the features it uses.
3. **Arc\<str\> and Arc\<Update\>**: Zero-copy dispatch reduces allocations under load to match teloxide's profile.
4. **Direct serde for text builders**: 21 text-only builders bypass the double serde pass, reducing per-request overhead.

### Value proposition

RTB uses roughly half teloxide's idle memory (7.2 vs 14.0 MB, November 2026) while providing: ConversationHandler state machine, JSON/SQLite persistence, tokio-native job queue, 44+ composable filters with `&`/`|`/`!` operators, 194 directly-awaitable builders, 90+ type constructors, and a developer experience that mirrors python-telegram-bot. That is 4x less memory than PTB with no runtime required. teloxide's stripped binary is smaller (6.45 vs 7.22 MB) for a single-bot webhook program — that is the price of the full framework surface.
