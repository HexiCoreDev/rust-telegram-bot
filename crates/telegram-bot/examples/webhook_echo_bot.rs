//! Webhook Echo Bot -- the echo bot served over HTTPS via a webhook.
//!
//! Same handlers as `echo_bot`, but updates arrive through `run_webhook`
//! instead of long polling. Useful for verifying that a real Telegram
//! delivery round-trips through `setWebhook`, the axum handler, and the
//! secret-token check.
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token" \
//! WEBHOOK_URL="https://example.ngrok.io" \
//! WEBHOOK_SECRET="some-secret" \
//! PORT="8443" \
//! cargo run -p rust-tg-bot --example webhook_echo_bot --features webhooks
//! ```
//!
//! `WEBHOOK_URL` is the publicly reachable base URL; the update endpoint is
//! mounted at `WEBHOOK_PATH` (default `/telegram/webhook`) beneath it.

use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, CommandHandler, Context, HandlerResult, MessageHandler, Update, COMMAND,
    TEXT,
};

/// Respond to `/start`.
async fn start(update: std::sync::Arc<Update>, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    context
        .reply_text(
            &update,
            &format!("Hi {name}! I am an echo bot running on a *webhook*. Send me anything."),
        )
        .await?;
    Ok(())
}

/// Echo back whatever text the user sends.
async fn echo(update: std::sync::Arc<Update>, context: Context) -> HandlerResult {
    let text = update
        .effective_message()
        .and_then(|m| m.text.as_deref())
        .unwrap_or("");
    if !text.is_empty() {
        context.reply_text(&update, text).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8443);
    let url_path = std::env::var("WEBHOOK_PATH").unwrap_or_else(|_| "/telegram/webhook".to_owned());
    let webhook_url = std::env::var("WEBHOOK_URL").ok();
    let secret_token = std::env::var("WEBHOOK_SECRET").ok();

    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(CommandHandler::new("start", start), 0).await;
    app.add_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0)
        .await;

    println!("Webhook echo bot listening on 127.0.0.1:{port}{url_path}");

    let config = rust_tg_bot::ext::updater::WebhookConfig {
        listen: "127.0.0.1".to_owned(),
        port,
        url_path,
        webhook_url,
        secret_token,
        ..Default::default()
    };

    if let Err(e) = app.run_webhook(config).await {
        eprintln!("Error running webhook bot: {e}");
    }
}
