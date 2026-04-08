//! Timer Bot -- demonstrates the job queue for scheduling delayed messages.
//!
//! This is the Rust port of Python's `timerbot.py`.
//!
//! Demonstrates:
//! - Command handling with argument parsing
//! - `JobQueue` integration (`run_once`)
//! - Job cancellation
//! - Accessing `job_queue` from context
//! - **Typed `Update` access**
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p telegram-bot --example timer_bot
//! ```
//!
//! Then in Telegram:
//! - `/start` -- shows usage instructions
//! - `/set 30` -- sets a timer for 30 seconds
//! - `/unset` -- cancels any active timer for your chat

use std::time::Duration;
use telegram_bot::ext::job_queue::{JobCallbackFn, JobContext, JobQueue};
use telegram_bot::ext::prelude::{
    Application, ApplicationBuilder, Context, FnHandler, HandlerError, HandlerResult,
    MessageEntityType, Update, Arc, RwLock,
};

/// A shared map to track active timer job IDs per chat.
type TimerStore = Arc<RwLock<std::collections::HashMap<i64, u64>>>;

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// `/start` -- show usage information.
async fn start_command(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().expect("update must have a chat").id;

    context
        .bot()
        .send_message(
            chat_id,
            "Hi! Use /set <seconds> to set a timer.\n\
             Use /unset to cancel the current timer.",
        )
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// `/set <seconds>` -- set a timer that fires after the given number of seconds.
async fn set_timer(
    update: Arc<Update>,
    context: Context,
    timer_store: TimerStore,
) -> HandlerResult {
    let msg = update
        .effective_message()
        .expect("update must have a message");
    let chat_id = msg.chat.id;
    let text = msg.text.as_deref().unwrap_or("");

    let args: Vec<&str> = text.split_whitespace().skip(1).collect();

    let seconds: u64 = match args.first() {
        Some(s) => match s.parse() {
            Ok(n) if n > 0 => n,
            _ => {
                context
                    .bot()
                    .send_message(
                        chat_id,
                        "Usage: /set <seconds>\nPlease provide a positive number.",
                    )
                    .send()
                    .await
                    .map_err(|e| HandlerError::Other(Box::new(e)))?;
                return Ok(());
            }
        },
        None => {
            context
                .bot()
                .send_message(chat_id, "Usage: /set <seconds>")
                .send()
                .await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
            return Ok(());
        }
    };

    // Build the callback that will fire when the timer expires.
    let bot = Arc::clone(context.bot());
    let alarm_callback: JobCallbackFn = Arc::new(move |ctx: JobContext| {
        let bot = Arc::clone(&bot);
        Box::pin(async move {
            let target_chat_id = ctx.chat_id.unwrap_or(0);
            if target_chat_id == 0 {
                return Ok(());
            }
            bot.send_message(target_chat_id, "BEEP! Timer is done!")
                .send()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            Ok(())
        })
    });

    // Schedule the job via the job queue.
    let jq = context
        .job_queue
        .as_ref()
        .expect("job_queue should be set on context");

    // Cancel any existing timer for this chat.
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

    let job = jq
        .once(alarm_callback, Duration::from_secs(seconds))
        .name(format!("timer_{chat_id}"))
        .chat_id(chat_id)
        .start()
        .await;

    // Store the job ID so we can cancel it later.
    timer_store.write().await.insert(chat_id, job.id);

    context
        .bot()
        .send_message(chat_id, &format!("Timer set for {seconds} seconds!"))
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// `/unset` -- cancel the active timer for this chat.
async fn unset_timer(
    update: Arc<Update>,
    context: Context,
    timer_store: TimerStore,
) -> HandlerResult {
    let chat_id = update.effective_chat().expect("update must have a chat").id;

    let jq = context
        .job_queue
        .as_ref()
        .expect("job_queue should be set on context");

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

    context
        .bot()
        .send_message(chat_id, reply)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn check_command(update: &Update, expected: &str) -> bool {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return false,
    };
    let text = match msg.text.as_deref() {
        Some(t) => t,
        None => return false,
    };
    let entities = match msg.entities.as_ref() {
        Some(e) => e,
        None => return false,
    };
    entities.first().map_or(false, |e| {
        if e.entity_type == MessageEntityType::BotCommand && e.offset == 0 {
            let length = e.length as usize;
            if length <= text.len() {
                let cmd_name = text[1..length].split('@').next().unwrap_or("");
                cmd_name.eq_ignore_ascii_case(expected)
            } else {
                false
            }
        } else {
            false
        }
    })
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

    // Create a job queue and share it with the application.
    let jq = Arc::new(JobQueue::new());

    let app: Arc<Application> = ApplicationBuilder::new()
        .token(token)
        .job_queue(Arc::clone(&jq))
        .build();

    // Shared timer store across handlers.
    let timer_store: TimerStore = Arc::new(RwLock::new(std::collections::HashMap::new()));

    // /start handler
    app.add_typed_handler(
        FnHandler::new(|u| check_command(u, "start"), start_command),
        0,
    )
    .await;

    // /set handler
    {
        let store = Arc::clone(&timer_store);
        app.add_typed_handler(
            FnHandler::new(
                |u| check_command(u, "set"),
                move |update, ctx| {
                    let s = Arc::clone(&store);
                    async move { set_timer(update, ctx, s).await }
                },
            ),
            0,
        )
        .await;
    }

    // /unset handler
    {
        let store = Arc::clone(&timer_store);
        app.add_typed_handler(
            FnHandler::new(
                |u| check_command(u, "unset"),
                move |update, ctx| {
                    let s = Arc::clone(&store);
                    async move { unset_timer(update, ctx, s).await }
                },
            ),
            0,
        )
        .await;
    }

    println!("Timer bot is running. Press Ctrl+C to stop.");
    println!("Commands: /start, /set <seconds>, /unset");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
