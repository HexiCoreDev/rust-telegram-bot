//! Ephemeral Bot -- demonstrates private bot-user exchanges inside group chats.
//!
//! Demonstrates:
//! - `reply_ephemeral` for sending a message visible only to one user
//! - Automatic `callback_query_id` routing when answering from a button press
//! - Receiving guest messages via the `GUEST_MESSAGE` update filter and
//!   answering them with `answerGuestQuery`
//!
//! Add the bot to a group and try `/whisper` there, or press the button sent
//! by `/buttons`.
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p rust-tg-bot --example ephemeral_bot
//! ```
use rust_tg_bot::ext::filters::update_type::GUEST_MESSAGE;
use rust_tg_bot::ext::prelude::{
    json, ApplicationBuilder, Arc, CommandHandler, Context, F, FnHandler, HandlerResult,
    InlineKeyboardButton, InlineKeyboardMarkup, MessageHandler, Update,
};
use rust_tg_bot::raw::types::update::UpdateKind;

/// `/start` -- describe what the bot can do.
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "In a group, send /whisper to get a private reply, or /buttons for a private button answer.",
        )
        .await?;
    Ok(())
}

/// `/whisper <text>` -- reply ephemerally to the sender.
async fn whisper_command(update: Arc<Update>, context: Context) -> HandlerResult {
    let user_id = update
        .effective_user()
        .map(|u| u.id)
        .expect("user in update");
    context
        .reply_ephemeral(&update, user_id, "Only you can see this.")
        .await?;
    Ok(())
}

/// `/buttons` -- send a message with a button that answers privately.
async fn buttons_command(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update
        .effective_chat()
        .map(|c| c.id)
        .expect("chat in update");
    let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "Whisper back",
        "whisper",
    )]]);
    context
        .bot()
        .send_message(chat_id, "Press the button for a private answer:")
        .reply_markup(json!(keyboard))
        .await?;
    Ok(())
}

/// Button press -- the callback query id routes the ephemeral reply to the
/// pressing client automatically.
async fn button_press(update: Arc<Update>, context: Context) -> HandlerResult {
    let user_id = update
        .effective_user()
        .map(|u| u.id)
        .expect("user in update");
    context
        .reply_ephemeral(&update, user_id, "Heard you — privately.")
        .await?;
    Ok(())
}

/// Guest message -- a message from a chat the bot is not a member of.
async fn guest_message(update: Arc<Update>, context: Context) -> HandlerResult {
    let query_id = match &update.kind {
        UpdateKind::GuestMessage(msg) => msg.guest_query_id.clone().unwrap_or_default(),
        _ => return Ok(()),
    };
    let result = json!({
        "type": "article",
        "id": "1",
        "title": "Hello, guest",
        "input_message_content": {"message_text": "Thanks for summoning me!"},
    });
    context
        .bot()
        .answer_guest_query(query_id, result)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

    let app = ApplicationBuilder::new().token(token).build();

    app.add_handler(CommandHandler::new("start", start), 0)
        .await;
    app.add_handler(CommandHandler::new("whisper", whisper_command), 0)
        .await;
    app.add_handler(CommandHandler::new("buttons", buttons_command), 0)
        .await;
    app.add_handler(FnHandler::on_callback_query(button_press), 0)
        .await;
    // Guest messages are message-like but arrive under their own update kind.
    app.add_handler(MessageHandler::new(F::new(GUEST_MESSAGE), guest_message), 0)
        .await;

    println!("Ephemeral bot is running. Press Ctrl+C to stop.");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
