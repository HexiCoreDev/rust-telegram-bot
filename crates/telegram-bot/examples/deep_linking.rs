//! Deep Linking Bot -- demonstrates Telegram's deep linking parameters.
//!
//! This is the Rust port of Python's `deeplinking.py`.
//!
//! Demonstrates:
//! - Generating deep-linked URLs with `create_deep_linked_url`
//! - Routing `/start <payload>` to different handlers based on the payload
//! - Inline keyboards with deep-link URLs
//! - Callback query handler for inline keyboard buttons
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p telegram-bot --example deep_linking
//! ```

use telegram_bot::ext::prelude::*;
use telegram_bot::types::inline::inline_keyboard_button::InlineKeyboardButton;
use telegram_bot::types::inline::inline_keyboard_markup::InlineKeyboardMarkup;

// ---------------------------------------------------------------------------
// Deep-link payload constants
// ---------------------------------------------------------------------------

const CHECK_THIS_OUT: &str = "check-this-out";
const USING_ENTITIES: &str = "using-entities-here";
const USING_KEYBOARD: &str = "using-keyboard-here";
const SO_COOL: &str = "so-cool";

const KEYBOARD_CALLBACKDATA: &str = "keyboard-callback-data";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Create a deep-linked URL: `https://t.me/{username}?start={payload}`
/// (or `?startgroup={payload}` for groups).
fn create_deep_linked_url(bot_username: &str, payload: &str, group: bool) -> String {
    let param = if group { "startgroup" } else { "start" };
    format!("https://t.me/{bot_username}?{param}={payload}")
}

fn extract_chat_id(update: &Update) -> i64 {
    update.effective_chat().map(|c| c.id).unwrap_or(0)
}

/// Retrieve the bot's username from cached bot_data (populated by initialize/getMe).
async fn get_bot_username(context: &Context) -> String {
    context
        .bot()
        .bot_data()
        .await
        .and_then(|u| u.username)
        .unwrap_or_else(|| "bot".to_string())
}

/// Extract the /start payload from context args.
fn get_start_payload(context: &Context) -> Option<String> {
    context.args.as_ref().and_then(|args| args.first().cloned())
}

/// Check whether the update is a /start command with a specific payload.
fn is_start_with_payload(update: &Update, payload: &str) -> bool {
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
    let is_cmd = entities.first().map_or(false, |e| {
        e.entity_type == MessageEntityType::BotCommand && e.offset == 0
    });
    if !is_cmd {
        return false;
    }
    // text looks like "/start <payload>"
    text.starts_with("/start") && text.contains(payload)
}

fn is_plain_start(update: &Update) -> bool {
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
    let is_cmd = entities.first().map_or(false, |e| {
        e.entity_type == MessageEntityType::BotCommand && e.offset == 0
    });
    // Only match bare "/start" (no payload).
    is_cmd && (text.trim() == "/start" || text.trim().starts_with("/start@"))
}

fn keyboard_markup_json(markup: &InlineKeyboardMarkup) -> serde_json::Value {
    serde_json::to_value(markup).expect("keyboard serialization")
}

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// Plain `/start` -- send a deep-linked URL for sharing.
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let bot_username = get_bot_username(&context).await;

    let url = create_deep_linked_url(&bot_username, CHECK_THIS_OUT, true);
    let text = format!("Feel free to tell your friends about it:\n\n{url}");

    context.bot().send_message(chat_id, &text).send().await?;

    Ok(())
}

/// Reached through the CHECK_THIS_OUT payload.
async fn deep_linked_level_1(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let bot_username = get_bot_username(&context).await;

    let url = create_deep_linked_url(&bot_username, SO_COOL, false);
    let text =
        "Awesome, you just accessed hidden functionality! Now let's get back to the private chat.";

    let keyboard =
        InlineKeyboardMarkup::from_button(InlineKeyboardButton::url("Continue here!", url));

    context
        .bot()
        .send_message(chat_id, text)
        .reply_markup(keyboard_markup_json(&keyboard))
        .send()
        .await?;

    Ok(())
}

