//! Persistent Conversation Bot -- demonstrates conversations that survive bot restarts.
//!
//! This is the Rust port of Python's `persistentconversationbot.py`.
//!
//! Demonstrates:
//! - `JsonFilePersistence` for persisting user_data across restarts
//! - Conversation state tracking via `context.user_data()` / `context.set_user_data()`
//! - Reply keyboard markup for category selection
//! - Multi-step data collection (category -> value) loop
//! - `/show_data` command outside the conversation
//!
//! The bot acts as "Doctor Botter" and collects facts about the user
//! (age, favourite colour, number of siblings, or custom categories),
//! persisting all data to a JSON file.
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p rust-tg-bot --example persistent_conversation_bot
//! ```
//!
//! Then in Telegram:
//! - `/start` -- begins the conversation
//! - Type a category (Age, Favourite colour, etc.) or "Something else..."
//! - Type a value for the chosen category
//! - Type "Done" to finish
//! - `/show_data` -- displays all stored facts

use rust_tg_bot::ext::persistence::json_file::JsonFilePersistence;
use rust_tg_bot::ext::prelude::{
    Application, ApplicationBuilder, Arc, Context, FnHandler, HandlerError, HandlerResult, HashMap,
    JsonValue, KeyboardButton, MessageEntityType, ReplyKeyboardMarkup, ReplyKeyboardRemove, RwLock,
    Update,
};

// ---------------------------------------------------------------------------
// Conversation states
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConvState {
    Choosing,
    TypingReply,
    TypingChoice,
}

type ConvStore = Arc<RwLock<HashMap<i64, ConvState>>>;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const CHOICE_KEY: &str = "_choice";

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

fn build_reply_keyboard() -> JsonValue {
    serde_json::to_value(
        ReplyKeyboardMarkup::new(vec![
            vec![
                KeyboardButton::text("Age"),
                KeyboardButton::text("Favourite colour"),
            ],
            vec![
                KeyboardButton::text("Number of siblings"),
                KeyboardButton::text("Something else..."),
            ],
            vec![KeyboardButton::text("Done")],
        ])
        .one_time()
        .resize(),
    )
    .expect("keyboard serialization")
}

fn facts_to_str(user_data: &HashMap<String, JsonValue>) -> String {
    let mut facts = Vec::new();
    for (key, value) in user_data {
        if key.starts_with('_') {
            continue;
        }
        let owned = value.to_string();
        let val_str = value.as_str().unwrap_or(&owned);
        facts.push(format!("{key} - {val_str}"));
    }
    if facts.is_empty() {
        return "\n(nothing yet)\n".to_string();
    }
    format!("\n{}\n", facts.join("\n"))
}

fn is_text_not_command(update: &Update) -> bool {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return false,
    };
    if msg.text.is_none() {
        return false;
    }
    let is_cmd = msg
        .entities
        .as_ref()
        .and_then(|ents| ents.first())
        .map(|e| e.entity_type == MessageEntityType::BotCommand && e.offset == 0)
        .unwrap_or(false);
    !is_cmd
}

