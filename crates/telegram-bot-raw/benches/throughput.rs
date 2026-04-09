//! Throughput and latency benchmarks for the raw Telegram Bot API crate.
//!
//! Run with:
//!   cargo bench -p rust-tg-bot-raw --bench throughput

use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use serde_json::json;

use rust_tg_bot_raw::bot::Bot;
use rust_tg_bot_raw::request::base::{async_trait, BaseRequest, HttpMethod, TimeoutOverride};
use rust_tg_bot_raw::request::request_data::RequestData;
use rust_tg_bot_raw::types::update::Update;

// ---------------------------------------------------------------------------
// Mock request backend (never hits the network)
// ---------------------------------------------------------------------------

struct NoopRequest;

#[async_trait]
impl BaseRequest for NoopRequest {
    async fn initialize(&self) -> rust_tg_bot_raw::error::Result<()> {
        Ok(())
    }
    async fn shutdown(&self) -> rust_tg_bot_raw::error::Result<()> {
        Ok(())
    }
    fn default_read_timeout(&self) -> Option<std::time::Duration> {
        None
    }
    async fn do_request(
        &self,
        _url: &str,
        _method: HttpMethod,
        _data: Option<&RequestData>,
        _timeouts: TimeoutOverride,
    ) -> rust_tg_bot_raw::error::Result<(u16, bytes::Bytes)> {
        unreachable!()
    }
    async fn do_request_json_bytes(
        &self,
        _url: &str,
        _body: &[u8],
        _timeouts: TimeoutOverride,
    ) -> rust_tg_bot_raw::error::Result<(u16, bytes::Bytes)> {
        unreachable!()
    }
}

fn make_bot() -> Bot {
    Bot::new(
        "000000000:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        Arc::new(NoopRequest),
    )
}

// ---------------------------------------------------------------------------
// Sample JSON payloads
// ---------------------------------------------------------------------------

/// A realistic text-message Update JSON payload.
fn sample_text_update_json() -> serde_json::Value {
    json!({
        "update_id": 123456789,
        "message": {
            "message_id": 42,
            "from": {
                "id": 987654321,
                "is_bot": false,
                "first_name": "John",
                "last_name": "Doe",
                "username": "johndoe",
                "language_code": "en"
            },
            "chat": {
                "id": -1001234567890_i64,
                "title": "Test Group",
                "type": "supergroup"
            },
            "date": 1700000000,
            "text": "Hello, this is a test message for benchmarking!",
            "entities": [
                {
                    "type": "bot_command",
                    "offset": 0,
                    "length": 6
                }
            ]
        }
    })
}

/// A command update (message starting with /start).
fn sample_command_update_json() -> serde_json::Value {
    json!({
        "update_id": 123456790,
        "message": {
            "message_id": 43,
            "from": {
                "id": 987654321,
                "is_bot": false,
                "first_name": "John",
                "username": "johndoe"
            },
            "chat": {
                "id": 987654321,
                "type": "private",
                "first_name": "John"
            },
            "date": 1700000001,
            "text": "/start some arguments here",
            "entities": [
                {
                    "type": "bot_command",
                    "offset": 0,
                    "length": 6
                }
            ]
        }
    })
}

/// A callback query update.
fn sample_callback_query_json() -> serde_json::Value {
    json!({
        "update_id": 123456791,
        "callback_query": {
            "id": "4382bfdwdsb323b2d9",
            "from": {
                "id": 987654321,
                "is_bot": false,
                "first_name": "John",
                "username": "johndoe"
            },
            "chat_instance": "aGlkZGVuIGNoYXQgaW5zdGFuY2U",
            "data": "button_callback_data"
        }
    })
}

/// A plain text update (no entities).
fn sample_plain_text_update_json() -> serde_json::Value {
    json!({
        "update_id": 123456792,
        "message": {
            "message_id": 44,
            "from": {
                "id": 111222333,
                "is_bot": false,
                "first_name": "Alice"
            },
            "chat": {
                "id": 111222333,
                "type": "private",
                "first_name": "Alice"
            },
            "date": 1700000002,
            "text": "Just a regular message without commands or special entities"
        }
    })
}

/// Pre-serialize a JSON value to bytes (simulates what arrives over the wire).
fn json_bytes(val: &serde_json::Value) -> Vec<u8> {
    serde_json::to_vec(val).unwrap()
}

// ---------------------------------------------------------------------------
// Benchmark: Update deserialization throughput
// ---------------------------------------------------------------------------

