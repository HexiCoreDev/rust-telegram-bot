//! Chat Member Bot -- tracks chat member status changes (joins, leaves, etc.).
//!
//! This is the Rust port of Python's `chatmemberbot.py`.
//!
//! Demonstrates:
//! - Handling `my_chat_member` updates (bot added/removed from chats)
//! - Handling `chat_member` updates (users joining/leaving groups)
//! - Tracking which chats the bot is a member of via `bot_data`
//! - **Typed data guards** (`add_to_id_set`, `remove_from_id_set`, `get_id_set`)
//! - `/show_chats` command to display tracked chats
//! - Status change extraction from `ChatMemberUpdated`
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p rust-tg-bot --example chat_member_bot
//! ```

use rust_tg_bot::ext::prelude::{
    ApplicationBuilder, Arc, ChatType, CommandHandler, Context, FnHandler, HandlerResult, Update,
};
use rust_tg_bot::raw::types::chat_member::ChatMember;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Determine whether the old/new status represents a "member" of the chat.
fn is_member_status(status: &ChatMember) -> bool {
    matches!(
        status,
        ChatMember::Owner(_) | ChatMember::Administrator(_) | ChatMember::Member(_)
    ) || matches!(status, ChatMember::Restricted(r) if r.is_member)
}

/// Extract (was_member, is_member) from a `ChatMemberUpdated`.
/// Returns `None` if the status didn't effectively change.
fn extract_status_change(old: &ChatMember, new: &ChatMember) -> Option<(bool, bool)> {
    let was = is_member_status(old);
    let is = is_member_status(new);
    if was == is {
        // Also check if the actual variant changed (e.g., member -> restricted but still is_member).
        let old_variant = std::mem::discriminant(old);
        let new_variant = std::mem::discriminant(new);
        if old_variant == new_variant {
            return None;
        }
    }
    Some((was, is))
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Track which chats the bot is in by handling `my_chat_member` updates.
///
/// Uses typed `add_to_id_set` / `remove_from_id_set` instead of manual JSON
/// array manipulation.
async fn track_chats(update: Arc<Update>, context: Context) -> HandlerResult {
    let cmu = match update.my_chat_member() {
        Some(c) => c,
        None => return Ok(()),
    };

    let change = extract_status_change(&cmu.old_chat_member, &cmu.new_chat_member);
    let (was_member, is_member) = match change {
        Some(c) => c,
        None => return Ok(()),
    };

    let cause_name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("Unknown");

    let chat = &cmu.chat;
    let mut bd = context.bot_data_mut().await;

    if chat.chat_type == ChatType::Private {
        if !was_member && is_member {
            tracing::info!("{cause_name} unblocked the bot");
            bd.add_to_id_set("user_ids", chat.id);
        } else if was_member && !is_member {
            tracing::info!("{cause_name} blocked the bot");
            bd.remove_from_id_set("user_ids", chat.id);
        }
    } else if chat.chat_type == ChatType::Group || chat.chat_type == ChatType::Supergroup {
        let title = chat.title.as_deref().unwrap_or("unknown");
        if !was_member && is_member {
            tracing::info!("{cause_name} added the bot to the group {title}");
            bd.add_to_id_set("group_ids", chat.id);
        } else if was_member && !is_member {
            tracing::info!("{cause_name} removed the bot from the group {title}");
            bd.remove_from_id_set("group_ids", chat.id);
        }
    } else {
        // Channel or other.
        let title = chat.title.as_deref().unwrap_or("unknown");
        if !was_member && is_member {
            tracing::info!("{cause_name} added the bot to the channel {title}");
            bd.add_to_id_set("channel_ids", chat.id);
        } else if was_member && !is_member {
            tracing::info!("{cause_name} removed the bot from the channel {title}");
            bd.remove_from_id_set("channel_ids", chat.id);
        }
    }

    Ok(())
}

/// `/show_chats` -- display which chats the bot is tracking.
///
/// Uses typed `get_id_set` instead of manual `.get().and_then(|v| v.as_array())` chains.
async fn show_chats(update: Arc<Update>, context: Context) -> HandlerResult {
    let bd = context.bot_data().await;

    let format_ids = |ids: std::collections::HashSet<i64>| -> String {
        let mut sorted: Vec<_> = ids.into_iter().collect();
        sorted.sort_unstable();
        sorted
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };

    let user_ids = format_ids(bd.get_id_set("user_ids"));
    let group_ids = format_ids(bd.get_id_set("group_ids"));
    let channel_ids = format_ids(bd.get_id_set("channel_ids"));

    let text = format!(
        "The bot is currently in a conversation with the user IDs {user_ids}. \
         Moreover it is a member of the groups with IDs {group_ids} \
         and administrator in the channels with IDs {channel_ids}."
    );

    context.reply_text(&update, &text).await?;
    Ok(())
}

/// Greet new users in chats and announce when someone leaves.
/// Handles `chat_member` updates (not `my_chat_member`).
async fn greet_chat_members(update: Arc<Update>, context: Context) -> HandlerResult {
    let cmu = match update.chat_member() {
        Some(c) => c,
        None => return Ok(()),
    };

    let change = extract_status_change(&cmu.old_chat_member, &cmu.new_chat_member);
    let (was_member, is_member) = match change {
        Some(c) => c,
        None => return Ok(()),
    };

    let cause_name = &cmu.from_user.first_name;

    let member_user = match &cmu.new_chat_member {
        ChatMember::Owner(o) => &o.user,
        ChatMember::Administrator(a) => &a.user,
        ChatMember::Member(m) => &m.user,
        ChatMember::Restricted(r) => &r.user,
        ChatMember::Left(l) => &l.user,
        ChatMember::Banned(b) => &b.user,
    };
    let member_name = &member_user.first_name;

    let chat_id = cmu.chat.id;

    if !was_member && is_member {
        let text = format!("{member_name} was added by {cause_name}. Welcome!");
        let _ = context.bot().send_message(chat_id, &text).await;
    } else if was_member && !is_member {
        let text = format!("{member_name} is no longer with us. Thanks a lot, {cause_name} ...");
        let _ = context.bot().send_message(chat_id, &text).await;
    }

    Ok(())
}

/// Any message or command in a private chat triggers recording the user.
///
/// Uses typed `get_id_set` for membership checks and `add_to_id_set` for insertion.
async fn start_private_chat(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat = match update.effective_chat() {
        Some(c) => c,
        None => return Ok(()),
    };

    if chat.chat_type != ChatType::Private {
        return Ok(());
    }

    let user_name = update
        .effective_user()
        .map(|u| u.first_name.as_str())
        .unwrap_or("friend");

    // Check if already tracked.
    {
        let bd = context.bot_data().await;
        if bd.get_id_set("user_ids").contains(&chat.id) {
            return Ok(());
        }
    }

    tracing::info!("{user_name} started a private chat with the bot");

    {
        let mut bd = context.bot_data_mut().await;
        bd.add_to_id_set("user_ids", chat.id);
    }

    context
        .reply_text(
            &update,
            &format!("Welcome {user_name}. Use /show_chats to see what chats I'm in."),
        )
        .await?;

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

    // Track which chats the bot is in (my_chat_member updates).
    app.add_handler(FnHandler::on_my_chat_member(track_chats), 0)
        .await;

    // /show_chats command.
    app.add_handler(CommandHandler::new("show_chats", show_chats), 0)
        .await;

    // Greet members joining/leaving (chat_member updates).
    app.add_handler(FnHandler::on_chat_member(greet_chat_members), 0)
        .await;

    // Catch-all: record private chats.
    app.add_handler(FnHandler::on_message(start_private_chat), 1)
        .await;

    println!("Chat member bot is running. Press Ctrl+C to stop.");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
