//! Teloxide benchmark bot — identical features to PTB and RTB versions.
//!
//! Features: /start (with inline keyboard), /help, echo with typing action,
//! callback query handler, webhook mode on port 8000.

use teloxide::dispatching::UpdateFilterExt;
use teloxide::dptree;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::types::{
    ChatAction, InlineKeyboardButton, InlineKeyboardMarkup,
};
use teloxide::update_listeners::webhooks;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let bot = Bot::from_env();
    let webhook_url = std::env::var("WEBHOOK_URL")
        .expect("WEBHOOK_URL must be set")
        .parse()
        .expect("WEBHOOK_URL must be a valid URL");

    let addr = ([127, 0, 0, 1], 8000).into();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, webhook_url))
        .await
        .expect("Failed to create webhook listener");

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(command_handler),
                )
                .branch(dptree::entry().endpoint(echo_handler)),
        )
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    println!("Teloxide benchmark bot running on port 8000. Send /start to test.");

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(listener, LoggingErrorHandler::new())
        .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Start,
    Help,
}

async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            let name = msg
                .from
                .as_ref()
                .map(|u| u.first_name.as_str())
                .unwrap_or("there");

            let keyboard = InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("Option 1", "1"),
                    InlineKeyboardButton::callback("Option 2", "2"),
                ],
                vec![InlineKeyboardButton::callback("Option 3", "3")],
            ]);

            bot.send_message(msg.chat.id, format!("Hi {name}! I am a benchmark bot.\nUse /help for info."))
                .reply_markup(keyboard)
                .await?;
        }
        Command::Help => {
            bot.send_message(
                msg.chat.id,
                "Commands: /start, /help\nSend any text to echo.\nPress inline buttons to test callbacks.",
            )
            .await?;
        }
    }
    Ok(())
}

async fn echo_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        bot.send_chat_action(msg.chat.id, ChatAction::Typing)
            .await?;
        bot.send_message(msg.chat.id, text).await?;
    }
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    bot.answer_callback_query(q.id.clone()).await?;
    if let (Some(data), Some(msg)) = (q.data.as_ref(), q.message.as_ref()) {
        let chat_id = msg.chat().id;
        let msg_id = msg.id();
        bot.edit_message_text(chat_id, msg_id, format!("You selected: Option {data}"))
            .await?;
    }
    Ok(())
}
