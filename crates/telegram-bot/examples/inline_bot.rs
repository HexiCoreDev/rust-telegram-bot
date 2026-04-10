//! Inline Bot -- demonstrates inline query handling with text transformations.
//!
//! This is the Rust port of Python's `inlinebot.py`.
//!
//! Demonstrates:
//! - Handling `inline_query` updates
//! - Building `InlineQueryResultArticle` typed responses
//! - `answer_inline_query` builder
//! - Text transforms: CAPS, **bold**, *italic*
//!
//! **Important:** Enable inline mode with @BotFather before running this bot.
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p rust-tg-bot --example inline_bot
//! ```
use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, FnHandler, HandlerResult, ParseMode, Update,
};
use rust_tg_bot::raw::types::inline::inline_query_result_article::InlineQueryResultArticle;
use rust_tg_bot::raw::types::inline::input_message_content::InputMessageContent;
use rust_tg_bot::raw::types::inline::input_text_message_content::InputTextMessageContent;

// ---------------------------------------------------------------------------
// Command handlers
// ---------------------------------------------------------------------------

/// `/start` -- simple greeting.
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Hi! Use me inline by typing @botusername <query> in any chat.",
        )
        .await?;
    Ok(())
}

/// `/help` -- usage info.
async fn help_command(update: Arc<Update>, context: Context) -> HandlerResult {
    context.reply_text(&update, "Type @botusername <text> in any chat. I will offer CAPS, Bold, and Italic transformations.").await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Inline query handler
// ---------------------------------------------------------------------------

/// Handle inline queries by offering three text transformations.
async fn inline_query_handler(update: Arc<Update>, context: Context) -> HandlerResult {
    let iq = match update.inline_query() {
        Some(q) => q,
        None => return Ok(()),
    };

    let query = &iq.query;

    // Empty queries should not be handled.
    if query.is_empty() {
        return Ok(());
    }

    // Escape HTML special characters for bold/italic results.
    let escaped = query
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");

    let caps_content = InputTextMessageContent::new(query.to_uppercase());
    let bold_content = InputTextMessageContent::new(format!("<b>{escaped}</b>")).parse_mode("HTML");
    let italic_content =
        InputTextMessageContent::new(format!("<i>{escaped}</i>")).parse_mode("HTML");

    let results = vec![
        serde_json::to_value(InlineQueryResultArticle::new(
            format!("caps-{}", iq.id),
            "Caps",
            InputMessageContent::Text(caps_content),
        ))
        .expect("article serialization"),
        serde_json::to_value(InlineQueryResultArticle::new(
            format!("bold-{}", iq.id),
            "Bold",
            InputMessageContent::Text(bold_content),
        ))
        .expect("article serialization"),
        serde_json::to_value(InlineQueryResultArticle::new(
            format!("italic-{}", iq.id),
            "Italic",
            InputMessageContent::Text(italic_content),
        ))
        .expect("article serialization"),
    ];

    context
        .bot()
        .answer_inline_query(&iq.id, results)
        .await?;

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

    // Command handlers.
    app.add_handler(CommandHandler::new("start", start), 0)
        .await;
    app.add_handler(CommandHandler::new("help", help_command), 0)
        .await;

    // Inline query handler.
    app.add_handler(FnHandler::on_inline_query(inline_query_handler), 0)
        .await;

    println!("Inline bot is running. Press Ctrl+C to stop.");
    println!("Remember to enable inline mode with @BotFather!");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
