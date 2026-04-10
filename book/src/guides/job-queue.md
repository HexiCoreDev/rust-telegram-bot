# Job Queue

The job queue lets you schedule delayed and recurring tasks. Common use cases include reminders, periodic notifications, and cleanup routines.

## Setup

Enable the `job-queue` feature:

```toml
[dependencies]
rust-tg-bot = { version = "1.0.0-beta.4c", features = ["job-queue"] }
```

Create a `JobQueue` and pass it to the application builder:

```rust
use std::sync::Arc;
use rust_tg_bot::ext::job_queue::JobQueue;
use rust_tg_bot::ext::prelude::ApplicationBuilder;

let jq = Arc::new(JobQueue::new());

let app = ApplicationBuilder::new()
    .token(token)
    .job_queue(Arc::clone(&jq))
    .build();
```

## Scheduling a One-Shot Job

Schedule a job that fires once after a delay:

```rust
use std::time::Duration;
use rust_tg_bot::ext::job_queue::{JobCallbackFn, JobContext};

async fn set_timer(
    update: Arc<Update>,
    context: Context,
    timer_store: TimerStore,
) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;
    let seconds: u64 = 30; // from user input

    // Build the callback
    let bot = Arc::clone(context.bot());
    let alarm_callback: JobCallbackFn = Arc::new(move |ctx: JobContext| {
        let bot = Arc::clone(&bot);
        Box::pin(async move {
            let target_chat_id = ctx.chat_id.unwrap_or(0);
            if target_chat_id == 0 {
                return Ok(());
            }
            bot.send_message(target_chat_id, "BEEP! Timer is done!")
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            Ok(())
        })
    });

    // Schedule via the job queue
    let jq = context.job_queue.as_ref().expect("job_queue should be set");

    let job = jq
        .once(alarm_callback, Duration::from_secs(seconds))
        .name(format!("timer_{chat_id}"))
        .chat_id(chat_id)
        .start()
        .await;

    // Store the job ID for later cancellation
    timer_store.write().await.insert(chat_id, job.id);

    context.bot()
        .send_message(chat_id, &format!("Timer set for {seconds} seconds!"))
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}
```

## Job Builder Methods

The job builder returned by `jq.once()` supports these methods:

| Method | Description |
|---|---|
| `.name(name)` | Human-readable name for the job |
| `.chat_id(id)` | Associates a chat ID with the job (passed to the callback via `JobContext`) |
| `.start()` | Schedules the job and returns a `Job` handle |

## Cancelling Jobs

Cancel a job by calling `schedule_removal()` on its handle:

```rust
async fn unset_timer(
    update: Arc<Update>,
    context: Context,
    timer_store: TimerStore,
) -> HandlerResult {
    let chat_id = update.effective_chat().unwrap().id;

    let jq = context.job_queue.as_ref().unwrap();

    let removed = {
        let mut store = timer_store.write().await;
        if let Some(old_job_id) = store.remove(&chat_id) {
            let jobs = jq.jobs().await;
            for job in jobs {
                if job.id == old_job_id {
                    job.schedule_removal();
                    break;
                }
            }
            true
        } else {
            false
        }
    };

    let reply = if removed {
        "Timer successfully cancelled!"
    } else {
        "You have no active timer."
    };

    context.bot()
        .send_message(chat_id, reply)
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}
```

## JobContext

The callback receives a `JobContext` with metadata about the job:

```rust
let callback: JobCallbackFn = Arc::new(|ctx: JobContext| {
    Box::pin(async move {
        let chat_id = ctx.chat_id.unwrap_or(0);
        // Use chat_id to send messages
        Ok(())
    })
});
```

## Tracking Active Jobs

Use a shared store to map chat IDs to job IDs:

```rust
use std::collections::HashMap;
use rust_tg_bot::ext::prelude::{Arc, RwLock};

type TimerStore = Arc<RwLock<HashMap<i64, u64>>>;
```

Before scheduling a new job, cancel any existing one for the same chat:

```rust
{
    let store = timer_store.read().await;
    if let Some(&old_job_id) = store.get(&chat_id) {
        let jobs = jq.jobs().await;
        for job in jobs {
            if job.id == old_job_id {
                job.schedule_removal();
                break;
            }
        }
    }
}
```

## Complete Timer Bot

See the `timer_bot` example in the repository for a full working implementation:

```sh
TELEGRAM_BOT_TOKEN="..." cargo run -p rust-tg-bot --example timer_bot --features job-queue
```

Commands:
- `/start` -- show usage
- `/set 30` -- set a 30-second timer
- `/unset` -- cancel the active timer

## Next Steps

- [Persistence](persistence.md) -- persist job data across restarts.
- [Error Handling](../advanced/error-handling.md) -- handle job callback failures.
