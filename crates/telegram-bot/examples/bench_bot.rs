//! RTB benchmark bot -- identical features to PTB and teloxide versions.
//!
//! Features: /start (with inline keyboard), /help, echo with typing action,
//! callback query handler, custom webhook on port 8000.
//!
//! Run: Copy this to examples/ or run directly referencing the workspace crates.
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

use telegram_bot::ext::prelude::{
    ApplicationBuilder, Arc, ChatId, CommandHandler, Context, FnHandler, HandlerResult,
    InlineKeyboardButton, InlineKeyboardMarkup, MessageHandler, Update, COMMAND, TEXT,
};
use telegram_bot::raw::types::update::Update as RawUpdate;

// -- Handlers ----------------------------------------------------------------

async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    let chat_id = update.effective_chat().map(|c| c.id).unwrap_or(0);

    let keyboard = serde_json::to_value(InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Option 1", "1"),
            InlineKeyboardButton::callback("Option 2", "2"),
        ],
        vec![InlineKeyboardButton::callback("Option 3", "3")],
    ]))
    .expect("keyboard serialization");

    context
        .bot()
        .send_message(
            chat_id,
            &format!("Hi {name}! I am a benchmark bot.\nUse /help for info."),
        )
        .reply_markup(keyboard)
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

// -- Webhook -----------------------------------------------------------------

#[derive(Clone)]
struct AppState {
    update_tx: mpsc::Sender<RawUpdate>,
}

async fn handle_webhook(
    State(state): State<AppState>,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    match serde_json::from_slice::<RawUpdate>(&body) {
        Ok(update) => {
            let _ = state.update_tx.send(update).await;
            StatusCode::OK
        }
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

async fn healthcheck() -> &'static str {
    "OK"
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

    app.add_typed_handler(CommandHandler::new("start", start), 0)
        .await;
    app.add_typed_handler(CommandHandler::new("help", help_cmd), 0)
        .await;
    app.add_typed_handler(FnHandler::on_callback_query(button_callback), 0)
        .await;
    app.add_typed_handler(MessageHandler::new(TEXT() & !COMMAND(), echo), 0)
        .await;

    app.initialize().await.expect("init failed");
    app.start().await.expect("start failed");

    app.bot()
        .set_webhook(&format!("{webhook_url}/telegram"))
        .await
        .expect("set_webhook failed");

    let state = AppState {
        update_tx: app.update_sender(),
    };
    let router = Router::new()
        .route("/telegram", post(handle_webhook))
        .route("/healthcheck", get(healthcheck))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("RTB benchmark bot running on port 8000. Send /start to test.");

    let stop = Arc::new(tokio::sync::Notify::new());
    let stop2 = stop.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        stop2.notify_waiters();
    });

    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            stop.notified().await;
        })
        .await
        .ok();

    app.stop().await.ok();
    app.shutdown().await.ok();
}
