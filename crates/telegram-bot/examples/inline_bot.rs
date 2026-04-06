//! Inline Bot -- demonstrates inline query handling with text transformations.
//!
//! This is the Rust port of Python's `inlinebot.py`.
//!
//! Demonstrates:
//! - Handling `inline_query` updates
//! - Building `InlineQueryResultArticle` JSON responses
//! - `answer_inline_query` builder
//! - Text transforms: CAPS, **bold**, *italic*
//!
//! **Important:** Enable inline mode with @BotFather before running this bot.
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p telegram-bot --example inline_bot
//! ```

use serde_json::json;

use telegram_bot::ext::prelude::*;

// ---------------------------------------------------------------------------
// Command handlers
// ---------------------------------------------------------------------------

/// `/start` -- simple greeting.
async fn start(update: Update, context: Context) -> HandlerResult {
    context.reply_text(&update, "Hi! Use me inline by typing @botusername <query> in any chat.").await?;
    Ok(())
}

/// `/help` -- usage info.
async fn help_command(update: Update, context: Context) -> HandlerResult {
    context.reply_text(&update, "Type @botusername <text> in any chat. I will offer CAPS, Bold, and Italic transformations.").await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Inline query handler
// ---------------------------------------------------------------------------

/// Handle inline queries by offering three text transformations.
async fn inline_query_handler(update: Update, context: Context) -> HandlerResult {
    let iq = match &update.inline_query {
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

    let results = vec![
        json!({
            "type": "article",
            "id": format!("caps-{}", iq.id),
            "title": "Caps",
            "input_message_content": {
                "message_text": query.to_uppercase(),
            },
        }),
        json!({
            "type": "article",
            "id": format!("bold-{}", iq.id),
            "title": "Bold",
            "input_message_content": {
                "message_text": format!("<b>{escaped}</b>"),
                "parse_mode": ParseMode::Html,
            },
        }),
        json!({
            "type": "article",
            "id": format!("italic-{}", iq.id),
            "title": "Italic",
            "input_message_content": {
                "message_text": format!("<i>{escaped}</i>"),
                "parse_mode": ParseMode::Html,
            },
        }),
    ];

    context
        .bot()
        .answer_inline_query(&iq.id, results)
        .send()
        .await?;

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

        let app = ApplicationBuilder::new().token(token).build();

        // Command handlers.
        app.add_typed_handler(CommandHandler::new("start", start), 0)
            .await;
        app.add_typed_handler(CommandHandler::new("help", help_command), 0)
            .await;

        // Inline query handler.
        app.add_typed_handler(FnHandler::on_inline_query(inline_query_handler), 0)
            .await;

        println!("Inline bot is running. Press Ctrl+C to stop.");
        println!("Remember to enable inline mode with @BotFather!");

        if let Err(e) = app.run_polling().await {
            eprintln!("Error running bot: {e}");
        }
    });
}