fn bench_update_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("update_deserialization");

    // Prepare a batch of 1000 different serialized updates.
    let payloads: Vec<Vec<u8>> = (0..1000)
        .map(|i| {
            let mut v = sample_text_update_json();
            v["update_id"] = json!(i);
            v["message"]["message_id"] = json!(i);
            json_bytes(&v)
        })
        .collect();

    let total_bytes: usize = payloads.iter().map(|p| p.len()).sum();
    group.throughput(Throughput::Bytes(total_bytes as u64));

    group.bench_function("1000_text_updates", |b| {
        b.iter(|| {
            for payload in &payloads {
                let _update: Update = black_box(serde_json::from_slice(payload).unwrap());
            }
        });
    });

    // Mixed update types
    let mixed_payloads: Vec<Vec<u8>> = (0..1000)
        .map(|i| {
            let v = match i % 4 {
                0 => {
                    let mut v = sample_text_update_json();
                    v["update_id"] = json!(i);
                    v
                }
                1 => {
                    let mut v = sample_command_update_json();
                    v["update_id"] = json!(i);
                    v
                }
                2 => {
                    let mut v = sample_callback_query_json();
                    v["update_id"] = json!(i);
                    v
                }
                _ => {
                    let mut v = sample_plain_text_update_json();
                    v["update_id"] = json!(i);
                    v
                }
            };
            json_bytes(&v)
        })
        .collect();

    let mixed_bytes: usize = mixed_payloads.iter().map(|p| p.len()).sum();
    group.throughput(Throughput::Bytes(mixed_bytes as u64));

    group.bench_function("1000_mixed_updates", |b| {
        b.iter(|| {
            for payload in &mixed_payloads {
                let _update: Update = black_box(serde_json::from_slice(payload).unwrap());
            }
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark: Update serialization (re-serialize to bytes)
// ---------------------------------------------------------------------------

fn bench_update_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("update_serialization");

    // Pre-deserialize 1000 updates.
    let updates: Vec<Update> = (0..1000)
        .map(|i| {
            let mut v = sample_text_update_json();
            v["update_id"] = json!(i);
            v["message"]["message_id"] = json!(i);
            serde_json::from_value(v).unwrap()
        })
        .collect();

    group.throughput(Throughput::Elements(1000));

    group.bench_function("1000_updates_to_bytes", |b| {
        b.iter(|| {
            for update in &updates {
                let _bytes = black_box(serde_json::to_vec(update).unwrap());
            }
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark: SendMessage builder construction
// ---------------------------------------------------------------------------

fn bench_builder_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("builder_construction");
    group.throughput(Throughput::Elements(1000));

    let bot = make_bot();

    group.bench_function("1000_send_message_builders", |b| {
        b.iter(|| {
            for i in 0..1000_i64 {
                let builder = bot.send_message(black_box(i), black_box("Hello, world!"));
                black_box(builder);
            }
        });
    });

    group.bench_function("1000_send_message_builders_with_options", |b| {
        b.iter(|| {
            for i in 0..1000_i64 {
                let builder = bot
                    .send_message(black_box(i), black_box("Hello, world!"))
                    .parse_mode("HTML")
                    .disable_notification(true)
                    .protect_content(false);
                black_box(builder);
            }
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark: SendMessage serialization (builder -> JSON bytes)
// ---------------------------------------------------------------------------

fn bench_builder_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("builder_serialization");
    group.throughput(Throughput::Elements(1000));

    let bot = make_bot();

    group.bench_function("1000_send_message_to_json", |b| {
        b.iter(|| {
            for i in 0..1000_i64 {
                let builder = bot
                    .send_message(black_box(i), black_box("Hello from benchmark!"))
                    .parse_mode("HTML")
                    .disable_notification(true);
                // Serialize the builder to JSON bytes (same path as .send())
                let bytes = serde_json::to_vec(&builder).unwrap();
                black_box(bytes);
            }
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark: Single Update deserialization latency
// ---------------------------------------------------------------------------

fn bench_single_update_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_update_latency");

    let text_bytes = json_bytes(&sample_text_update_json());
    let command_bytes = json_bytes(&sample_command_update_json());
    let callback_bytes = json_bytes(&sample_callback_query_json());

    group.bench_function("text_message", |b| {
        b.iter(|| {
            let _: Update = black_box(serde_json::from_slice(&text_bytes).unwrap());
        });
    });

    group.bench_function("command_message", |b| {
        b.iter(|| {
            let _: Update = black_box(serde_json::from_slice(&command_bytes).unwrap());
        });
    });

    group.bench_function("callback_query", |b| {
        b.iter(|| {
            let _: Update = black_box(serde_json::from_slice(&callback_bytes).unwrap());
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark: Type deserialization microbenchmarks
// ---------------------------------------------------------------------------

fn bench_type_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("type_deserialization");

    let user_json = json_bytes(
        &json!({"id": 1, "is_bot": false, "first_name": "Test", "username": "test_user"}),
    );
    let chat_json = json_bytes(&json!({"id": -100, "type": "supergroup", "title": "Test"}));
    let message_json = json_bytes(&json!({
        "message_id": 1, "date": 0,
        "chat": {"id": 1, "type": "private"},
        "from": {"id": 1, "is_bot": false, "first_name": "T"},
        "text": "hello world"
    }));

    group.bench_function("user", |b| {
        b.iter(|| {
            let _: rust_tg_bot_raw::types::user::User =
                black_box(serde_json::from_slice(&user_json).unwrap());
        });
    });

    group.bench_function("chat", |b| {
        b.iter(|| {
            let _: rust_tg_bot_raw::types::chat::Chat =
                black_box(serde_json::from_slice(&chat_json).unwrap());
        });
    });

    group.bench_function("message", |b| {
        b.iter_batched(
            || message_json.clone(),
            |data| {
                let _: rust_tg_bot_raw::types::message::Message =
                    black_box(serde_json::from_slice(&data).unwrap());
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark: JSON Value parsing vs typed deserialization
// ---------------------------------------------------------------------------

fn bench_value_vs_typed(c: &mut Criterion) {
    let mut group = c.benchmark_group("value_vs_typed");

    let update_bytes = json_bytes(&sample_text_update_json());

    group.bench_function("to_serde_value", |b| {
        b.iter(|| {
            let _: serde_json::Value = black_box(serde_json::from_slice(&update_bytes).unwrap());
        });
    });

    group.bench_function("to_typed_update", |b| {
        b.iter(|| {
            let _: Update = black_box(serde_json::from_slice(&update_bytes).unwrap());
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Register all benchmark groups
// ---------------------------------------------------------------------------

criterion_group!(
    benches,
    bench_update_deserialization,
    bench_update_serialization,
    bench_builder_construction,
    bench_builder_serialization,
    bench_single_update_latency,
    bench_type_deserialization,
    bench_value_vs_typed,
);

criterion_main!(benches);
