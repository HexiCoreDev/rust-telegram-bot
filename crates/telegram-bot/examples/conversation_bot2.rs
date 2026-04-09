//! Conversation Bot 2 -- conversation with reply keyboard buttons and user data.
//!
//! This is the Rust port of Python's `conversationbot2.py`.
//!
//! Demonstrates:
//! - Reply keyboard markup for category selection
//! - Multi-step conversation with states: CHOOSING, TYPING_REPLY, TYPING_CHOICE
//! - Storing user-provided facts in a per-user HashMap
//! - "Done" fallback to summarize and exit
//! - Manual conversation state tracking with `RwLock<HashMap>`
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p rust-tg-bot --example conversation_bot2
//! ```
//!
//! Then in Telegram:
//! - `/start` -- begins the conversation
//! - Reply "Age", "Favourite colour", "Number of siblings", "Something else...", or "Done"

use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, Context, FnHandler, HandlerError, HandlerResult, HashMap,
    KeyboardButton, MessageEntityType, ReplyKeyboardMarkup, ReplyKeyboardRemove, RwLock, Update,
};

// ---------------------------------------------------------------------------
// Conversation state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConvState {
    Choosing,
    TypingReply,
    TypingChoice,
}

type ConvStore = Arc<RwLock<HashMap<i64, ConvState>>>;

