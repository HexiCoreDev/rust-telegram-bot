//! Rich Message Bot -- demonstrates sending Bot API rich messages.
//!
//! Demonstrates:
//! - `sendRichMessage` with explicit blocks (heading, paragraph, list)
//! - `InputRichMessage::markdown` markup via the `reply_rich_message` shortcut
//! - Streaming a draft with `sendRichMessageDraft`, then persisting it with
//!   `sendRichMessage`
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p rust-tg-bot --example rich_message_bot
//! ```
use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, CommandHandler, Context, HandlerResult, Update,
};
use rust_tg_bot::raw::types::input_rich_block::{
    InputRichBlock, InputRichBlockList, InputRichBlockListItem, InputRichBlockParagraph,
    InputRichBlockSectionHeading,
};
use rust_tg_bot::raw::types::input_rich_message::InputRichMessage;

/// `/start` -- describe what the bot can do.
async fn start(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_text(
            &update,
            "Send /rich for a block-built message, /markdown for markup, or /stream for a draft preview.",
        )
        .await?;
    Ok(())
}

/// `/rich` -- send a message built from explicit blocks.
async fn rich_command(update: Arc<Update>, context: Context) -> HandlerResult {
    let message = InputRichMessage::blocks(vec![
        InputRichBlock::SectionHeading(InputRichBlockSectionHeading::new("Quarterly report", 1)),
        InputRichBlock::Paragraph(InputRichBlockParagraph::new(
            "Highlights with **bold** and _italic_ text.",
        )),
        InputRichBlock::List(InputRichBlockList::new(vec![
            InputRichBlockListItem::new(vec![InputRichBlock::Paragraph(
                InputRichBlockParagraph::new("Revenue up: +$1M"),
            )]),
            InputRichBlockListItem::new(vec![InputRichBlock::Paragraph(
                InputRichBlockParagraph::new("Churn down: -2%"),
            )]),
        ])),
    ]);
    let chat_id = update
        .effective_chat()
        .map(|c| c.id)
        .expect("chat in update");
    context.bot().send_rich_message(chat_id, message).await?;
    Ok(())
}

/// `/markdown` -- send markup through the context shortcut.
async fn markdown_command(update: Arc<Update>, context: Context) -> HandlerResult {
    context
        .reply_rich_message(
            &update,
            InputRichMessage::markdown("# Hello\n\n- one\n- two"),
        )
        .await?;
    Ok(())
}

/// `/stream` -- stream a partial draft, then persist the final message.
async fn stream_command(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update
        .effective_chat()
        .map(|c| c.id)
        .expect("chat in update");
    let draft_id = 1;
    context
        .bot()
        .send_rich_message_draft(chat_id, draft_id, InputRichMessage::markdown("Thinking…"))
        .can_stop(true)
        .await?;
    context
        .bot()
        .send_rich_message(
            chat_id,
            InputRichMessage::markdown("# Done\n\nFinal answer."),
        )
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
    app.add_handler(CommandHandler::new("rich", rich_command), 0)
        .await;
    app.add_handler(CommandHandler::new("markdown", markdown_command), 0)
        .await;
    app.add_handler(CommandHandler::new("stream", stream_command), 0)
        .await;

    println!("Rich message bot is running. Press Ctrl+C to stop.");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
