//! Raw API Bot -- uses the `Bot` directly without the ext framework.
//!
//! This is the Rust port of Python's `rawapibot.py`.
//!
//! Demonstrates:
//! - Using `Bot` directly (no `Application`, no `Handler` system)
//! - Manual polling loop with `get_updates`
//! - `send_message` and `send_photo` builders
//! - Offset tracking for update acknowledgement
//! - **Builder pattern** for clean API calls (no more 15 `None` arguments)
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p telegram-bot --example raw_api_bot
//! ```

use std::sync::Arc;

use telegram_bot::raw::bot::Bot;
use telegram_bot::raw::request::reqwest_impl::ReqwestRequest;
use telegram_bot::raw::types::files::input_file::InputFile;
use telegram_bot::raw::constants::MessageEntityType;

fn main() {
    telegram_bot::run(async {
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable must be set");

    // Create the HTTP request backend and the raw Bot.
    let request = Arc::new(
        ReqwestRequest::new().expect("Failed to create HTTP client"),
    );
    let mut bot = Bot::new(&token, request);

    // Initialize the bot (calls getMe to verify the token).
    match bot.initialize().await {
        Ok(()) => {
            let bot_data = bot.bot_data().await;
            if let Some(user) = bot_data {
                println!(
                    "Bot initialized: @{} (id: {})",
                    user.username.as_deref().unwrap_or("unknown"),
                    user.id
                );
            } else {
                println!("Bot initialized (no user data cached yet).");
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize bot: {e}");
            return;
        }
    }

    println!("Raw API bot is running. Press Ctrl+C to stop.");
    println!("Send /start for a greeting, /photo for a sample photo, or any text to echo.");

    // Manual polling loop.
    let mut offset: Option<i64> = None;

    loop {
        let updates = match bot
            .get_updates(offset, Some(100), Some(30), None)
            .await
        {
            Ok(u) => u,
            Err(e) => {
                eprintln!("Error fetching updates: {e}");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
        };

        for update in updates {
            // Advance the offset to acknowledge this update.
            offset = Some(update.update_id + 1);

            // Process only message updates.
            let message = match &update.message {
                Some(msg) => msg,
                None => continue,
            };

            let chat_id = message.chat.id;
            let text = message.text.as_deref().unwrap_or("");

            // Check for commands.
            let is_command = message
                .entities
                .as_ref()
                .and_then(|ents| ents.first())
                .map(|e| e.entity_type == MessageEntityType::BotCommand && e.offset == 0)
                .unwrap_or(false);

            if is_command {
                let cmd_length = message
                    .entities
                    .as_ref()
                    .and_then(|ents| ents.first())
                    .map(|e| e.length as usize)
                    .unwrap_or(0);

                let raw_cmd = if cmd_length <= text.len() {
                    &text[1..cmd_length]
                } else {
                    ""
                };
                let cmd_name = raw_cmd.split('@').next().unwrap_or("");

                match cmd_name.to_lowercase().as_str() {
                    "start" => {
                        let user_name = message
                            .from_user
                            .as_ref()
                            .map(|u| u.first_name.as_str())
                            .unwrap_or("friend");

                        let greeting = format!(
                            "Hello, {user_name}! This bot uses the raw Telegram API directly.\n\n\
                             Available commands:\n\
                             /start - This greeting\n\
                             /photo - Get a sample photo\n\n\
                             Send any text and I will echo it back."
                        );

                        if let Err(e) = bot
                            .send_message(chat_id, &greeting)
                            .send()
                            .await
                        {
                            eprintln!("Failed to send message: {e}");
                        }
                    }
                    "photo" => {
                        // Send a photo from a URL.
                        let photo = InputFile::Url(
                            "https://telegram.org/img/t_logo.png".to_string(),
                        );

                        match bot
                            .send_photo(chat_id, photo)
                            .caption("Here is the Telegram logo!")
                            .send()
                            .await
                        {
                            Ok(msg) => {
                                println!(
                                    "Sent photo to chat {}, message_id: {}",
                                    message.chat.id, msg.message_id
                                );
                            }
                            Err(e) => {
                                eprintln!("Failed to send photo: {e}");
                                let _ = bot
                                    .send_message(
                                        chat_id,
                                        "Sorry, I could not send the photo. Try again later.",
                                    )
                                    .send()
                                    .await;
                            }
                        }
                    }
                    _ => {
                        let reply = format!("Unknown command: /{cmd_name}\nTry /start or /photo.");
                        let _ = bot
                            .send_message(chat_id, &reply)
                            .send()
                            .await;
                    }
                }
            } else if !text.is_empty() {
                // Echo non-command text.
                if let Err(e) = bot
                    .send_message(chat_id, text)
                    .send()
                    .await
                {
                    eprintln!("Failed to echo message: {e}");
                }
            }
        }
    }
    });
}
