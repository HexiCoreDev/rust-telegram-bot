# Job Queue

The job queue schedules work to run outside of the normal update-driven flow. Use it for
reminders, periodic reports, maintenance tasks, or anything that needs to fire on a
timer rather than in response to a user message.

The implementation uses tokio timers directly. Each job runs in its own spawned task.
Cancellation is handled via `tokio::sync::watch` channels.

---

## Creating a JobQueue

```rust
use std::sync::Arc;
use telegram_bot::ext::job_queue::JobQueue;

let jq = Arc::new(JobQueue::new());
```

Jobs can be registered before the application starts polling. They will not fire until
the polling loop is active.

---

## Wiring to Application

Pass the queue to the builder so that handlers can access it via `context.job_queue`:

```rust
use telegram_bot::ext::prelude::*;
use telegram_bot::ext::job_queue::JobQueue;
use std::sync::Arc;

let jq = Arc::new(JobQueue::new());

let app: Arc<Application> = ApplicationBuilder::new()
    .token(token)
    .job_queue(Arc::clone(&jq))
    .build();
```

The `Application` calls `jq.stop()` automatically when the application shuts down.

---

## Job Callbacks

All scheduling methods accept a `JobCallbackFn`:

```rust
pub type JobCallbackFn =
    Arc<dyn Fn(JobContext) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send>> + Send + Sync>;
```

The callback receives a `JobContext`:

```rust
pub struct JobContext {
    pub job_name: String,
    pub chat_id: Option<i64>,
    pub user_id: Option<i64>,
    pub data: Option<serde_json::Value>,
}
```

`chat_id` and `user_id` are the values you passed when scheduling the job. `data` is any
`serde_json::Value` you attached. These let you pass context from the handler that
creates the job to the callback that runs it later.

---

## once (one-shot)

Schedule a job to run once after a delay. Uses a builder pattern for optional fields.

```rust
use std::sync::Arc;
use std::time::Duration;
use serde_json::json;
use telegram_bot::ext::job_queue::{JobCallbackFn, JobContext};

let bot = Arc::clone(context.bot());
let callback: JobCallbackFn = Arc::new(move |ctx: JobContext| {
    let bot = Arc::clone(&bot);
    Box::pin(async move {
        let chat_id = ctx.chat_id.unwrap_or(0);
        if chat_id != 0 {
            bot.send_message(chat_id, "Reminder!").send().await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        }
        Ok(())
    })
});

let job = jq
    .once(callback, Duration::from_secs(30))
    .name("reminder")
    .chat_id(chat_id)
    .user_id(user_id)
    .data(json!({ "text": "Time's up!" }))
    .start()
    .await;
```

Returns a `Job` handle for cancellation.

---

## repeating

Schedule a job to run at a fixed interval.

```rust
let job = jq
    .repeating(callback, Duration::from_secs(60))
    .name("heartbeat")
    .first(Duration::from_secs(5))    // initial delay (omit to fire immediately)
    .last(Duration::from_secs(3600))  // stop after this duration (omit for forever)
    .start()
    .await;
```

The `first` delay controls when the first execution happens relative to registration.
The `last` deadline is measured from registration: after `last` duration has elapsed,
the job stops itself.

Missed ticks are skipped (not accumulated). If the system is overloaded and a tick is
late, the scheduler catches up to the next tick rather than firing multiple times.

---

## daily

Schedule a job to run daily at a specific time on selected days of the week.

```rust
use chrono::NaiveTime;

let time = NaiveTime::from_hms_opt(9, 0, 0).unwrap(); // 09:00 UTC

let job = jq
    .daily(callback, time, &[1, 2, 3, 4, 5])
    .name("morning_report")
    .chat_id(chat_id)
    .start()
    .await;
```

Day numbering follows the python-telegram-bot convention: 0 = Sunday, 1 = Monday, ...,
6 = Saturday. All times are in UTC.

To run every day, pass all seven days:

```rust
let job = jq
    .daily(callback, time, &[0, 1, 2, 3, 4, 5, 6])
    .name("daily_task")
    .start()
    .await;
```

---

## monthly

Schedule a job to run on a specific day of each month.

