//! Arbitrary Callback Data Bot -- demonstrates rich callback data beyond simple strings.
//!
//! This is the Rust port of Python's `arbitrarycallbackdatabot.py`.
//!
//! Demonstrates:
//! - `ApplicationBuilder::arbitrary_callback_data()` to enable the callback data cache
//! - Storing arbitrary `serde_json::Value` as callback data (not just strings)
//! - Retrieving resolved callback data from the cache
//! - `context.drop_callback_data()` to clean up after processing
//! - `/clear` to manually clear the callback data cache
//! - Handling invalid (expired/cleared) callback data gracefully
//!
//! When `arbitrary_callback_data` is enabled, the bot replaces the actual
//! callback data with short UUIDs before sending to Telegram, and resolves
//! the UUIDs back to the original data when a callback query arrives.
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p telegram-bot --example arbitrary_callback_data_bot
//! ```
//!
//! Then in Telegram:
//! - `/start` -- sends 5 number buttons; each click appends to a list
//! - `/help` -- shows usage info
//! - `/clear` -- clears the callback data cache (demonstrates invalid data handling)

use telegram_bot::ext::prelude::{
    Application, ApplicationBuilder, CommandHandler, Context, FnHandler, HandlerError,
    HandlerResult, InlineKeyboardButton, InlineKeyboardMarkup, Update, Arc,
    JsonValue,
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Maximum number of cached callback data entries.
const CALLBACK_DATA_CACHE_SIZE: usize = 1024;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_chat_id(update: &Update) -> i64 {
    update.effective_chat().expect("update must have a chat").id
}

/// Build an inline keyboard with 5 number buttons.
///
/// Each button's callback_data encodes both the selected number and the
/// current list of previously selected numbers as a JSON array. With
/// `arbitrary_callback_data` enabled, the framework caches this rich data
/// and replaces it with a short UUID for the actual Telegram API call.
fn build_keyboard(current_list: &[i64]) -> JsonValue {
    let buttons: Vec<InlineKeyboardButton> = (1..=5)
        .map(|i| {
            // The callback data is a JSON array: [selected_number, [...previous_selections]]
            let data = serde_json::to_value((i, current_list)).expect("tuple serialization");
            InlineKeyboardButton::callback(i.to_string(), data.to_string())
        })
        .collect();

    serde_json::to_value(InlineKeyboardMarkup::from_row(buttons)).expect("keyboard serialization")
}

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// `/start` -- send a message with 5 inline number buttons.
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    context
        .bot()
        .send_message(chat_id, "Please choose:")
        .reply_markup(build_keyboard(&[]))
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// `/help` -- show usage info.
async fn help_command(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    context
        .bot()
        .send_message(
            chat_id,
            "Use /start to test this bot. Use /clear to clear the stored data so that you can \
             see what happens if the button data is not available.",
        )
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// `/clear` -- clear the callback data cache.
async fn clear(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    // Clear the callback data cache via the ExtBot.
    // Pass `None` as time_cutoff to clear all data.
    if let Some(cache) = context.bot().callback_data_cache() {
        let mut guard = cache.write().await;
        guard.clear_callback_data(None);
        guard.clear_callback_queries();
    }

    context
        .bot()
        .send_message(chat_id, "All clear!")
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// Handle callback queries from the number buttons.
///
/// Parses the callback data to extract the selected number and the current
/// list, appends the new number, and updates the message with a fresh keyboard.
async fn list_button(update: Arc<Update>, context: Context) -> HandlerResult {
    let cq = update
        .callback_query
        .as_ref()
        .expect("callback query handler requires callback_query");

    // Answer the callback query.
    context
        .bot()
        .answer_callback_query(&cq.id)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    let data_str = cq.data.as_deref().unwrap_or("[]");

    // Try to parse the callback data as [number, [list...]].
    let parsed: JsonValue =
        serde_json::from_str(data_str).unwrap_or(JsonValue::Null);

    if parsed.is_null() || !parsed.is_array() {
        // Invalid callback data -- the cache was likely cleared.
        if let Some(ref msg) = cq.message {
            context
                .bot()
                .edit_message_text(
                    "Sorry, I could not process this button click. Please send /start to get a new keyboard.",
                )
                .chat_id(msg.chat().id)
                .message_id(msg.message_id())
                .send()
                .await
                .map_err(|e| HandlerError::Other(Box::new(e)))?;
        }
        return Ok(());
    }

    let arr = parsed.as_array().expect("verified as array");
    let number = arr.first().and_then(|v| v.as_i64()).unwrap_or(0);
    let mut number_list: Vec<i64> = arr
        .get(1)
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_i64()).collect())
        .unwrap_or_default();

    number_list.push(number);

    let text = format!(
        "So far you've selected {:?}. Choose the next item:",
        number_list
    );

    if let Some(ref msg) = cq.message {
        context
            .bot()
            .edit_message_text(&text)
            .chat_id(msg.chat().id)
            .message_id(msg.message_id())
            .reply_markup(build_keyboard(&number_list))
            .send()
            .await
            .map_err(|e| HandlerError::Other(Box::new(e)))?;
    }

    // Clean up the old callback data since we've replaced the keyboard.
    let _ = context.drop_callback_data(&cq.id).await;

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

    let app: Arc<Application> = ApplicationBuilder::new()
        .token(token)
        .arbitrary_callback_data(CALLBACK_DATA_CACHE_SIZE)
        .build();

    app.add_typed_handler(CommandHandler::new("start", start), 0)
        .await;
    app.add_typed_handler(CommandHandler::new("help", help_command), 0)
        .await;
    app.add_typed_handler(CommandHandler::new("clear", clear), 0)
        .await;
    app.add_typed_handler(FnHandler::on_callback_query(list_button), 0)
        .await;

    println!("Arbitrary callback data bot is running. Press Ctrl+C to stop.");
    println!("Commands: /start, /help, /clear");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
