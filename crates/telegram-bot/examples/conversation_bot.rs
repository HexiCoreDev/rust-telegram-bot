//! Conversation Bot -- demonstrates a multi-step conversation handler.
//!
//! This is the Rust port of Python's `conversationbot.py`.
//!
//! Demonstrates:
//! - Manual conversation state tracking with `RwLock<HashMap>`
//! - Entry points and fallbacks
//! - State transitions
//! - `/cancel` to end the conversation
//! - **Typed `Update` access** -- `update.effective_message().unwrap().chat.id`
//!
//! The bot asks the user for their name, then their age, then a location,
//! and finally a short bio. Each step transitions to the next state.
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p telegram-bot --example conversation_bot
//! ```
//!
//! Then in Telegram:
//! - `/start` -- begins the conversation
//! - `/cancel` -- cancels at any point

use telegram_bot::ext::prelude::{
    Application, ApplicationBuilder, Context, FnHandler, HandlerError, HandlerResult,
    MessageEntityType, Update, Arc, HashMap, RwLock,
};

// ---------------------------------------------------------------------------
// Conversation state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConvState {
    AskName,
    AskAge,
    AskLocation,
    AskBio,
}

type ConvStore = Arc<RwLock<HashMap<i64, ConvState>>>;
type UserDataStore = Arc<RwLock<HashMap<i64, UserProfile>>>;

#[derive(Debug, Clone, Default)]
struct UserProfile {
    name: Option<String>,
    age: Option<String>,
    location: Option<String>,
    bio: Option<String>,
}

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

async fn start_command(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    conv_store.write().await.insert(chat_id, ConvState::AskName);

    context
        .bot()
        .send_message(
            chat_id,
            "Hi! I would like to get to know you.\n\n\
             What is your name? (Send /cancel to stop at any time.)",
        )
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

async fn receive_name(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
    user_data: UserDataStore,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let text = extract_text(&update).unwrap_or_default();

    user_data.write().await.entry(chat_id).or_default().name = Some(text.to_string());
    conv_store.write().await.insert(chat_id, ConvState::AskAge);

    context
        .bot()
        .send_message(
            chat_id,
            &format!("Nice to meet you, {text}! How old are you?"),
        )
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

async fn receive_age(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
    user_data: UserDataStore,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let text = extract_text(&update).unwrap_or_default();

    user_data.write().await.entry(chat_id).or_default().age = Some(text.to_string());
    conv_store
        .write()
        .await
        .insert(chat_id, ConvState::AskLocation);

    context
        .bot()
        .send_message(chat_id, "Great! Where are you from? (City or country)")
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

async fn receive_location(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
    user_data: UserDataStore,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let text = extract_text(&update).unwrap_or_default();

    user_data.write().await.entry(chat_id).or_default().location = Some(text.to_string());
    conv_store.write().await.insert(chat_id, ConvState::AskBio);

    context
        .bot()
        .send_message(chat_id, "Wonderful! Tell me a little about yourself.")
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

async fn receive_bio(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
    user_data: UserDataStore,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let text = extract_text(&update).unwrap_or_default();

    user_data.write().await.entry(chat_id).or_default().bio = Some(text.to_string());
    conv_store.write().await.remove(&chat_id);

    let profile = user_data
        .read()
        .await
        .get(&chat_id)
        .cloned()
        .unwrap_or_default();
    let summary = format!(
        "Thank you! Here is your profile:\n\n\
         Name: {}\n\
         Age: {}\n\
         Location: {}\n\
         Bio: {}\n\n\
         Send /start to fill it out again!",
        profile.name.as_deref().unwrap_or("N/A"),
        profile.age.as_deref().unwrap_or("N/A"),
        profile.location.as_deref().unwrap_or("N/A"),
        profile.bio.as_deref().unwrap_or("N/A"),
    );

    context
        .bot()
        .send_message(chat_id, &summary)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

async fn cancel(update: Arc<Update>, context: Context, conv_store: ConvStore) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    conv_store.write().await.remove(&chat_id);

    context
        .bot()
        .send_message(
            chat_id,
            "Conversation cancelled. Send /start to begin again.",
        )
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_chat_id(update: &Update) -> i64 {
    update.effective_chat().expect("update must have a chat").id
}

fn extract_text<'a>(update: &'a Update) -> Option<&'a str> {
    update.effective_message().and_then(|m| m.text.as_deref())
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

fn is_text_in_state(update: &Update, conv_store: &ConvStore, state: ConvState) -> bool {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return false,
    };
    let has_text = msg.text.is_some();
    if !has_text {
        return false;
    }
    let is_cmd = msg
        .entities
        .as_ref()
        .and_then(|ents| ents.first())
        .map(|e| e.entity_type == MessageEntityType::BotCommand && e.offset == 0)
        .unwrap_or(false);
    if is_cmd {
        return false;
    }
    let chat_id = msg.chat.id;
    conv_store
        .try_read()
        .map(|guard| guard.get(&chat_id) == Some(&state))
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

    let app: Arc<Application> = ApplicationBuilder::new().token(token).build();

    let conv_store: ConvStore = Arc::new(RwLock::new(HashMap::new()));
    let user_data: UserDataStore = Arc::new(RwLock::new(HashMap::new()));

    // Entry point: /start
    {
        let cs = Arc::clone(&conv_store);
        app.add_typed_handler(
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

    // Fallback: /cancel
    {
        let cs = Arc::clone(&conv_store);
        app.add_typed_handler(
            FnHandler::new(
                |u| check_command(u, "cancel"),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { cancel(update, ctx, cs).await }
                },
            ),
            0,
        )
        .await;
    }

    // State: AskName
    {
        let cs = Arc::clone(&conv_store);
        let ud = Arc::clone(&user_data);
        let cs_check = Arc::clone(&conv_store);
        app.add_typed_handler(
            FnHandler::new(
                move |u| is_text_in_state(u, &cs_check, ConvState::AskName),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    let ud = Arc::clone(&ud);
                    async move { receive_name(update, ctx, cs, ud).await }
                },
            ),
            1,
        )
        .await;
    }

    // State: AskAge
    {
        let cs = Arc::clone(&conv_store);
        let ud = Arc::clone(&user_data);
        let cs_check = Arc::clone(&conv_store);
        app.add_typed_handler(
            FnHandler::new(
                move |u| is_text_in_state(u, &cs_check, ConvState::AskAge),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    let ud = Arc::clone(&ud);
                    async move { receive_age(update, ctx, cs, ud).await }
                },
            ),
            1,
        )
        .await;
    }

    // State: AskLocation
    {
        let cs = Arc::clone(&conv_store);
        let ud = Arc::clone(&user_data);
        let cs_check = Arc::clone(&conv_store);
        app.add_typed_handler(
            FnHandler::new(
                move |u| is_text_in_state(u, &cs_check, ConvState::AskLocation),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    let ud = Arc::clone(&ud);
                    async move { receive_location(update, ctx, cs, ud).await }
                },
            ),
            1,
        )
        .await;
    }

    // State: AskBio
    {
        let cs = Arc::clone(&conv_store);
        let ud = Arc::clone(&user_data);
        let cs_check = Arc::clone(&conv_store);
        app.add_typed_handler(
            FnHandler::new(
                move |u| is_text_in_state(u, &cs_check, ConvState::AskBio),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    let ud = Arc::clone(&ud);
                    async move { receive_bio(update, ctx, cs, ud).await }
                },
            ),
            1,
        )
        .await;
    }

    println!("Conversation bot is running. Press Ctrl+C to stop.");
    println!("Commands: /start, /cancel");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