```rust
use chrono::NaiveTime;

let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap(); // midnight UTC

// Run on the 1st of every month
let job = jq
    .monthly(callback, time, 1)
    .name("monthly_summary")
    .start()
    .await;

// Run on the last day of every month (pass -1)
let job = jq
    .monthly(callback, time, -1)
    .name("end_of_month")
    .start()
    .await;
```

If the specified day does not exist in a given month (e.g. day 31 in February), the
job is skipped for that month.

---

## run_custom

A one-shot job that works as a method on `Arc<JobQueue>`. The job is registered and
the timer started in a background task; the `Job` handle is returned immediately.

```rust
let job = jq.run_custom(
    callback,
    Duration::from_secs(10),
    Some("custom_job".to_string()),
    Some(json!({ "key": "value" })),
    Some(chat_id),
    Some(user_id),
);
// Note: run_custom is not async; it returns immediately
```

---

## Job Cancellation and Control

Every scheduling method returns a `Job` handle:

```rust
pub struct Job {
    pub id: u64,
    pub name: String,
    pub chat_id: Option<i64>,
    pub user_id: Option<i64>,
    pub data: Option<serde_json::Value>,
    // private fields
}

impl Job {
    pub fn schedule_removal(&self);        // cancel: will not fire again
    pub fn is_removed(&self) -> bool;
    pub fn is_enabled(&self) -> bool;
    pub fn set_enabled(&self, enabled: bool);  // pause / resume
    pub async fn run(&self);               // fire the callback immediately (bypass schedule)
}
```

Cancel a job:

```rust
let job = jq
    .repeating(callback, Duration::from_secs(30))
    .name("ping")
    .start()
    .await;

// Later, cancel it
job.schedule_removal();
```

Pause and resume without cancelling:

```rust
job.set_enabled(false); // paused: timer still runs, callback is skipped
job.set_enabled(true);  // resumed
```

Run immediately (useful for testing or forcing an immediate execution):

```rust
job.run().await; // only runs if enabled
```

---

## Querying Scheduled Jobs

```rust
// All non-removed jobs
let jobs = jq.jobs().await;

// Jobs with an exact name
let reminders = jq.get_jobs_by_name("reminder").await;

// Jobs matching a regex pattern
let timed = jq.jobs_by_pattern(r"^timer_").await;
```

Names are not unique. Multiple jobs can share the same name -- this is useful for
per-user reminder jobs where you want to cancel all reminders for a given user:

```rust
// Schedule a per-user reminder
jq.once(callback, Duration::from_secs(300))
    .name(format!("reminder_{}", user_id))
    .chat_id(chat_id)
    .user_id(user_id)
    .start()
    .await;

// Cancel all of this user's reminders
for job in jq.get_jobs_by_name(&format!("reminder_{}", user_id)).await {
    job.schedule_removal();
}
```

---

## Stopping the Queue

```rust
jq.stop().await;
```

Marks all registered jobs as removed and sends a cancellation signal to all running
timer loops. When wired to an `Application`, this is called automatically on shutdown.

---

## Accessing the Job Queue from Handlers

If you passed the queue to the `ApplicationBuilder`, retrieve it from the context:

```rust
use telegram_bot::ext::prelude::*;
use telegram_bot::ext::job_queue::{JobCallbackFn, JobContext};

async fn set_reminder(update: Update, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().expect("update must have a chat").id;

    let jq = context
        .job_queue
        .as_ref()
        .expect("job_queue should be set on context");

    let bot = Arc::clone(context.bot());
    let callback: JobCallbackFn = Arc::new(move |ctx: JobContext| {
        let bot = Arc::clone(&bot);
        Box::pin(async move {
            let target_chat_id = ctx.chat_id.unwrap_or(0);
            if target_chat_id != 0 {
                bot.send_message(target_chat_id, "Reminder!").send().await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            }
            Ok(())
        })
    });

    jq.once(callback, Duration::from_secs(60))
        .name("user_reminder")
        .chat_id(chat_id)
        .start()
        .await;

    context.reply_text(&update, "Reminder set for 60 seconds!").await?;
    Ok(())
}
```
