//! Web App Bot -- demonstrates Telegram Web App integration.
//!
//! This is the Rust port of Python's `webappbot.py`.
//!
//! Demonstrates:
//! - Sending a `KeyboardButton` with `web_app` field to open a Web App
//! - Receiving `WebAppData` from the Web App
//! - Parsing JSON data sent from the Web App
//! - Removing the custom keyboard after receiving data
//!
//! The Web App itself is a separate HTML/JS page hosted externally.
//! This example uses the PTB team's hosted color picker as the Web App URL.
//!
//! Note: Web App buttons only work in private chats and require HTTPS URLs.
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p telegram-bot --example webapp_bot
//! ```
//!
//! Then in Telegram:
//! - `/start` -- shows a keyboard button that opens the color picker Web App
//! - Select a color in the Web App; the bot will display the chosen color

use std::sync::Arc;

use serde_json::json;

use telegram_bot::ext::prelude::*;
use telegram_bot::types::keyboard_button::KeyboardButton;
use telegram_bot::types::reply_keyboard_markup::ReplyKeyboardMarkup;
use telegram_bot::types::web_app_info::WebAppInfo;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// URL of the Web App (color picker hosted by the PTB team).
const WEBAPP_URL: &str = "https://python-telegram-bot.org/static/webappbot";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_chat_id(update: &Update) -> i64 {
    update.effective_chat().expect("update must have a chat").id
}

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// `/start` -- send a keyboard button that opens the Web App.
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    // Build a ReplyKeyboardMarkup with a single button that opens the Web App.
    let mut button = KeyboardButton::text("Open the color picker!");
    button.web_app = Some(WebAppInfo {
        url: WEBAPP_URL.to_string(),
    });

    let keyboard = serde_json::to_value(
        ReplyKeyboardMarkup::new(vec![vec![button]])
            .resize()
            .one_time(),
    )
    .expect("keyboard serialization");

    context
        .bot()
        .send_message(
            chat_id,
            "Please press the button below to choose a color via the WebApp.",
        )
        .reply_markup(keyboard)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// Handle incoming Web App data.
async fn web_app_data(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);
    let msg = update
        .effective_message()
        .expect("web_app_data handler requires a message");

    let webapp_data = match &msg.web_app_data {
        Some(d) => d,
        None => return Ok(()),
    };

    // The Web App sends data as a JSON-serialized string.
    let parsed: serde_json::Value = serde_json::from_str(&webapp_data.data)
        .unwrap_or_else(|_| json!({"raw": &webapp_data.data}));

    let hex = parsed
        .get("hex")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let rgb_text = parsed
        .get("rgb")
        .map(|rgb| {
            let r = rgb.get("r").and_then(|v| v.as_i64()).unwrap_or(0);
            let g = rgb.get("g").and_then(|v| v.as_i64()).unwrap_or(0);
            let b = rgb.get("b").and_then(|v| v.as_i64()).unwrap_or(0);
            format!("({r}, {g}, {b})")
        })
        .unwrap_or_else(|| "unknown".to_string());

    let text = format!(
        "You selected the color with the HEX value {hex}. \
         The corresponding RGB value is {rgb_text}."
    );

    // Reply with the selected color and remove the custom keyboard.
    context
        .bot()
        .send_message(chat_id, &text)
        .reply_markup(json!({"remove_keyboard": true}))
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

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

        let app: Arc<Application> = ApplicationBuilder::new().token(token).build();

        // /start
        app.add_typed_handler(CommandHandler::new("start", start), 0)
            .await;

        // Handle Web App data (messages with web_app_data present).
        app.add_typed_handler(
            FnHandler::new(
                |u| {
                    u.effective_message()
                        .and_then(|m| m.web_app_data.as_ref())
                        .is_some()
                },
                web_app_data,
            ),
            0,
        )
        .await;

        println!("Web App bot is running. Press Ctrl+C to stop.");
        println!("Commands: /start");
        println!("Note: The Web App HTML page is hosted externally at {WEBAPP_URL}");

        if let Err(e) = app.run_polling().await {
            eprintln!("Error running bot: {e}");
        }
    });
}
