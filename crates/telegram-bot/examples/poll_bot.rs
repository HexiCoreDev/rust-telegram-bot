//! Poll Bot -- demonstrates creating polls, receiving answers, and quiz mode.
//!
//! This is the Rust port of Python's `pollbot.py`.
//!
//! Demonstrates:
//! - `send_poll` for creating regular polls and quizzes
//! - Handling `poll_answer` updates
//! - Handling `poll` updates (for quiz auto-close)
//! - Storing poll metadata in `bot_data`
//! - `stop_poll` when enough participants have voted
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p rust-tg-bot --example poll_bot
//! ```
use rust_tg_bot::ext::prelude::{
    json, ApplicationBuilder, Arc, CommandHandler, Context, FnHandler, HandlerResult, JsonValue,
    Update,
};
use rust_tg_bot::raw::types::poll::InputPollOption;

/// After this many voters, polls and quizzes are automatically closed.
const TOTAL_VOTER_COUNT: i64 = 3;

// ---------------------------------------------------------------------------
// Command handlers
// ---------------------------------------------------------------------------

/// `/start` -- describe what the bot can do.
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Please select /poll to get a Poll, /quiz to get a Quiz, or /help for more info.",
        )
        .await?;
    Ok(())
}

/// `/help` -- show available commands.
async fn help_handler(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(&update, "Use /quiz, /poll to test this bot.")
        .await?;
    Ok(())
}

/// `/poll` -- send a predefined multi-answer poll.
async fn poll_command(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap_or(0);

    let options: Vec<JsonValue> = vec![
        serde_json::to_value(InputPollOption::new("Good")).expect("poll option serialization"),
        serde_json::to_value(InputPollOption::new("Really good"))
            .expect("poll option serialization"),
        serde_json::to_value(InputPollOption::new("Fantastic")).expect("poll option serialization"),
        serde_json::to_value(InputPollOption::new("Great")).expect("poll option serialization"),
    ];

    let msg = context
        .bot()
        .send_poll(chat_id, "How are you?", options)
        .is_anonymous(false)
        .allows_multiple_answers(true)
        .await?;

    // Store poll metadata in bot_data for later use in receive_poll_answer.
    if let Some(ref poll) = msg.poll {
        let payload = json!({
            "questions": ["Good", "Really good", "Fantastic", "Great"],
            "message_id": msg.message_id,
            "chat_id": chat_id,
            "answers": 0,
        });
        context
            .set_chat_data(format!("poll_{}", poll.id), payload)
            .await;
        // Also store in bot_data for cross-chat access.
        let mut bd = context.bot_data_mut().await;
        bd.insert(
            format!("poll_{}", poll.id),
            json!({
                "questions": ["Good", "Really good", "Fantastic", "Great"],
                "message_id": msg.message_id,
                "chat_id": chat_id,
                "answers": 0,
            }),
        );
    }

    Ok(())
}

/// `/quiz` -- send a predefined quiz poll.
async fn quiz_command(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap_or(0);

    let options: Vec<JsonValue> = vec![
        serde_json::to_value(InputPollOption::new("1")).expect("poll option serialization"),
        serde_json::to_value(InputPollOption::new("2")).expect("poll option serialization"),
        serde_json::to_value(InputPollOption::new("4")).expect("poll option serialization"),
        serde_json::to_value(InputPollOption::new("20")).expect("poll option serialization"),
    ];

    let msg = context
        .bot()
        .send_poll(chat_id, "How many eggs do you need for a cake?", options)
        .poll_type("quiz")
        .correct_option_id(2)
        .await?;

    if let Some(ref poll) = msg.poll {
        let mut bd = context.bot_data_mut().await;
        bd.insert(
            format!("quiz_{}", poll.id),
            json!({
                "chat_id": chat_id,
                "message_id": msg.message_id,
            }),
        );
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Update handlers (poll_answer and poll)
// ---------------------------------------------------------------------------

/// Handle a `poll_answer` update -- summarize the user's vote.
async fn receive_poll_answer(update: Arc<Update>, context: Context) -> HandlerResult {
    let answer = match update.poll_answer() {
        Some(a) => a,
        None => return Ok(()),
    };

    let poll_key = format!("poll_{}", answer.poll_id);

    let (questions, chat_id, message_id, answers) = {
        let bd = context.bot_data().await;
        let entry = match bd.get(&poll_key) {
            Some(v) => v.clone(),
            None => return Ok(()),
        };
        let questions: Vec<String> = entry["questions"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let chat_id = entry["chat_id"].as_i64().unwrap_or(0);
        let message_id = entry["message_id"].as_i64().unwrap_or(0);
        let answers = entry["answers"].as_i64().unwrap_or(0);
        (questions, chat_id, message_id, answers)
    };

    // Build a string describing the user's choices.
    let selected: Vec<String> = answer
        .option_ids
        .iter()
        .filter_map(|&id| questions.get(id as usize).cloned())
        .collect();
    let answer_string = selected.join(" and ");

    let user_name = answer
        .user
        .as_ref()
        .map(|u| u.first_name.as_str())
        .unwrap_or("Someone");

    let text = format!("{user_name} feels {answer_string}!");
    let _ = context.bot().send_message(chat_id, &text).await;

    // Track answer count and close the poll after TOTAL_VOTER_COUNT.
    let new_count = answers + 1;
    {
        let mut bd = context.bot_data_mut().await;
        if let Some(entry) = bd.get_mut(&poll_key) {
            entry["answers"] = json!(new_count);
        }
    }

    if new_count >= TOTAL_VOTER_COUNT {
        let _ = context
            .bot()
            .stop_poll(chat_id.into(), message_id, None, None)
            .await;
    }

    Ok(())
}

/// Handle a `poll` update -- close quiz after enough participants.
async fn receive_quiz_answer(update: Arc<Update>, context: Context) -> HandlerResult {
    let poll = match update.poll() {
        Some(p) => p,
        None => return Ok(()),
    };

    if poll.is_closed {
        return Ok(());
    }

    if poll.total_voter_count >= TOTAL_VOTER_COUNT {
        let quiz_key = format!("quiz_{}", poll.id);
        let (chat_id, message_id) = {
            let bd = context.bot_data().await;
            match bd.get(&quiz_key) {
                Some(entry) => (
                    entry["chat_id"].as_i64().unwrap_or(0),
                    entry["message_id"].as_i64().unwrap_or(0),
                ),
                None => return Ok(()),
            }
        };

        let _ = context
            .bot()
            .stop_poll(chat_id.into(), message_id, None, None)
            .await;
    }

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

    // Command handlers.
    app.add_handler(CommandHandler::new("start", start), 0)
        .await;
    app.add_handler(CommandHandler::new("poll", poll_command), 0)
        .await;
    app.add_handler(CommandHandler::new("quiz", quiz_command), 0)
        .await;
    app.add_handler(CommandHandler::new("help", help_handler), 0)
        .await;

    // Poll answer handler (fires when a user votes in a non-anonymous poll).
    app.add_handler(FnHandler::on_poll_answer(receive_poll_answer), 0)
        .await;

    // Poll handler (fires when a poll state changes, e.g., vote counts).
    app.add_handler(FnHandler::on_poll(receive_quiz_answer), 0)
        .await;

    println!("Poll bot is running. Press Ctrl+C to stop.");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