fn is_in_state(update: &Update, conv_store: &ConvStore, state: ConvState) -> bool {
    if !is_text_not_command(update) {
        return false;
    }
    let chat_id = match update.effective_chat() {
        Some(c) => c.id,
        None => return false,
    };
    conv_store
        .try_read()
        .map(|guard| guard.get(&chat_id) == Some(&state))
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// `/start` -- begin the conversation.
async fn start(update: Arc<Update>, context: Context, conv_store: ConvStore) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    conv_store
        .write()
        .await
        .insert(chat_id, ConvState::Choosing);

    let user_data = context.user_data().await.unwrap_or_default();
    let reply_text = if user_data.is_empty() || user_data.keys().all(|k| k.starts_with('_')) {
        "Hi! My name is Doctor Botter. I will hold a more complex conversation with you. \
         Why don't you tell me something about yourself?"
            .to_string()
    } else {
        let known_keys: Vec<&str> = user_data
            .keys()
            .filter(|k| !k.starts_with('_'))
            .map(String::as_str)
            .collect();
        format!(
            "Hi! My name is Doctor Botter. You already told me your {}. \
             Why don't you tell me something more about yourself? Or change anything I already know.",
            known_keys.join(", ")
        )
    };

    context
        .bot()
        .send_message(chat_id, &reply_text)
        .reply_markup(build_reply_keyboard())
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// Handle a predefined category selection (Age, Favourite colour, Number of siblings).
async fn regular_choice(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let text = extract_text(&update).unwrap_or("").to_lowercase();

    // Store which category was chosen.
    context
        .set_user_data(CHOICE_KEY.to_string(), JsonValue::String(text.clone()))
        .await;

    conv_store
        .write()
        .await
        .insert(chat_id, ConvState::TypingReply);

    let user_data = context.user_data().await.unwrap_or_default();
    let reply_text = if user_data.contains_key(&text) {
        format!(
            "Your {text}? I already know the following about that: {}",
            user_data.get(&text).and_then(|v| v.as_str()).unwrap_or("?")
        )
    } else {
        format!("Your {text}? Yes, I would love to hear about that!")
    };

    context
        .bot()
        .send_message(chat_id, &reply_text)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// Handle "Something else..." -- ask for a custom category name.
async fn custom_choice(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    conv_store
        .write()
        .await
        .insert(chat_id, ConvState::TypingChoice);

    context
        .bot()
        .send_message(
            chat_id,
            "Alright, please send me the category first, for example \"Most impressive skill\"",
        )
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// Store a received category value.
async fn received_information(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let text = extract_text(&update).unwrap_or("").to_lowercase();

    let user_data = context.user_data().await.unwrap_or_default();
    let category = user_data
        .get(CHOICE_KEY)
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    context
        .set_user_data(category, JsonValue::String(text))
        .await;

    conv_store
        .write()
        .await
        .insert(chat_id, ConvState::Choosing);

    // Re-read user data to display the updated facts.
    let user_data = context.user_data().await.unwrap_or_default();
    let reply = format!(
        "Neat! Just so you know, this is what you already told me:{}\
         You can tell me more, or change your opinion on something.",
        facts_to_str(&user_data),
    );

    context
        .bot()
        .send_message(chat_id, &reply)
        .reply_markup(build_reply_keyboard())
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// "Done" -- finish the conversation and display gathered info.
async fn done(update: Arc<Update>, context: Context, conv_store: ConvStore) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    conv_store.write().await.remove(&chat_id);

    let user_data = context.user_data().await.unwrap_or_default();
    let reply = format!(
        "I learned these facts about you:{}Until next time!",
        facts_to_str(&user_data),
    );

    // Send with ReplyKeyboardRemove to hide the custom keyboard.
    let remove_keyboard =
        serde_json::to_value(ReplyKeyboardRemove::new()).expect("keyboard remove serialization");

    context
        .bot()
        .send_message(chat_id, &reply)
        .reply_markup(remove_keyboard)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// `/show_data` -- display stored facts (available outside the conversation).
async fn show_data(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    let user_data = context.user_data().await.unwrap_or_default();
    let reply = format!(
        "This is what you already told me:{}",
        facts_to_str(&user_data),
    );

    context
        .bot()
        .send_message(chat_id, &reply)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

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

    // Set up JSON file persistence (the Rust equivalent of PicklePersistence).
    let persistence = JsonFilePersistence::new("persistent_conversation_bot", true, false);

    let app: Arc<Application> = ApplicationBuilder::new()
        .token(token)
        .persistence(Box::new(persistence))
        .build();

    let conv_store: ConvStore = Arc::new(RwLock::new(HashMap::new()));

    // /start
    {
        let cs = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                |u| check_command(u, "start"),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { start(update, ctx, cs).await }
                },
            ),
            0,
        )
        .await;
    }

    // /show_data (works outside conversation too)
    app.add_handler(
        FnHandler::new(|u| check_command(u, "show_data"), show_data),
        0,
    )
    .await;

    // Choosing state: predefined categories
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| {
                    if !is_in_state(u, &cs_check, ConvState::Choosing) {
                        return false;
                    }
                    let text = u
                        .effective_message()
                        .and_then(|m| m.text.as_deref())
                        .unwrap_or("");
                    matches!(text, "Age" | "Favourite colour" | "Number of siblings")
                },
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { regular_choice(update, ctx, cs).await }
                },
            ),
            1,
        )
        .await;
    }

    // Choosing state: "Something else..."
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| {
                    if !is_in_state(u, &cs_check, ConvState::Choosing) {
                        return false;
                    }
                    let text = u
                        .effective_message()
                        .and_then(|m| m.text.as_deref())
                        .unwrap_or("");
                    text == "Something else..."
                },
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { custom_choice(update, ctx, cs).await }
                },
            ),
            1,
        )
        .await;
    }

    // Choosing state: "Done"
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| {
                    if !is_in_state(u, &cs_check, ConvState::Choosing) {
                        return false;
                    }
                    let text = u
                        .effective_message()
                        .and_then(|m| m.text.as_deref())
                        .unwrap_or("");
                    text == "Done"
                },
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { done(update, ctx, cs).await }
                },
            ),
            1,
        )
        .await;
    }

    // TypingChoice state: user sends a custom category name (treat as regular_choice)
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| {
                    if !is_in_state(u, &cs_check, ConvState::TypingChoice) {
                        return false;
                    }
                    let text = u
                        .effective_message()
                        .and_then(|m| m.text.as_deref())
                        .unwrap_or("");
                    text != "Done"
                },
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { regular_choice(update, ctx, cs).await }
                },
            ),
            1,
        )
        .await;
    }

    // TypingReply state: user provides a value for the category
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| {
                    if !is_in_state(u, &cs_check, ConvState::TypingReply) {
                        return false;
                    }
                    let text = u
                        .effective_message()
                        .and_then(|m| m.text.as_deref())
                        .unwrap_or("");
                    text != "Done"
                },
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    async move { received_information(update, ctx, cs).await }
                },
            ),
            1,
        )
        .await;
    }

    println!("Persistent conversation bot is running. Press Ctrl+C to stop.");
    println!("Data is saved to persistent_conversation_bot.json");
    println!("Commands: /start, /show_data");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
