//! Error Handler Bot -- demonstrates custom error handler registration.
//!
//! This is the Rust port of Python's `errorhandlerbot.py`.
//!
//! Demonstrates:
//! - `app.add_error_handler()` registration
//! - Logging errors via `tracing`
//! - Sending error details to a developer chat
//! - A `/bad_command` that intentionally triggers an error
//! - `context.error` inspection inside error handlers
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" \
//! DEVELOPER_CHAT_ID="your-chat-id" \
//! cargo run -p telegram-bot --example error_handler_bot
//! ```

use std::sync::Arc;

use telegram_bot::ext::application;
use telegram_bot::ext::prelude::*;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Read the developer chat ID from the environment. This is the chat where
/// error reports will be sent. Use `/start` to discover your own chat ID.
fn developer_chat_id() -> i64 {
    std::env::var("DEVELOPER_CHAT_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// Respond to `/start` -- show chat ID and usage instructions.
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap_or(0);
    let text = format!(
        "Use /bad_command to cause an error.\n\
         Your chat id is {}.",
        chat_id
    );
    context.reply_text(&update, &text).await?;
    Ok(())
}

/// Intentionally trigger an error so the error handler fires.
async fn bad_command(_update: Arc<Update>, _context: Context) -> HandlerResult {
    // Simulate an application error by returning an Err.
    Err(application::HandlerError::Other(Box::new(
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "This is a deliberately triggered error from /bad_command!",
        ),
    )))
}

// ---------------------------------------------------------------------------
// Error handler
// ---------------------------------------------------------------------------

/// Log the error and send a Telegram message to the developer chat.
///
/// Mirrors Python's `error_handler` which formats the traceback and sends
/// it to `DEVELOPER_CHAT_ID`.
async fn error_handler(update: Option<Arc<Update>>, context: CallbackContext) -> bool {
    let error_text = context
        .error
        .as_ref()
        .map(|e| format!("{e}"))
        .unwrap_or_else(|| "Unknown error".to_string());

    tracing::error!("Exception while handling an update: {error_text}");

    // Build a diagnostic message with update details.
    let update_str = update
        .as_ref()
        .map(|u| format!("{u:?}"))
        .unwrap_or_else(|| "No update".to_string());

    let chat_data_str = context
        .chat_data()
        .await
        .map(|d| format!("{d:?}"))
        .unwrap_or_else(|| "None".to_string());

    let user_data_str = context
        .user_data()
        .await
        .map(|d| format!("{d:?}"))
        .unwrap_or_else(|| "None".to_string());

    // Truncate to respect the 4096-char Telegram limit.
    let message = format!(
        "An exception was raised while handling an update\n\n\
         update = {update_str}\n\n\
         chat_data = {chat_data_str}\n\n\
         user_data = {user_data_str}\n\n\
         error = {error_text}"
    );
    let message = if message.len() > 4000 {
        format!("{}...(truncated)", &message[..4000])
    } else {
        message
    };

    let dev_id = developer_chat_id();
    if dev_id != 0 {
        let _ = context.bot().send_message(dev_id, &message).send().await;
    } else {
        tracing::warn!("DEVELOPER_CHAT_ID not set -- error report was not sent to Telegram.");
    }

    // Return false so other error handlers (if any) can also run.
    false
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();

        let token = std::env::var("TELEGRAM_BOT_TOKEN")
            .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

        let app = ApplicationBuilder::new().token(token).build();

        // Register command handlers.
        app.add_typed_handler(CommandHandler::new("start", start), 0)
            .await;
        app.add_typed_handler(CommandHandler::new("bad_command", bad_command), 0)
            .await;

        // Register the error handler.
        app.add_error_handler(
            Arc::new(|update, ctx| Box::pin(error_handler(update, ctx))),
            true,
        )
        .await;

        println!("Error handler bot is running. Press Ctrl+C to stop.");

        if let Err(e) = app.run_polling().await {
            eprintln!("Error running bot: {e}");
        }
    });
}
