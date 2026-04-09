//! Simple Webhook Bot -- demonstrates the built-in webhook server.
//!
//! This is the simplest way to run a bot in webhook mode. The framework
//! handles the axum server, secret token validation, and update dispatching
//! internally. No manual axum setup needed.
//!
//! For custom routes alongside the webhook, see `custom_webhook_bot.rs`.
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token" \
//! WEBHOOK_URL="https://your.domain" \
//! cargo run -p rust-tg-bot --example webhook_bot --features webhooks
//! ```

use std::sync::Arc;

use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, CommandHandler, Context, FnHandler, HandlerResult, InlineKeyboardButton,
    InlineKeyboardMarkup, MessageEntityType, MessageHandler, ParseMode, Update, COMMAND, TEXT,
};
use rust_tg_bot::ext::updater::WebhookConfig;

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");

    let keyboard = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("Ping", "ping"),
        InlineKeyboardButton::callback("Info", "info"),
    ]]);

    let chat_id = update.effective_chat().map(|c| c.id).unwrap_or(0);
    context
        .bot()
        .send_message(chat_id, &format!("Hi {name}! I am running via webhook."))
        .parse_mode(ParseMode::Html)
        .reply_markup(serde_json::to_value(&keyboard).unwrap_or_default())
        .await?;
    Ok(())
}

async fn help(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(&update, "Commands: /start, /help\nSend any text to echo.")
        .await?;
    Ok(())
}

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

async fn button(update: Arc<Update>, context: Context) -> HandlerResult {
    context.answer_callback_query(&update).await?;
    let data = update
        .callback_query()
        .and_then(|cq| cq.data.as_deref())
        .unwrap_or("unknown");
    let reply = match data {
        "ping" => "Pong!",
        "info" => "rust-tg-bot v1.0.0-beta.2 -- webhook mode",
        _ => "Unknown button",
    };
    context.edit_callback_message_text(&update, reply).await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    let webhook_url = std::env::var("WEBHOOK_URL").expect("WEBHOOK_URL must be set");
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".into())
        .parse()
        .expect("PORT must be a number");

    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(CommandHandler::new("start", start), 0)
        .await;
    app.add_handler(CommandHandler::new("help", help), 0).await;
    app.add_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0)
        .await;
    app.add_handler(FnHandler::on_callback_query(button), 0)
        .await;

    println!("Webhook bot starting on port {port}...");

    let config = WebhookConfig::new(format!("{webhook_url}/telegram"))
        .port(port)
        .url_path("/telegram")
        .secret_token("my-secret-token");

    if let Err(e) = app.run_webhook(config).await {
        eprintln!("Error: {e}");
    }
}
