//! Payment Bot -- demonstrates sending invoices and handling payments.
//!
//! This is the Rust port of Python's `paymentbot.py`.
//!
//! Demonstrates:
//! - Sending invoices with `bot.send_invoice()`
//! - Handling shipping queries with `bot.answer_shipping_query()`
//! - Handling pre-checkout queries with `bot.answer_pre_checkout_query()`
//! - Handling successful payments
//! - Flexible pricing with shipping options
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" \
//! PAYMENT_PROVIDER_TOKEN="your-provider-token" \
//! cargo run -p telegram-bot --example payment_bot
//! ```
//!
//! Then in Telegram:
//! - `/start` -- shows usage instructions
//! - `/shipping` -- sends an invoice with shipping required
//! - `/noshipping` -- sends an invoice without shipping

use telegram_bot::ext::prelude::{
    Application, ApplicationBuilder, CommandHandler, Context, FnHandler, HandlerError,
    HandlerResult, MessageEntityType, Update, Arc,
};
use telegram_bot::raw::types::payment::labeled_price::LabeledPrice;
use telegram_bot::raw::types::payment::shipping_option::ShippingOption;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Unique payload to identify payment requests from this bot.
const INVOICE_PAYLOAD: &str = "Custom-Payload";

/// Invoice details.
const INVOICE_TITLE: &str = "Payment Example";
const INVOICE_DESCRIPTION: &str =
    "Example of a payment process using the rust-telegram-bot library.";
const INVOICE_CURRENCY: &str = "USD";
/// Price in cents (100 = $1.00).
const INVOICE_PRICE_CENTS: i64 = 100;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_chat_id(update: &Update) -> i64 {
    update.effective_chat().expect("update must have a chat").id
}

// ---------------------------------------------------------------------------
// Handler functions
// ---------------------------------------------------------------------------

/// `/start` -- display usage instructions.
async fn start_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    context
        .bot()
        .send_message(
            chat_id,
            "Use /shipping to receive an invoice with shipping included, \
             or /noshipping for an invoice without shipping.",
        )
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// `/shipping` -- send an invoice that triggers a shipping query.
async fn start_with_shipping(
    update: Arc<Update>,
    context: Context,
    provider_token: String,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    let prices = vec![
        serde_json::to_value(LabeledPrice::new("Test", INVOICE_PRICE_CENTS))
            .expect("price serialization"),
    ];

    context
        .bot()
        .send_invoice(
            chat_id,
            INVOICE_TITLE,
            INVOICE_DESCRIPTION,
            INVOICE_PAYLOAD,
            INVOICE_CURRENCY,
            prices,
        )
        .provider_token(&provider_token)
        .need_name(true)
        .need_phone_number(true)
        .need_email(true)
        .need_shipping_address(true)
        .is_flexible(true)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// `/noshipping` -- send an invoice without requiring shipping details.
async fn start_without_shipping(
    update: Arc<Update>,
    context: Context,
    provider_token: String,
) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    let prices = vec![
        serde_json::to_value(LabeledPrice::new("Test", INVOICE_PRICE_CENTS))
            .expect("price serialization"),
    ];

    context
        .bot()
        .send_invoice(
            chat_id,
            INVOICE_TITLE,
            INVOICE_DESCRIPTION,
            INVOICE_PAYLOAD,
            INVOICE_CURRENCY,
            prices,
        )
        .provider_token(&provider_token)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// Handle incoming shipping queries.
async fn shipping_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let query = update
        .shipping_query
        .as_ref()
        .expect("shipping query handler requires shipping_query");

    // Verify the payload matches our bot.
    if query.invoice_payload != INVOICE_PAYLOAD {
        context
            .bot()
            .answer_shipping_query(&query.id, false)
            .error_message("Something went wrong...")
            .send()
            .await
            .map_err(|e| HandlerError::Other(Box::new(e)))?;
        return Ok(());
    }

    // Define available shipping options.
    let options = vec![
        serde_json::to_value(ShippingOption::new(
            "1",
            "Shipping Option A",
            vec![LabeledPrice::new("A", 100)],
        ))
        .expect("shipping option serialization"),
        serde_json::to_value(ShippingOption::new(
            "2",
            "Shipping Option B",
            vec![
                LabeledPrice::new("B1", 150),
                LabeledPrice::new("B2", 200),
            ],
        ))
        .expect("shipping option serialization"),
    ];

    context
        .bot()
        .answer_shipping_query(&query.id, true)
        .shipping_options(options)
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

/// Handle pre-checkout queries (final confirmation).
async fn precheckout_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let query = update
        .pre_checkout_query
        .as_ref()
        .expect("pre-checkout query handler requires pre_checkout_query");

    if query.invoice_payload != INVOICE_PAYLOAD {
        context
            .bot()
            .answer_pre_checkout_query(&query.id, false)
            .error_message("Something went wrong...")
            .send()
            .await
            .map_err(|e| HandlerError::Other(Box::new(e)))?;
    } else {
        context
            .bot()
            .answer_pre_checkout_query(&query.id, true)
            .send()
            .await
            .map_err(|e| HandlerError::Other(Box::new(e)))?;
    }

    Ok(())
}

