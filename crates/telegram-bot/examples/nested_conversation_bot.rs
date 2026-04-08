//! Nested Conversation Bot -- demonstrates multi-level conversation state management.
//!
//! This is the Rust port of Python's `nestedconversationbot.py`.
//!
//! Demonstrates:
//! - Nested conversation levels (top -> member selection -> feature input)
//! - State machine transitions across three levels
//! - Storing per-user family data with `Arc<RwLock<HashMap>>`
//! - Inline keyboard navigation between conversation states
//! - `/stop` to end the conversation from any level
//!
//! The bot collects information about the user's family members (parents,
//! children, self) organized in a three-level conversation tree:
//!
//! Level 1 (Top): Add member / Add self / Show data / Done
//! Level 2 (Member): Add parent / Add child / Show data / Back
//! Level 3 (Features): Name / Age / Done
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p telegram-bot --example nested_conversation_bot
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use telegram_bot::ext::prelude::*;
use telegram_bot::types::inline::inline_keyboard_button::InlineKeyboardButton;
use telegram_bot::types::inline::inline_keyboard_markup::InlineKeyboardMarkup;

// ---------------------------------------------------------------------------
// State definitions
// ---------------------------------------------------------------------------

/// Top-level conversation states.
///
/// Some variants represent conceptual states from the Python original that
/// are encoded in `ConvState` but not directly pattern-matched on; they are
/// retained for completeness and documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum TopState {
    SelectingAction,
    AddingMember,
    DescribingSelf,
    ShowingData,
    Stopped,
}

/// Second-level states (member selection).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemberState {
    SelectingLevel,
    SelectingGender,
}

/// Third-level states (feature input).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FeatureState {
    SelectingFeature,
    Typing,
}

/// Combined conversation state for the user.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ConvState {
    Top(TopState),
    Member(MemberState),
    Feature(FeatureState),
    End,
}

// ---------------------------------------------------------------------------
// Data constants
// ---------------------------------------------------------------------------

const PARENTS: &str = "parents";
const CHILDREN: &str = "children";
const SELF_LEVEL: &str = "self";

// Callback data identifiers
const CB_ADD_MEMBER: &str = "add_member";
const CB_ADD_SELF: &str = "add_self";
const CB_SHOW: &str = "show";
const CB_DONE: &str = "done";
const CB_PARENTS: &str = "parents";
const CB_CHILDREN: &str = "children";
const CB_BACK: &str = "back";
const CB_MALE: &str = "male";
const CB_FEMALE: &str = "female";
const CB_NAME: &str = "name";
const CB_AGE: &str = "age";

// ---------------------------------------------------------------------------
// Shared state types
// ---------------------------------------------------------------------------

/// Per-user family member record.
#[derive(Debug, Clone, Default)]
struct PersonInfo {
    gender: Option<String>,
    name: Option<String>,
    age: Option<String>,
}

/// All user data for the conversation.
#[derive(Debug, Clone, Default)]
struct UserState {
    conv: ConvState,
    current_level: String,
    current_feature: String,
    current_person: PersonInfo,
    family: HashMap<String, Vec<PersonInfo>>,
}

impl Default for ConvState {
    fn default() -> Self {
        ConvState::End
    }
}

type StateStore = Arc<RwLock<HashMap<i64, UserState>>>;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_chat_id(update: &Update) -> i64 {
    update.effective_chat().expect("update must have a chat").id
}

fn extract_user_id(update: &Update) -> i64 {
    update.effective_user().expect("update must have a user").id
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

fn name_switcher(level: &str) -> (&str, &str) {
    if level == PARENTS {
        ("Father", "Mother")
    } else {
        ("Brother", "Sister")
    }
}

fn pretty_print(family: &HashMap<String, Vec<PersonInfo>>, level: &str) -> String {
    let people = match family.get(level) {
        Some(p) if !p.is_empty() => p,
        _ => return "\nNo information yet.".to_string(),
    };

    let mut result = String::new();
    if level == SELF_LEVEL {
        for person in people {
            result.push_str(&format!(
                "\nName: {}, Age: {}",
                person.name.as_deref().unwrap_or("-"),
                person.age.as_deref().unwrap_or("-"),
            ));
        }
    } else {
        let (male, female) = name_switcher(level);
        for person in people {
            let gender_label = if person.gender.as_deref() == Some(CB_FEMALE) {
                female
            } else {
                male
            };
            result.push_str(&format!(
                "\n{gender_label}: Name: {}, Age: {}",
                person.name.as_deref().unwrap_or("-"),
                person.age.as_deref().unwrap_or("-"),
            ));
        }
    }
    result
}

fn keyboard_json(markup: &InlineKeyboardMarkup) -> serde_json::Value {
    serde_json::to_value(markup).expect("keyboard serialization")
}

// ---------------------------------------------------------------------------
// Keyboard builders
// ---------------------------------------------------------------------------

fn top_menu_keyboard() -> serde_json::Value {
    keyboard_json(&InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Add family member", CB_ADD_MEMBER),
            InlineKeyboardButton::callback("Add yourself", CB_ADD_SELF),
        ],
        vec![
            InlineKeyboardButton::callback("Show data", CB_SHOW),
            InlineKeyboardButton::callback("Done", CB_DONE),
        ],
    ]))
}