/// Per-user fact storage.
type UserFacts = Arc<RwLock<HashMap<i64, HashMap<String, String>>>>;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_chat_id(update: &Update) -> i64 {
    update.effective_chat().map(|c| c.id).unwrap_or(0)
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
    if msg.text.is_none() {
        return false;
    }
    // Exclude commands.
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

fn is_text_matching_in_state(
    update: &Update,
    conv_store: &ConvStore,
    state: ConvState,
    pattern: &str,
) -> bool {
    if !is_text_in_state(update, conv_store, state) {
        return false;
    }
    let text = extract_text(update).unwrap_or("");
    // Check if text matches the provided pattern (exact match for categories).
    text == pattern
}

fn facts_to_str(facts: &HashMap<String, String>) -> String {
    if facts.is_empty() {
        return String::from("\n(nothing yet)\n");
    }
    let items: Vec<String> = facts
        .iter()
        .filter(|(k, _)| k.as_str() != "choice")
        .map(|(k, v)| format!("{k} - {v}"))
        .collect();
    format!("\n{}\n", items.join("\n"))
}

/// Build the reply keyboard markup.
fn reply_keyboard() -> serde_json::Value {
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

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// `/start` -- begin the conversation.
async fn start_command(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    conv_store
        .write()
        .await
        .insert(chat_id, ConvState::Choosing);

    context
        .bot()
        .send_message(
            chat_id,
            "Hi! My name is Doctor Botter. I will hold a more complex conversation with you. \
             Why don't you tell me something about yourself?",
        )
        .reply_markup(reply_keyboard())
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// User chose a predefined category (Age, Favourite colour, Number of siblings).
async fn regular_choice(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
    user_facts: UserFacts,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let text = extract_text(&update).unwrap_or_default();

    // Remember which category was chosen.
    user_facts
        .write()
        .await
        .entry(chat_id)
        .or_default()
        .insert("choice".to_string(), text.to_string());

    conv_store
        .write()
        .await
        .insert(chat_id, ConvState::TypingReply);

    context
        .bot()
        .send_message(
            chat_id,
            &format!(
                "Your {}? Yes, I would love to hear about that!",
                text.to_lowercase()
            ),
        )
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// User chose "Something else..." -- ask for a custom category name.
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

/// Received the user's reply (the actual fact value).
async fn received_information(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
    user_facts: UserFacts,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let text = extract_text(&update).unwrap_or_default().to_string();

    let facts_summary = {
        let mut store = user_facts.write().await;
        let facts = store.entry(chat_id).or_default();
        let category = facts
            .remove("choice")
            .unwrap_or_else(|| "Unknown".to_string());
        facts.insert(category, text);
        facts_to_str(facts)
    };

    conv_store
        .write()
        .await
        .insert(chat_id, ConvState::Choosing);

    context
        .bot()
        .send_message(
            chat_id,
            &format!(
                "Neat! Just so you know, this is what you already told me:\
                 {facts_summary}\
                 You can tell me more, or change your opinion on something."
            ),
        )
        .reply_markup(reply_keyboard())
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// "Done" -- display gathered facts and end conversation.
async fn done(
    update: Arc<Update>,
    context: Context,
    conv_store: ConvStore,
    user_facts: UserFacts,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    let facts_summary = {
        let mut store = user_facts.write().await;
        let facts = store.entry(chat_id).or_default();
        facts.remove("choice");
        let summary = facts_to_str(facts);
        facts.clear();
        summary
    };

    conv_store.write().await.remove(&chat_id);

    // Remove the reply keyboard.
    let remove_keyboard =
        serde_json::to_value(ReplyKeyboardRemove::new()).expect("keyboard remove serialization");

    context
        .bot()
        .send_message(
            chat_id,
            &format!("I learned these facts about you:{facts_summary}Until next time!"),
        )
        .reply_markup(remove_keyboard)
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

    let app = ApplicationBuilder::new().token(token).build();

    let conv_store: ConvStore = Arc::new(RwLock::new(HashMap::new()));
    let user_facts: UserFacts = Arc::new(RwLock::new(HashMap::new()));

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

    // CHOOSING state: predefined categories
    for category in &["Age", "Favourite colour", "Number of siblings"] {
        let cs = Arc::clone(&conv_store);
        let uf = Arc::clone(&user_facts);
        let cs_check = Arc::clone(&conv_store);
        let cat = category.to_string();
        app.add_handler(
            FnHandler::new(
                move |u| is_text_matching_in_state(u, &cs_check, ConvState::Choosing, &cat),
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    let uf = Arc::clone(&uf);
                    async move { regular_choice(update, ctx, cs, uf).await }
                },
            ),
            1,
        )
        .await;
    }

    // CHOOSING state: "Something else..."
    {
        let cs = Arc::clone(&conv_store);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| {
                    is_text_matching_in_state(
                        u,
                        &cs_check,
                        ConvState::Choosing,
                        "Something else...",
                    )
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

    // TYPING_CHOICE state: user types a custom category name, then transitions to regular_choice behavior.
    // In the Python example, TYPING_CHOICE uses the same regular_choice handler.
    {
        let cs = Arc::clone(&conv_store);
        let uf = Arc::clone(&user_facts);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| {
                    is_text_in_state(u, &cs_check, ConvState::TypingChoice)
                        && extract_text(u).map_or(true, |t| t != "Done")
                },
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    let uf = Arc::clone(&uf);
                    async move { regular_choice(update, ctx, cs, uf).await }
                },
            ),
            1,
        )
        .await;
    }

    // TYPING_REPLY state: user provides the actual value.
    {
        let cs = Arc::clone(&conv_store);
        let uf = Arc::clone(&user_facts);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| {
                    is_text_in_state(u, &cs_check, ConvState::TypingReply)
                        && extract_text(u).map_or(true, |t| t != "Done")
                },
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    let uf = Arc::clone(&uf);
                    async move { received_information(update, ctx, cs, uf).await }
                },
            ),
            1,
        )
        .await;
    }

    // Fallback: "Done" text in any conversation state.
    {
        let cs = Arc::clone(&conv_store);
        let uf = Arc::clone(&user_facts);
        let cs_check = Arc::clone(&conv_store);
        app.add_handler(
            FnHandler::new(
                move |u| {
                    let text = extract_text(u).unwrap_or("");
                    text == "Done"
                        && (is_text_in_state(u, &cs_check, ConvState::Choosing)
                            || is_text_in_state(u, &cs_check, ConvState::TypingReply)
                            || is_text_in_state(u, &cs_check, ConvState::TypingChoice))
                },
                move |update, ctx| {
                    let cs = Arc::clone(&cs);
                    let uf = Arc::clone(&uf);
                    async move { done(update, ctx, cs, uf).await }
                },
            ),
            1,
        )
        .await;
    }

    println!("Conversation bot 2 is running. Press Ctrl+C to stop.");
    println!("Commands: /start");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
