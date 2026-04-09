//! Inline Keyboard 2 -- advanced inline keyboard with conversation-style state routing.
//!
//! This is the Rust port of Python's `inlinekeyboard2.py`.
//!
//! Demonstrates:
//! - Multi-step inline keyboard navigation
//! - Callback query data pattern matching
//! - `edit_message_text` to update menus in-place
//! - Manual conversation state tracking with `RwLock<HashMap>`
//! - Two "stages": `START_ROUTES` and `END_ROUTES`
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p rust-tg-bot --example inline_keyboard2
//! ```
//!
//! Then in Telegram:
//! - `/start` -- begins the interactive menu

use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, Context, FnHandler, HandlerError, HandlerResult, HashMap,
    InlineKeyboardButton, InlineKeyboardMarkup, JsonValue, MessageEntityType, RwLock, Update,
};

// ---------------------------------------------------------------------------
// Conversation state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    StartRoutes,
    EndRoutes,
}

type ConvStore = Arc<RwLock<HashMap<i64, Stage>>>;

// Callback data constants (mirrors Python's ONE, TWO, THREE, FOUR).
const ONE: &str = "0";
const TWO: &str = "1";
const THREE: &str = "2";
const FOUR: &str = "3";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_chat_id(update: &Update) -> i64 {
    update.effective_chat().map(|c| c.id).unwrap_or(0)
}

fn keyboard_markup_json(markup: &InlineKeyboardMarkup) -> JsonValue {
    serde_json::to_value(markup).expect("keyboard serialization")
}

// ---------------------------------------------------------------------------
// /start command handler
// ---------------------------------------------------------------------------

async fn start_command(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    let user_name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    tracing::info!("User {user_name} started the conversation.");

    let keyboard = InlineKeyboardMarkup::from_row(vec![
        InlineKeyboardButton::callback("1", ONE),
        InlineKeyboardButton::callback("2", TWO),
    ]);

    context
        .bot()
        .send_message(chat_id, "Start handler, Choose a route")
        .reply_markup(keyboard_markup_json(&keyboard))
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    conv_store.write().await.insert(chat_id, Stage::StartRoutes);

    Ok(())
}

// ---------------------------------------------------------------------------
// Callback query handlers for START_ROUTES
// ---------------------------------------------------------------------------

async fn handler_one(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let cq = update
        .callback_query()
        .expect("handler_one: missing callback_query");

    context.bot().answer_callback_query(&cq.id).await?;

    let keyboard = InlineKeyboardMarkup::from_row(vec![
        InlineKeyboardButton::callback("3", THREE),
        InlineKeyboardButton::callback("4", FOUR),
    ]);

    if let Some(msg) = cq.message.as_deref() {
        context
            .bot()
            .edit_message_text("First CallbackQueryHandler, Choose a route")
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .reply_markup(keyboard_markup_json(&keyboard))
            .await?;
    }

    let chat_id = extract_chat_id(&update);
    conv_store.write().await.insert(chat_id, Stage::StartRoutes);

    Ok(())
}

async fn handler_two(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let cq = update
        .callback_query()
        .expect("handler_two: missing callback_query");

    context.bot().answer_callback_query(&cq.id).await?;

    let keyboard = InlineKeyboardMarkup::from_row(vec![
        InlineKeyboardButton::callback("1", ONE),
        InlineKeyboardButton::callback("3", THREE),
    ]);

    if let Some(msg) = cq.message.as_deref() {
        context
            .bot()
            .edit_message_text("Second CallbackQueryHandler, Choose a route")
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .reply_markup(keyboard_markup_json(&keyboard))
            .await?;
    }

    let chat_id = extract_chat_id(&update);
    conv_store.write().await.insert(chat_id, Stage::StartRoutes);

    Ok(())
}

async fn handler_three(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let cq = update
        .callback_query()
        .expect("handler_three: missing callback_query");

    context.bot().answer_callback_query(&cq.id).await?;

    let keyboard = InlineKeyboardMarkup::from_row(vec![
        InlineKeyboardButton::callback("Yes, let's do it again!", ONE),
        InlineKeyboardButton::callback("Nah, I've had enough ...", TWO),
    ]);

    if let Some(msg) = cq.message.as_deref() {
        context
            .bot()
            .edit_message_text("Third CallbackQueryHandler. Do you want to start over?")
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .reply_markup(keyboard_markup_json(&keyboard))
            .await?;
    }

    // Transition to END_ROUTES.
    let chat_id = extract_chat_id(&update);
    conv_store.write().await.insert(chat_id, Stage::EndRoutes);

    Ok(())
}

async fn handler_four(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let cq = update
        .callback_query()
        .expect("handler_four: missing callback_query");

    context.bot().answer_callback_query(&cq.id).await?;

    let keyboard = InlineKeyboardMarkup::from_row(vec![
        InlineKeyboardButton::callback("2", TWO),
        InlineKeyboardButton::callback("3", THREE),
    ]);

    if let Some(msg) = cq.message.as_deref() {
        context
            .bot()
            .edit_message_text("Fourth CallbackQueryHandler, Choose a route")
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .reply_markup(keyboard_markup_json(&keyboard))
            .await?;
    }

    let chat_id = extract_chat_id(&update);
    conv_store.write().await.insert(chat_id, Stage::StartRoutes);

    Ok(())
}

