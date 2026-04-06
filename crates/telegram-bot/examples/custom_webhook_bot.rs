//! Custom Webhook Bot -- a bot with custom axum routes alongside Telegram.
//!
//! This is the Rust port of Python's `customwebhookbot/starlettebot.py`.
//! Where Python uses Starlette + uvicorn, Rust uses axum + tokio.
//!
//! Demonstrates:
//! - A custom axum web server running alongside the Telegram bot
//! - `POST /telegram` endpoint that receives Telegram webhook updates
//! - `GET /healthcheck` endpoint for liveness probes
//! - `GET /submitpayload` endpoint that accepts custom webhook payloads
//! - Forwarding custom payloads to an admin chat via `send_message`
//! - `get_chat_member` to resolve user display names
//! - Running the bot's update processing and the web server concurrently
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token" \
//! WEBHOOK_URL="https://your.domain" \
//! ADMIN_CHAT_ID="123456" \
//! PORT="8000" \
//! cargo run -p telegram-bot --example custom_webhook_bot --features webhooks
//! ```

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

use telegram_bot::ext::prelude::*;
use telegram_bot::raw::bot::ChatId;
use telegram_bot::raw::types::chat_member::ChatMember;
use telegram_bot::raw::types::update::Update as RawUpdate;

// ---------------------------------------------------------------------------
// Configuration constants
// ---------------------------------------------------------------------------

/// Default port when `PORT` is not set.
const DEFAULT_PORT: u16 = 8000;

/// Default listen address.
const LISTEN_ADDR: &str = "127.0.0.1";

/// Webhook base URL, set once at startup from the `WEBHOOK_URL` env var.
static WEBHOOK_BASE_URL: std::sync::OnceLock<String> = std::sync::OnceLock::new();

/// Telegram webhook URL path.
const TELEGRAM_WEBHOOK_PATH: &str = "/telegram";

/// Health check endpoint path.
const HEALTHCHECK_PATH: &str = "/healthcheck";

/// Custom payload submission endpoint path.
const SUBMIT_PAYLOAD_PATH: &str = "/submitpayload";

// ---------------------------------------------------------------------------
// Shared application state for axum routes
// ---------------------------------------------------------------------------

/// State shared with all axum route handlers.
///
/// Holds the update channel, the bot reference (reused from the Application),
/// and configuration needed by the custom endpoints.
#[derive(Clone)]
struct AppState {
    /// Channel to forward Telegram updates into the Application's processing loop.
    update_tx: mpsc::UnboundedSender<RawUpdate>,
    /// The Application's bot, for sending messages from custom routes.
    app: Arc<Application>,
    /// Admin chat ID to receive forwarded custom payloads.
    admin_chat_id: i64,
    /// The public webhook URL, captured at startup.
    webhook_url: String,
}

// ---------------------------------------------------------------------------
// Custom webhook payload
// ---------------------------------------------------------------------------

/// Query parameters for the `/submitpayload` endpoint.
///
/// Mirrors Python's `WebhookUpdate` dataclass -- `user_id` and `payload`
/// arrive as query parameters rather than as a Telegram update.
#[derive(Debug, Deserialize)]
struct SubmitPayloadParams {
    user_id: Option<i64>,
    payload: Option<String>,
}

// ---------------------------------------------------------------------------
// Telegram bot handler: /start
// ---------------------------------------------------------------------------

/// Respond to `/start` with instructions on how to use the custom endpoints.
async fn start(update: Update, context: Context) -> HandlerResult {
    let webhook_url = WEBHOOK_BASE_URL.get().map(|s| s.as_str()).unwrap_or("https://your.domain");

    let text = format!(
        "To check if the bot is still running, call <code>{webhook_url}{HEALTHCHECK_PATH}</code>.\n\n\
         To post a custom update, call \
         <code>{webhook_url}{SUBMIT_PAYLOAD_PATH}?user_id=&lt;your user id&gt;&amp;payload=&lt;payload&gt;</code>."
    );

    let chat_id = update
        .effective_chat()
        .map(|c| c.id)
        .expect("start command must originate from a chat");

    context
        .bot()
        .send_message(chat_id, &text)
        .parse_mode(ParseMode::Html)
        .send()
        .await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Axum route handlers
// ---------------------------------------------------------------------------

/// `POST /telegram` -- receives Telegram webhook updates and forwards them
/// into the Application's update processing channel.
async fn handle_telegram_webhook(
    State(state): State<AppState>,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    let update: RawUpdate = match serde_json::from_slice(&body) {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Failed to parse Telegram update: {e}");
            return StatusCode::BAD_REQUEST;
        }
    };

    if let Err(e) = state.update_tx.send(update) {
        tracing::error!("Failed to enqueue Telegram update: {e}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

/// `GET /healthcheck` -- simple liveness probe.
async fn handle_healthcheck() -> &'static str {
    "The bot is still running fine :)"
}

/// `GET /submitpayload` -- accepts a custom payload via query parameters,
/// looks up the user via `get_chat_member`, and forwards the payload to
/// the admin chat.
///
/// This mirrors Python's `custom_updates` handler in `starlettebot.py`.
async fn handle_submit_payload(
    State(state): State<AppState>,
    Query(params): Query<SubmitPayloadParams>,
) -> impl IntoResponse {
    let user_id = match params.user_id {
        Some(id) => id,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "Please pass both `user_id` and `payload` as query parameters.",
            )
                .into_response();
        }
    };

    let payload = match params.payload {
        Some(ref p) if !p.is_empty() => p.clone(),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                "Please pass both `user_id` and `payload` as query parameters.",
            )
                .into_response();
        }
    };

    // Attempt to resolve the user's display name via get_chat_member.
    let bot = state.app.bot();
    let user_mention = match bot
        .get_chat_member(ChatId::Id(user_id), user_id)
        .await
    {
        Ok(member) => {
            let user = extract_user_from_member(&member);
            format!(
                "<a href=\"tg://user?id={user_id}\">{}</a>",
                html_escape(&user.first_name)
            )
        }
        Err(_) => format!("User {user_id}"),
    };

    let text = format!(
        "The user {user_mention} has sent a new payload: \n\n\
         <code>{}</code>",
        html_escape(&payload)
    );

    if let Err(e) = bot
        .send_message(state.admin_chat_id, &text)
        .parse_mode(ParseMode::Html)
        .send()
        .await
    {
        tracing::error!("Failed to forward payload to admin chat: {e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to forward the payload.",
        )
            .into_response();
    }

    (
        StatusCode::OK,
        "Thank you for the submission! It's being forwarded.",
    )
        .into_response()
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract the `User` reference from any `ChatMember` variant.
fn extract_user_from_member(
    member: &ChatMember,
) -> &telegram_bot::raw::types::user::User {
    match member {
        ChatMember::Owner(m) => &m.user,
        ChatMember::Administrator(m) => &m.user,
        ChatMember::Member(m) => &m.user,
        ChatMember::Restricted(m) => &m.user,
        ChatMember::Left(m) => &m.user,
        ChatMember::Banned(m) => &m.user,
    }
}

