//! RTB benchmark bot -- identical features to PTB and teloxide versions.
//!
//! Features: /start (with inline keyboard), /help, echo with typing action,
//! callback query handler, webhook on port 8000 via `run_webhook()`.
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token" \
//! WEBHOOK_URL="https://your.domain" \
//! cargo run -p rust-tg-bot --example bench_bot --features webhooks
//! ```

use std::sync::Arc;

use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, CommandHandler, Context, FnHandler, HandlerResult, InlineKeyboardButton,
    InlineKeyboardMarkup, MessageHandler, Update, COMMAND, TEXT,
};
use rust_tg_bot::ext::updater::WebhookConfig;
use rust_tg_bot::raw::bot::ChatId;

// -- Handlers ----------------------------------------------------------------

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    let chat_id = update.effective_chat().map(|c| c.id).unwrap_or(0);

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Option 1", "1"),
            InlineKeyboardButton::callback("Option 2", "2"),
        ],
        vec![InlineKeyboardButton::callback("Option 3", "3")],
    ]);

    context
        .bot()
        .send_message(
            chat_id,
            &format!("Hi {name}! I am a benchmark bot.\nUse /help for info."),
        )
        .reply_markup(serde_json::to_value(&keyboard).unwrap_or_default())
        .await?;
    Ok(())
}

async fn help_cmd(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Commands: /start, /help\nSend any text to echo.\nPress inline buttons to test callbacks.",
        )
        .await?;
    Ok(())
}

async fn echo(update: Arc<Update>, context: Context) -> HandlerResult {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return Ok(()),
    };
    let chat_id = msg.chat.id;
    let text = msg.text.as_deref().unwrap_or("");
    if text.is_empty() {
        return Ok(());
    }

    context
        .bot()
        .send_chat_action(ChatId::Id(chat_id), "typing")
        .await
        .ok();
    context.bot().send_message(chat_id, text).await?;
    Ok(())
}

async fn button_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let cq = match update.callback_query() {
        Some(c) => c,
        None => return Ok(()),
    };
    let data = cq.data.as_deref().unwrap_or("?");
    context.bot().answer_callback_query(&cq.id).await?;
    if let Some(msg) = cq.message.as_ref().and_then(|m| m.as_message()) {
        context
            .bot()
            .edit_message_text(&format!("You selected: Option {data}"))
            .chat_id(msg.chat.id)
            .message_id(msg.message_id)
            .await?;
    }
    Ok(())
}

// -- Main --------------------------------------------------------------------

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN required");
    let webhook_url = std::env::var("WEBHOOK_URL").expect("WEBHOOK_URL required");

    let app = ApplicationBuilder::new().token(&token).build();

    app.add_handler(CommandHandler::new("start", start), 0)
        .await;
    app.add_handler(CommandHandler::new("help", help_cmd), 0)
        .await;
    app.add_handler(FnHandler::on_callback_query(button_callback), 0)
        .await;
    app.add_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0)
        .await;

    println!("RTB benchmark bot starting on port 8000. Send /start to test.");

    let config = WebhookConfig::new(format!("{webhook_url}/telegram"))
        .port(8000)
        .url_path("/telegram")
        .secret_token("bench-secret-token");

    if let Err(e) = app.run_webhook(config).await {
        eprintln!("Error: {e}");
    }
}
