//! Simple Echo Bot -- replies with whatever text the user sends.
//!
//! This is the Rust port of Python's `echobot.py`.
//!
//! Demonstrates:
//! - `ApplicationBuilder` with token from environment variable
//! - Command handling for `/start` and `/help` via `CommandHandler::new`
//! - Text-but-not-command filter for echoing via `MessageHandler::new`
//! - `context.reply_text()` convenience method
//! - `run_polling`
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p rust-tg-bot --example echo_bot
//! ```

use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, HandlerResult, MessageHandler, Update,
    COMMAND, TEXT,
};

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// Respond to `/start` with a greeting.
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    context
        .reply_text(
            &update,
            &format!(
        "Hi {name}! I am an echo bot. Send me any message and I will repeat it back to you.\n\n\
         Use /help to see available commands."
    ),
        )
        .await?;
    Ok(())
}

/// Respond to `/help` with usage instructions.
async fn help(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Available commands:\n\
         /start - Start the bot\n\
         /help - Show this help message\n\n\
         Send any text message and I will echo it back!",
        )
        .await?;
    Ok(())
}

/// Echo back whatever text the user sends.
async fn echo(update: Arc<Update>, context: Context) -> HandlerResult {
    let text = update
        .effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or("");
    if !text.is_empty() {
        context.reply_text(&update, text).await?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(CommandHandler::new("start", start), 0)
        .await;
    app.add_handler(CommandHandler::new("help", help), 0).await;
    app.add_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0)
        .await;

    println!("Echo bot is running. Press Ctrl+C to stop.");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
