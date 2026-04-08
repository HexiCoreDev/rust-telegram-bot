//! Context Types Bot -- demonstrates custom context types with per-user and per-chat data.
//!
//! This is the Rust port of Python's `contexttypesbot.py`.
//!
//! Demonstrates:
//! - Custom per-chat data via `context.chat_data()` / `context.set_chat_data()`
//! - Bot-wide data via `context.bot_data()` / `context.bot_data_mut()`
//! - Tracking user IDs across all updates (group -1 handler)
//! - Click counting per message via callback queries
//! - `send_message` with inline keyboard JSON
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p telegram-bot --example context_types_bot
//! ```

use std::sync::Arc;

use serde_json::json;

use telegram_bot::ext::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const CLICKS_KEY_PREFIX: &str = "clicks_msg_";
const USER_IDS_KEY: &str = "user_ids";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_chat_id(update: &Update) -> i64 {
    update.effective_chat().expect("update must have a chat").id
}

fn build_click_keyboard() -> serde_json::Value {
    json!({
        "inline_keyboard": [[
            {"text": "Click me!", "callback_data": "button"}
        ]]
    })
}

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// Track every user that interacts with the bot in bot_data.
async fn track_users(update: Arc<Update>, context: Context) -> HandlerResult {
    if let Some(user) = update.effective_user() {
        let user_id_str = user.id.to_string();
        let mut bd = context.bot_data_mut().await;
        let ids = bd
            .entry(USER_IDS_KEY.to_string())
            .or_insert_with(|| serde_json::Value::Array(vec![]));
        if let serde_json::Value::Array(ref mut arr) = ids {
            let val = serde_json::Value::String(user_id_str.clone());
            if !arr.contains(&val) {
                arr.push(val);
            }
        }
    }
    Ok(())
}

/// `/start` -- send a message with a click counter button.
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    context
        .bot()
        .send_message(chat_id, "This button was clicked 0 times.")
        .reply_markup(build_click_keyboard())
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// Handle button clicks -- increment a per-message click counter stored in chat_data.
async fn count_click(update: Arc<Update>, context: Context) -> HandlerResult {
    let cq = update
        .callback_query
        .as_ref()
        .expect("callback query handler requires callback_query");

    // Answer the callback query to dismiss the loading indicator.
    context
        .bot()
        .answer_callback_query(&cq.id)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    // Get the message ID to use as the key for click counting.
    let msg = cq
        .message
        .as_ref()
        .expect("callback query must have a message");
    let msg_id = msg.message_id();
    let chat_id = msg.chat().id;
    let key = format!("{CLICKS_KEY_PREFIX}{msg_id}");

    // Read current clicks, increment, and store back.
    let current: i64 = context
        .chat_data()
        .await
        .and_then(|cd| cd.get(&key).cloned())
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let new_count = current + 1;

    context
        .set_chat_data(key, serde_json::Value::Number(new_count.into()))
        .await;

    // Edit the message to show the updated count.
    let text = format!("This button was clicked {new_count} times.");
    context
        .bot()
        .edit_message_text(&text)
        .chat_id(chat_id)
        .message_id(msg_id)
        .reply_markup(build_click_keyboard())
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// `/print_users` -- show all user IDs that have interacted with the bot.
async fn print_users(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    let bd = context.bot_data().await;
    let user_ids_text = bd
        .get(USER_IDS_KEY)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "No users tracked yet.".to_string());

    let text = format!("The following user IDs have used this bot: {user_ids_text}");

    context
        .bot()
        .send_message(chat_id, &text)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();

        let token = std::env::var("TELEGRAM_BOT_TOKEN")
            .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

        let app: Arc<Application> = ApplicationBuilder::new().token(token).build();

        // Group -1: track all users before any other handler runs.
        app.add_typed_handler(FnHandler::on_any(track_users), -1)
            .await;

        // /start -- send a button
        app.add_typed_handler(CommandHandler::new("start", start), 0)
            .await;

        // Callback query handler for button clicks
        app.add_typed_handler(
            FnHandler::new(
                |u| {
                    u.callback_query
                        .as_ref()
                        .and_then(|cq| cq.data.as_deref())
                        .map_or(false, |d| d == "button")
                },
                count_click,
            ),
            0,
        )
        .await;

        // /print_users -- show tracked user IDs
        app.add_typed_handler(CommandHandler::new("print_users", print_users), 0)
            .await;

        println!("Context types bot is running. Press Ctrl+C to stop.");
        println!("Commands: /start, /print_users");

        if let Err(e) = app.run_polling().await {
            eprintln!("Error running bot: {e}");
        }
    });
}