fn member_level_keyboard() -> serde_json::Value {
    keyboard_json(&InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Add parent", CB_PARENTS),
            InlineKeyboardButton::callback("Add child", CB_CHILDREN),
        ],
        vec![
            InlineKeyboardButton::callback("Show data", CB_SHOW),
            InlineKeyboardButton::callback("Back", CB_BACK),
        ],
    ]))
}

fn gender_keyboard(level: &str) -> serde_json::Value {
    let (male, female) = name_switcher(level);
    keyboard_json(&InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback(format!("Add {male}"), CB_MALE),
            InlineKeyboardButton::callback(format!("Add {female}"), CB_FEMALE),
        ],
        vec![
            InlineKeyboardButton::callback("Show data", CB_SHOW),
            InlineKeyboardButton::callback("Back", CB_BACK),
        ],
    ]))
}

fn feature_keyboard() -> serde_json::Value {
    keyboard_json(&InlineKeyboardMarkup::from_row(vec![
        InlineKeyboardButton::callback("Name", CB_NAME),
        InlineKeyboardButton::callback("Age", CB_AGE),
        InlineKeyboardButton::callback("Done", CB_DONE),
    ]))
}

fn back_keyboard() -> serde_json::Value {
    keyboard_json(&InlineKeyboardMarkup::from_button(
        InlineKeyboardButton::callback("Back", CB_BACK),
    ))
}

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// `/start` -- begin the conversation.
async fn start_command(update: Arc<Update>, context: Context, store: StateStore) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let user_id = extract_user_id(&update);

    {
        let mut s = store.write().await;
        let user_state = s.entry(user_id).or_default();
        user_state.conv = ConvState::Top(TopState::SelectingAction);
    }

    context
        .bot()
        .send_message(
            chat_id,
            "Hi, I'm Family Bot and I'm here to help you gather information about your family.",
        )
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    context
        .bot()
        .send_message(
            chat_id,
            "You may choose to add a family member, yourself, show the gathered data, or end the \
             conversation. To abort, simply type /stop.",
        )
        .reply_markup(top_menu_keyboard())
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// Handle top-level actions from inline keyboard.
async fn handle_top_action(
    update: Arc<Update>,
    context: Context,
    store: StateStore,
) -> HandlerResult {
    let user_id = extract_user_id(&update);
    let cq = update
        .callback_query
        .as_ref()
        .expect("must have callback_query");
    let data = cq.data.as_deref().unwrap_or("");

    // Answer callback query
    context
        .bot()
        .answer_callback_query(&cq.id)
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    let msg = cq.message.as_ref().expect("must have message");
    let chat_id = msg.chat().id;

    match data {
        CB_ADD_MEMBER => {
            let mut s = store.write().await;
            let us = s.entry(user_id).or_default();
            us.conv = ConvState::Member(MemberState::SelectingLevel);

            context.bot()
                .edit_message_text("You may add a parent or a child. Also you can show the gathered data or go back.")
                .chat_id(chat_id).message_id(msg.message_id())
                .reply_markup(member_level_keyboard())
                .await.map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        CB_ADD_SELF => {
            let mut s = store.write().await;
            let us = s.entry(user_id).or_default();
            us.current_level = SELF_LEVEL.to_string();
            us.current_person = PersonInfo::default();
            us.conv = ConvState::Feature(FeatureState::SelectingFeature);

            context
                .bot()
                .edit_message_text("Please select a feature to update.")
                .chat_id(chat_id)
                .message_id(msg.message_id())
                .reply_markup(feature_keyboard())
                .await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        CB_SHOW => {
            let s = store.read().await;
            let us = s.get(&user_id);
            let text = if let Some(us) = us {
                format!(
                    "Yourself:{}\n\nParents:{}\n\nChildren:{}",
                    pretty_print(&us.family, SELF_LEVEL),
                    pretty_print(&us.family, PARENTS),
                    pretty_print(&us.family, CHILDREN),
                )
            } else {
                "No data collected yet.".to_string()
            };
            drop(s);

            {
                let mut s = store.write().await;
                let us = s.entry(user_id).or_default();
                us.conv = ConvState::Top(TopState::ShowingData);
            }

            context
                .bot()
                .edit_message_text(&text)
                .chat_id(chat_id)
                .message_id(msg.message_id())
                .reply_markup(back_keyboard())
                .await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        CB_DONE => {
            let mut s = store.write().await;
            let us = s.entry(user_id).or_default();
            us.conv = ConvState::End;

            context
                .bot()
                .edit_message_text("See you around!")
                .chat_id(chat_id)
                .message_id(msg.message_id())
                .await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        CB_BACK => {
            // "Back" from showing data -- return to top menu.
            let mut s = store.write().await;
            let us = s.entry(user_id).or_default();
            us.conv = ConvState::Top(TopState::SelectingAction);
            drop(s);

            context.bot()
                .edit_message_text(
                    "You may choose to add a family member, yourself, show the gathered data, or end the conversation.",
                )
                .chat_id(chat_id).message_id(msg.message_id())
                .reply_markup(top_menu_keyboard())
                .await.map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        _ => {}
    }

    Ok(())
}

/// Handle second-level member selection.
async fn handle_member_action(
    update: Arc<Update>,
    context: Context,
    store: StateStore,
) -> HandlerResult {
    let user_id = extract_user_id(&update);
    let cq = update
        .callback_query
        .as_ref()
        .expect("must have callback_query");
    let data = cq.data.as_deref().unwrap_or("");

    context
        .bot()
        .answer_callback_query(&cq.id)
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    let msg = cq.message.as_ref().expect("must have message");
    let chat_id = msg.chat().id;

    match data {
        CB_PARENTS | CB_CHILDREN => {
            let mut s = store.write().await;
            let us = s.entry(user_id).or_default();
            us.current_level = data.to_string();
            us.conv = ConvState::Member(MemberState::SelectingGender);

            let kb = gender_keyboard(data);
            context
                .bot()
                .edit_message_text("Please choose whom to add.")
                .chat_id(chat_id)
                .message_id(msg.message_id())
                .reply_markup(kb)
                .await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        CB_SHOW => {
            let s = store.read().await;
            let us = s.get(&user_id);
            let text = if let Some(us) = us {
                format!(
                    "Yourself:{}\n\nParents:{}\n\nChildren:{}",
                    pretty_print(&us.family, SELF_LEVEL),
                    pretty_print(&us.family, PARENTS),
                    pretty_print(&us.family, CHILDREN),
                )
            } else {
                "No data collected yet.".to_string()
            };
            drop(s);

            {
                let mut s = store.write().await;
                let us = s.entry(user_id).or_default();
                us.conv = ConvState::Top(TopState::ShowingData);
            }

            context
                .bot()
                .edit_message_text(&text)
                .chat_id(chat_id)
                .message_id(msg.message_id())
                .reply_markup(back_keyboard())
                .await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        CB_BACK => {
            // Return to top level.
            let mut s = store.write().await;
            let us = s.entry(user_id).or_default();
            us.conv = ConvState::Top(TopState::SelectingAction);
            drop(s);

            context.bot()
                .edit_message_text(
                    "You may choose to add a family member, yourself, show the gathered data, or end the conversation.",
                )
                .chat_id(chat_id).message_id(msg.message_id())
                .reply_markup(top_menu_keyboard())
                .await.map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        _ => {}
    }

    Ok(())
}

/// Handle gender selection and feature-level callbacks.
async fn handle_feature_action(
    update: Arc<Update>,
    context: Context,
    store: StateStore,
) -> HandlerResult {
    let user_id = extract_user_id(&update);
    let cq = update
        .callback_query
        .as_ref()
        .expect("must have callback_query");
    let data = cq.data.as_deref().unwrap_or("");

    context
        .bot()
        .answer_callback_query(&cq.id)
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    let msg = cq.message.as_ref().expect("must have message");
    let chat_id = msg.chat().id;

    match data {
        CB_MALE | CB_FEMALE => {
            // Gender selected -- start collecting features.
            let mut s = store.write().await;
            let us = s.entry(user_id).or_default();
            us.current_person = PersonInfo {
                gender: Some(data.to_string()),
                name: None,
                age: None,
            };
            us.conv = ConvState::Feature(FeatureState::SelectingFeature);

            context
                .bot()
                .edit_message_text("Please select a feature to update.")
                .chat_id(chat_id)
                .message_id(msg.message_id())
                .reply_markup(feature_keyboard())
                .await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        CB_NAME | CB_AGE => {
            // Feature selected -- prompt for input.
            let mut s = store.write().await;
            let us = s.entry(user_id).or_default();
            us.current_feature = data.to_string();
            us.conv = ConvState::Feature(FeatureState::Typing);

            context
                .bot()
                .edit_message_text("Okay, tell me.")
                .chat_id(chat_id)
                .message_id(msg.message_id())
                .await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        CB_DONE => {
            // Save the current person and return to the appropriate level.
            let mut s = store.write().await;
            let us = s.entry(user_id).or_default();
            let level = us.current_level.clone();
            let person = us.current_person.clone();
            us.family.entry(level.clone()).or_default().push(person);
            us.current_person = PersonInfo::default();

            if level == SELF_LEVEL {
                us.conv = ConvState::Top(TopState::SelectingAction);
                drop(s);

                context.bot()
                    .edit_message_text(
                        "You may choose to add a family member, yourself, show the gathered data, or end the conversation.",
                    )
                    .chat_id(chat_id).message_id(msg.message_id())
                    .reply_markup(top_menu_keyboard())
                    .await.map_err(|e| HandlerError::Other(Box::new(e)))?;
            } else {
                us.conv = ConvState::Member(MemberState::SelectingLevel);
                drop(s);

                context.bot()
                    .edit_message_text("You may add a parent or a child. Also you can show the gathered data or go back.")
                    .chat_id(chat_id).message_id(msg.message_id())
                    .reply_markup(member_level_keyboard())
                    .await.map_err(|e| HandlerError::Other(Box::new(e)))?;
            }
        }
        CB_BACK | CB_SHOW => {
            // Handle back/show from gender selection -- return to member level.
            let mut s = store.write().await;
            let us = s.entry(user_id).or_default();
            us.conv = ConvState::Member(MemberState::SelectingLevel);
            drop(s);

            context.bot()
                .edit_message_text("You may add a parent or a child. Also you can show the gathered data or go back.")
                .chat_id(chat_id).message_id(msg.message_id())
                .reply_markup(member_level_keyboard())
                .await.map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        _ => {}
    }

    Ok(())
}

/// Handle free-text input for features (name, age).
async fn handle_text_input(
    update: Arc<Update>,
    context: Context,
    store: StateStore,
) -> HandlerResult {
    let user_id = extract_user_id(&update);
    let chat_id = extract_chat_id(&update);
    let text = extract_text(&update).unwrap_or("").to_string();

    let mut s = store.write().await;
    let us = s.entry(user_id).or_default();

    match us.current_feature.as_str() {
        CB_NAME => us.current_person.name = Some(text),
        CB_AGE => us.current_person.age = Some(text),
        _ => {}
    }
    us.conv = ConvState::Feature(FeatureState::SelectingFeature);
    drop(s);

    context
        .bot()
        .send_message(chat_id, "Got it! Please select a feature to update.")
        .reply_markup(feature_keyboard())
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// `/stop` -- end the conversation.
async fn stop_command(update: Arc<Update>, context: Context, store: StateStore) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let user_id = extract_user_id(&update);

    {
        let mut s = store.write().await;
        let us = s.entry(user_id).or_default();
        us.conv = ConvState::End;
    }

    context
        .bot()
        .send_message(chat_id, "Okay, bye.")
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Predicates
// ---------------------------------------------------------------------------

fn is_in_state(store: &StateStore, user_id: i64, expected: &ConvState) -> bool {
    store
        .try_read()
        .map(|guard| {
            guard
                .get(&user_id)
                .map(|us| &us.conv == expected)
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

fn is_callback_in_top_state(update: &Update, store: &StateStore) -> bool {
    if update.callback_query.is_none() {
        return false;
    }
    let user_id = match update.effective_user() {
        Some(u) => u.id,
        None => return false,
    };
    is_in_state(store, user_id, &ConvState::Top(TopState::SelectingAction))
        || is_in_state(store, user_id, &ConvState::Top(TopState::ShowingData))
}

fn is_callback_in_member_state(update: &Update, store: &StateStore) -> bool {
    if update.callback_query.is_none() {
        return false;
    }
    let user_id = match update.effective_user() {
        Some(u) => u.id,
        None => return false,
    };
    is_in_state(
        store,
        user_id,
        &ConvState::Member(MemberState::SelectingLevel),
    )
}

fn is_callback_in_gender_or_feature_state(update: &Update, store: &StateStore) -> bool {
    if update.callback_query.is_none() {
        return false;
    }
    let user_id = match update.effective_user() {
        Some(u) => u.id,
        None => return false,
    };
    is_in_state(
        store,
        user_id,
        &ConvState::Member(MemberState::SelectingGender),
    ) || is_in_state(
        store,
        user_id,
        &ConvState::Feature(FeatureState::SelectingFeature),
    )
}

fn is_text_in_typing_state(update: &Update, store: &StateStore) -> bool {
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
    if is_cmd {
        return false;
    }
    let user_id = match update.effective_user() {
        Some(u) => u.id,
        None => return false,
    };
    is_in_state(store, user_id, &ConvState::Feature(FeatureState::Typing))
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();

        let token = std::env::var("TELEGRAM_BOT_TOKEN")
            .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

        let app: Arc<Application> = ApplicationBuilder::new().token(token).build();
        let store: StateStore = Arc::new(RwLock::new(HashMap::new()));

        // /start
        {
            let s = Arc::clone(&store);
            app.add_typed_handler(
                FnHandler::new(
                    |u| check_command(u, "start"),
                    move |update, ctx| {
                        let s = Arc::clone(&s);
                        async move { start_command(update, ctx, s).await }
                    },
                ),
                0,
            )
            .await;
        }

        // /stop
        {
            let s = Arc::clone(&store);
            app.add_typed_handler(
                FnHandler::new(
                    |u| check_command(u, "stop"),
                    move |update, ctx| {
                        let s = Arc::clone(&s);
                        async move { stop_command(update, ctx, s).await }
                    },
                ),
                0,
            )
            .await;
        }

        // Top-level callback handler
        {
            let s = Arc::clone(&store);
            let s_check = Arc::clone(&store);
            app.add_typed_handler(
                FnHandler::new(
                    move |u| is_callback_in_top_state(u, &s_check),
                    move |update, ctx| {
                        let s = Arc::clone(&s);
                        async move { handle_top_action(update, ctx, s).await }
                    },
                ),
                1,
            )
            .await;
        }

        // Member-level callback handler
        {
            let s = Arc::clone(&store);
            let s_check = Arc::clone(&store);
            app.add_typed_handler(
                FnHandler::new(
                    move |u| is_callback_in_member_state(u, &s_check),
                    move |update, ctx| {
                        let s = Arc::clone(&s);
                        async move { handle_member_action(update, ctx, s).await }
                    },
                ),
                1,
            )
            .await;
        }

        // Gender/feature-level callback handler
        {
            let s = Arc::clone(&store);
            let s_check = Arc::clone(&store);
            app.add_typed_handler(
                FnHandler::new(
                    move |u| is_callback_in_gender_or_feature_state(u, &s_check),
                    move |update, ctx| {
                        let s = Arc::clone(&s);
                        async move { handle_feature_action(update, ctx, s).await }
                    },
                ),
                1,
            )
            .await;
        }

        // Text input handler (typing state)
        {
            let s = Arc::clone(&store);
            let s_check = Arc::clone(&store);
            app.add_typed_handler(
                FnHandler::new(
                    move |u| is_text_in_typing_state(u, &s_check),
                    move |update, ctx| {
                        let s = Arc::clone(&s);
                        async move { handle_text_input(update, ctx, s).await }
                    },
                ),
                1,
            )
            .await;
        }

        println!("Nested conversation bot is running. Press Ctrl+C to stop.");
        println!("Commands: /start, /stop");

        if let Err(e) = app.run_polling().await {
            eprintln!("Error running bot: {e}");
        }
    });
}