// ---------------------------------------------------------------------------
// Callback query handlers for END_ROUTES
// ---------------------------------------------------------------------------

async fn start_over(update: Arc<Update>, context: Context, conv_store: ConvStore) -> HandlerResult {
    let cq = update
        .callback_query()
        .expect("start_over: missing callback_query");

    context.bot().answer_callback_query(&cq.id).await?;

    let keyboard = InlineKeyboardMarkup::from_row(vec![
        InlineKeyboardButton::callback("1", ONE),
        InlineKeyboardButton::callback("2", TWO),
    ]);

    if let Some(msg) = cq.message.as_deref() {
        context
            .bot()
            .edit_message_text("Start handler, Choose a route")
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .reply_markup(keyboard_markup_json(&keyboard))
            .await?;
    }

    let chat_id = extract_chat_id(&update);
    conv_store.write().await.insert(chat_id, Stage::StartRoutes);

    Ok(())
}

async fn end_conversation(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let cq = update
        .callback_query()
        .expect("end: missing callback_query");

    context.bot().answer_callback_query(&cq.id).await?;

    if let Some(msg) = cq.message.as_deref() {
        context
            .bot()
            .edit_message_text("See you next time!")
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .await?;
    }

    let chat_id = extract_chat_id(&update);
    conv_store.write().await.remove(&chat_id);

    Ok(())
}

// ---------------------------------------------------------------------------
// Predicate helpers
// ---------------------------------------------------------------------------

fn is_callback_with_data_in_stage(
    update: &Update,
    conv_store: &ConvStore,
    data: &str,
    stage: Stage,
) -> bool {
    let cq = match update.callback_query() {
        Some(cq) => cq,
        None => return false,
    };
    let d = match &cq.data {
        Some(d) => d.as_str(),
        None => return false,
    };
    if d != data {
        return false;
    }
    let chat_id = extract_chat_id(update);
    conv_store
        .try_read()
        .map(|guard| guard.get(&chat_id) == Some(&stage))
        .unwrap_or(false)
}

fn check_command(update: &Update, expected: &str) -> bool {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return false,
    };
    let text = match msg.text.as_deref() {
        Some(t) => t,
        None => return false,
    };
    let entities = match msg.entities.as_ref() {
        Some(e) => e,
        None => return false,
    };
    entities.first().map_or(false, |e| {
        if e.entity_type == MessageEntityType::BotCommand && e.offset == 0 {
            let length = e.length as usize;
            if length <= text.len() {
                let cmd_name = text[1..length].split('@').next().unwrap_or("");
                cmd_name.eq_ignore_ascii_case(expected)
            } else {
                false
            }
        } else {
            false
        }
    })
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
    let conv_store: ConvStore = Arc::new(RwLock::new(HashMap::new()));

    // Entry point: /start
    {
        let cs = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                |u| check_command(u, "start"),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { start_command(update, ctx, cs).await }
                },
            ),
            0,
        )
        .await;
    }

    // START_ROUTES: callback data "0" (ONE) -> handler_one
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| is_callback_with_data_in_stage(u, &cs_check, ONE, Stage::StartRoutes),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { handler_one(update, ctx, cs).await }
                },
            ),
            1,
        )
        .await;
    }

    // START_ROUTES: callback data "1" (TWO) -> handler_two
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| is_callback_with_data_in_stage(u, &cs_check, TWO, Stage::StartRoutes),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { handler_two(update, ctx, cs).await }
                },
            ),
            1,
        )
        .await;
    }

    // START_ROUTES: callback data "2" (THREE) -> handler_three
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| is_callback_with_data_in_stage(u, &cs_check, THREE, Stage::StartRoutes),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { handler_three(update, ctx, cs).await }
                },
            ),
            1,
        )
        .await;
    }

    // START_ROUTES: callback data "3" (FOUR) -> handler_four
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| is_callback_with_data_in_stage(u, &cs_check, FOUR, Stage::StartRoutes),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { handler_four(update, ctx, cs).await }
                },
            ),
            1,
        )
        .await;
    }

    // END_ROUTES: callback data "0" (ONE) -> start_over
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| is_callback_with_data_in_stage(u, &cs_check, ONE, Stage::EndRoutes),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { start_over(update, ctx, cs).await }
                },
            ),
            1,
        )
        .await;
    }

    // END_ROUTES: callback data "1" (TWO) -> end_conversation
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| is_callback_with_data_in_stage(u, &cs_check, TWO, Stage::EndRoutes),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { end_conversation(update, ctx, cs).await }
                },
            ),
            1,
        )
        .await;
    }

    println!("Inline keyboard 2 bot is running. Press Ctrl+C to stop.");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