/// Handle successful payments.
async fn successful_payment_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = extract_chat_id(&update);

    context
        .bot()
        .send_message(chat_id, "Thank you for your payment.")
        .send()
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

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

    let provider_token = std::env::var("PAYMENT_PROVIDER_TOKEN")
        .expect("PAYMENT_PROVIDER_TOKEN environment variable must be set");

    let app: Arc<Application> = ApplicationBuilder::new().token(token).build();

    // /start
    app.add_typed_handler(CommandHandler::new("start", start_callback), 0)
        .await;

    // /shipping
    {
        let pt = provider_token.clone();
        app.add_typed_handler(
            FnHandler::new(
                |u| {
                    u.effective_message()
                        .and_then(|m| m.text.as_deref())
                        .and_then(|t| {
                            let entities = u.effective_message()?.entities.as_ref()?;
                            let e = entities.first()?;
                            if e.entity_type == MessageEntityType::BotCommand && e.offset == 0 {
                                let cmd = t[1..e.length as usize].split('@').next()?;
                                if cmd.eq_ignore_ascii_case("shipping") {
                                    Some(true)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .is_some()
                },
                move |update, ctx| {
                    let pt = pt.clone();
                    async move { start_with_shipping(update, ctx, pt).await }
                },
            ),
            0,
        )
        .await;
    }

    // /noshipping
    {
        let pt = provider_token.clone();
        app.add_typed_handler(
            FnHandler::new(
                |u| {
                    u.effective_message()
                        .and_then(|m| m.text.as_deref())
                        .and_then(|t| {
                            let entities = u.effective_message()?.entities.as_ref()?;
                            let e = entities.first()?;
                            if e.entity_type == MessageEntityType::BotCommand && e.offset == 0 {
                                let cmd = t[1..e.length as usize].split('@').next()?;
                                if cmd.eq_ignore_ascii_case("noshipping") {
                                    Some(true)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .is_some()
                },
                move |update, ctx| {
                    let pt = pt.clone();
                    async move { start_without_shipping(update, ctx, pt).await }
                },
            ),
            0,
        )
        .await;
    }

    // Shipping query handler
    app.add_typed_handler(FnHandler::on_shipping_query(shipping_callback), 0)
        .await;

    // Pre-checkout query handler
    app.add_typed_handler(FnHandler::on_pre_checkout_query(precheckout_callback), 0)
        .await;

    // Successful payment handler
    app.add_typed_handler(
        FnHandler::new(
            |u| {
                u.effective_message()
                    .and_then(|m| m.successful_payment.as_ref())
                    .is_some()
            },
            successful_payment_callback,
        ),
        0,
    )
    .await;

    println!("Payment bot is running. Press Ctrl+C to stop.");
    println!("Commands: /start, /shipping, /noshipping");

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