/// Reached through the SO_COOL payload.
async fn deep_linked_level_2(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let bot_username = get_bot_username(&context).await;

    let url = create_deep_linked_url(&bot_username, USING_ENTITIES, false);
    let text = format!(
        "You can also mask the deep-linked URLs as links: <a href=\"{url}\">CLICK HERE</a>."
    );

    context
        .bot()
        .send_message(chat_id, &text)
        .parse_mode(ParseMode::Html)
        .send()
        .await?;

    Ok(())
}

/// Reached through the USING_ENTITIES payload.
async fn deep_linked_level_3(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    let keyboard = InlineKeyboardMarkup::from_button(InlineKeyboardButton::callback(
        "Like this!",
        KEYBOARD_CALLBACKDATA,
    ));

    context
        .bot()
        .send_message(
            chat_id,
            "It is also possible to make deep-linking using InlineKeyboardButtons.",
        )
        .reply_markup(keyboard_markup_json(&keyboard))
        .send()
        .await?;

    Ok(())
}

/// Callback query from the inline button -- answer with the deep link URL.
async fn deep_link_level_3_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let cq = match &update.callback_query {
        Some(q) => q,
        None => return Ok(()),
    };

    let bot_username = get_bot_username(&context).await;
    let url = create_deep_linked_url(&bot_username, USING_KEYBOARD, false);

    context
        .bot()
        .answer_callback_query(&cq.id)
        .url(url)
        .send()
        .await?;

    Ok(())
}

/// Reached through the USING_KEYBOARD payload -- the deepest level.
async fn deep_linked_level_4(update: Arc<Update>, context: Context) -> HandlerResult {
    let payload = get_start_payload(&context).unwrap_or_default();

    context
        .reply_text(
            &update,
            &format!("Congratulations! This is as deep as it gets.\n\nThe payload was: {payload}"),
        )
        .await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    telegram_bot::run(async {
        tracing_subscriber::fmt::init();

        let token = std::env::var("TELEGRAM_BOT_TOKEN")
            .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

        let app = ApplicationBuilder::new().token(token).build();

        // Deep-link handlers are registered BEFORE the plain /start handler.
        // The first matching handler wins within its group.

        // Level 1: /start check-this-out
        app.add_typed_handler(
            FnHandler::new(
                |u| is_start_with_payload(u, CHECK_THIS_OUT),
                deep_linked_level_1,
            ),
            0,
        )
        .await;

        // Level 2: /start so-cool
        app.add_typed_handler(
            FnHandler::new(|u| is_start_with_payload(u, SO_COOL), deep_linked_level_2),
            0,
        )
        .await;

        // Level 3: /start using-entities-here
        app.add_typed_handler(
            FnHandler::new(
                |u| is_start_with_payload(u, USING_ENTITIES),
                deep_linked_level_3,
            ),
            0,
        )
        .await;

        // Level 4: /start using-keyboard-here
        app.add_typed_handler(
            FnHandler::new(
                |u| is_start_with_payload(u, USING_KEYBOARD),
                deep_linked_level_4,
            ),
            0,
        )
        .await;

        // Callback query handler for the inline keyboard button.
        app.add_typed_handler(
            FnHandler::new(
                |u| {
                    u.callback_query.as_ref().and_then(|cq| cq.data.as_deref())
                        == Some(KEYBOARD_CALLBACKDATA)
                },
                deep_link_level_3_callback,
            ),
            0,
        )
        .await;

        // Plain /start -- must be registered AFTER the deep-link handlers.
        app.add_typed_handler(FnHandler::new(is_plain_start, start), 0)
            .await;

        println!("Deep linking bot is running. Press Ctrl+C to stop.");

        if let Err(e) = app.run_polling().await {
            eprintln!("Error running bot: {e}");
        }
    });
}