/// Minimal HTML escaping for safe embedding in Telegram HTML messages.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();

        // -- Read configuration from environment --------------------------------
        let token = std::env::var("TELEGRAM_BOT_TOKEN")
            .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

        let webhook_url = std::env::var("WEBHOOK_URL")
            .expect("WEBHOOK_URL environment variable must be set");

        let admin_chat_id: i64 = std::env::var("ADMIN_CHAT_ID")
            .expect("ADMIN_CHAT_ID environment variable must be set")
            .parse()
            .expect("ADMIN_CHAT_ID must be a valid integer");

        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse()
            .expect("PORT must be a valid u16");

        // -- Build the Application ----------------------------------------------
        let app = ApplicationBuilder::new().token(&token).build();

        // Store webhook URL in a static so handlers can access it without magic string keys.
        WEBHOOK_BASE_URL.set(webhook_url.clone()).ok();

        // -- Register handlers --------------------------------------------------
        app.add_typed_handler(CommandHandler::new("start", start), 0).await;

        // -- Initialize and start the Application (without the built-in updater) -
        // Calling initialize + start manually (instead of run_webhook) lets us
        // run our own axum server alongside the Application's update processor.
        app.initialize().await.expect("Failed to initialize application");
        app.start().await.expect("Failed to start application");

        // -- Set the webhook on Telegram's side ---------------------------------
        let full_webhook_url = format!("{webhook_url}{TELEGRAM_WEBHOOK_PATH}");
        app.bot()
            .set_webhook(&full_webhook_url)
            .send()
            .await
            .expect("Failed to set webhook");

        tracing::info!("Webhook set to {full_webhook_url}");

        // -- Build the custom axum router ---------------------------------------
        let state = AppState {
            update_tx: app.update_sender(),
            app: Arc::clone(&app),
            admin_chat_id,
            webhook_url,
        };

        let router = Router::new()
            .route(TELEGRAM_WEBHOOK_PATH, post(handle_telegram_webhook))
            .route(HEALTHCHECK_PATH, get(handle_healthcheck))
            .route(SUBMIT_PAYLOAD_PATH, get(handle_submit_payload))
            .with_state(state);

        // -- Bind and serve the web server --------------------------------------
        let addr = format!("{LISTEN_ADDR}:{port}");
        let listener = TcpListener::bind(&addr)
            .await
            .unwrap_or_else(|e| panic!("Failed to bind to {addr}: {e}"));

        tracing::info!("Custom webhook server listening on {addr}");
        println!(
            "Bot is running.\n  Webhook: {TELEGRAM_WEBHOOK_PATH}\n  Health:  {HEALTHCHECK_PATH}\n  Payload: {SUBMIT_PAYLOAD_PATH}\nPress Ctrl+C to stop."
        );

        // -- Run web server with graceful shutdown on Ctrl+C --------------------
        let stop_notify = Arc::new(tokio::sync::Notify::new());
        let stop_for_signal = stop_notify.clone();

        tokio::spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen for Ctrl+C");
            tracing::info!("Received Ctrl+C, shutting down...");
            stop_for_signal.notify_waiters();
        });

        let shutdown = stop_notify.clone();
        axum::serve(listener, router)
            .with_graceful_shutdown(async move {
                shutdown.notified().await;
            })
            .await
            .expect("Web server error");

        // -- Teardown -----------------------------------------------------------
        if let Err(e) = app.stop().await {
            tracing::warn!("Error stopping application: {e}");
        }
        if let Err(e) = app.shutdown().await {
            tracing::warn!("Error shutting down application: {e}");
        }

        tracing::info!("Bot stopped.");
    });
}
