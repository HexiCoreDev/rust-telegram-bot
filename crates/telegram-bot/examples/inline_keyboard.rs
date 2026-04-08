//! Inline Keyboard Bot -- demonstrates inline keyboard buttons and callback queries.
//!
//! This is the Rust port of Python's `inlinekeyboard.py`.
//!
//! Demonstrates:
//! - Sending messages with `InlineKeyboardMarkup`
//! - Handling callback queries with typed `Update` access
//! - `answer_callback_query` and `edit_message_text` via builders
//! - `CommandHandler::new` and `context.reply_text()` ergonomics
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p telegram-bot --example inline_keyboard
//! ```

use telegram_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, FnHandler, HandlerError, HandlerResult,
    InlineKeyboardButton, InlineKeyboardMarkup, Update,
};

// ---------------------------------------------------------------------------
// Keyboard builder
// ---------------------------------------------------------------------------

/// Build the inline keyboard for the menu.
fn build_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Option 1", "1"),
            InlineKeyboardButton::callback("Option 2", "2"),
        ],
        vec![InlineKeyboardButton::callback("Option 3", "3")],
    ])
}

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// Handle the `/start` command -- send a message with an inline keyboard.
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update
        .effective_chat()
        .map(|c| c.id)
        .expect("start requires a chat");
    let keyboard = serde_json::to_value(build_keyboard()).expect("keyboard serialization");

    context
        .bot()
        .send_message(chat_id, "Please choose an option:")
        .reply_markup(keyboard)
        .send()
        .await?;

    Ok(())
}

/// Handle the `/help` command.
async fn help(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Use /start to get an inline keyboard with options.",
        )
        .await?;
    Ok(())
}

/// Handle callback queries from inline keyboard button presses.
async fn button_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let cq = update
        .callback_query()
        .expect("callback query handler received update without callback_query");

    let data = cq.data.as_deref().unwrap_or("unknown");

    // Answer the callback query (removes the loading indicator on the client).
    context.bot().answer_callback_query(&cq.id).send().await?;

    // Edit the original message to show which option was selected.
    if let Some(msg) = cq.message.as_deref() {
        let response_text = format!("You selected: Option {data}");
        context
            .bot()
            .edit_message_text(&response_text)
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .send()
            .await?;
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

    app.add_typed_handler(CommandHandler::new("start", start), 0)
        .await;
    app.add_typed_handler(CommandHandler::new("help", help), 0)
        .await;
    app.add_typed_handler(FnHandler::on_callback_query(button_callback), 0)
        .await;

    println!("Inline keyboard bot is running. Press Ctrl+C to stop.");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
