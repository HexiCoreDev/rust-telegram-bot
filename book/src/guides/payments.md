# Payments

The Telegram Payments API lets your bot accept credit card and other payments directly in chat. You need a payment provider token from BotFather and a provider account (Stripe in test mode works for development).

## Getting a Provider Token

1. Send `/mybots` to `@BotFather`, select your bot, then choose **Payments**.
2. Pick a provider (Stripe for testing).
3. Complete the provider's onboarding to receive a token like `284685040:TEST:...`.

Set it as an environment variable: `PAYMENT_PROVIDER_TOKEN`.

## Sending an Invoice

Use `bot.send_invoice()` to create a payment message in the chat:

```rust
use rust_tg_bot::ext::prelude::{
    Application, ApplicationBuilder, Arc, CommandHandler, Context,
    FnHandler, HandlerError, HandlerResult, Update,
};
use rust_tg_bot::raw::types::payment::labeled_price::LabeledPrice;

async fn start_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap();

    context
        .bot()
        .send_message(
            chat_id,
            "Use /shipping to receive an invoice with shipping, \
             or /noshipping for an invoice without shipping.",
        )
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}
```

Prices are always in the smallest currency unit (cents for USD, pence for GBP, etc.).

### Invoice with Shipping

When you need to charge different shipping rates based on the destination, set `is_flexible(true)` and `need_shipping_address(true)`:

```rust
async fn start_with_shipping(
    update: Arc<Update>,
    context: Context,
    provider_token: String,
) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap();

    let prices = vec![
        serde_json::to_value(LabeledPrice::new("Test Item", 100))
            .expect("price serialization"),
    ];

    context
        .bot()
        .send_invoice(
            chat_id,
            "Payment Example",        // title
            "Example payment process", // description
            "Custom-Payload",          // your internal payload
            "USD",                     // currency
            prices,                    // price list
        )
        .provider_token(&provider_token)
        .need_name(true)
        .need_phone_number(true)
        .need_email(true)
        .need_shipping_address(true)
        .is_flexible(true)
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}
```

### Invoice without Shipping

For digital goods or services that do not require a shipping address:

```rust
async fn start_without_shipping(
    update: Arc<Update>,
    context: Context,
    provider_token: String,
) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap();

    let prices = vec![
        serde_json::to_value(LabeledPrice::new("Test Item", 100))
            .expect("price serialization"),
    ];

    context
        .bot()
        .send_invoice(
            chat_id,
            "Payment Example",
            "Example payment process",
            "Custom-Payload",
            "USD",
            prices,
        )
        .provider_token(&provider_token)
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}
```

## Handling Shipping Queries

When `is_flexible` is `true`, Telegram sends a `ShippingQuery` after the user enters their address. Register a handler with `FnHandler::on_shipping_query`:

```rust
use rust_tg_bot::raw::types::payment::shipping_option::ShippingOption;

async fn shipping_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let query = update
        .shipping_query()
        .expect("shipping handler requires shipping_query");

    // Verify the payload matches your bot
    if query.invoice_payload != "Custom-Payload" {
        context
            .bot()
            .answer_shipping_query(&query.id, false)
            .error_message("Something went wrong...")
            .await
            .map_err(|e| HandlerError::Other(Box::new(e)))?;
        return Ok(());
    }

    // Define available shipping options
    let options = vec![
        serde_json::to_value(ShippingOption::new(
            "1",
            "Shipping Option A",
            vec![LabeledPrice::new("A", 100)],
        )).expect("shipping option serialization"),
        serde_json::to_value(ShippingOption::new(
            "2",
            "Shipping Option B",
            vec![LabeledPrice::new("B1", 150), LabeledPrice::new("B2", 200)],
        )).expect("shipping option serialization"),
    ];

    context
        .bot()
        .answer_shipping_query(&query.id, true)
        .shipping_options(options)
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}
```

Register the handler:

```rust
app.add_handler(FnHandler::on_shipping_query(shipping_callback), 0).await;
```

## Pre-Checkout Confirmation

Before Telegram charges the user, it sends a `PreCheckoutQuery`. You must answer within 10 seconds. This is your last chance to validate the order.

```rust
async fn precheckout_callback(update: Arc<Update>, context: Context) -> HandlerResult {
    let query = update
        .pre_checkout_query()
        .expect("pre-checkout handler requires pre_checkout_query");

    if query.invoice_payload != "Custom-Payload" {
        context
            .bot()
            .answer_pre_checkout_query(&query.id, false)
            .error_message("Something went wrong...")
            .await
            .map_err(|e| HandlerError::Other(Box::new(e)))?;
    } else {
        context
            .bot()
            .answer_pre_checkout_query(&query.id, true)
            .await
            .map_err(|e| HandlerError::Other(Box::new(e)))?;
    }

    Ok(())
}
```

Register the handler:

```rust
app.add_handler(FnHandler::on_pre_checkout_query(precheckout_callback), 0).await;
```

## Handling Successful Payments

After the charge completes, Telegram sends a regular `Message` containing a `SuccessfulPayment` object. Use a `FnHandler` with a predicate that checks for the `successful_payment` field:

```rust
async fn successful_payment_callback(
    update: Arc<Update>,
    context: Context,
) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap();

    context
        .bot()
        .send_message(chat_id, "Thank you for your payment.")
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}

// Register with a predicate that checks for successful_payment
app.add_handler(
    FnHandler::new(
        |u| {
            u.effective_message()
                .and_then(|m| m.successful_payment.as_ref())
                .is_some()
        },
        successful_payment_callback,
    ),
    0,
).await;
```

## LabeledPrice and ShippingOption

### LabeledPrice

Represents a single price component. The amount is in the smallest currency unit.

```rust
use rust_tg_bot::raw::types::payment::labeled_price::LabeledPrice;

let item = LabeledPrice::new("Widget", 1500);      // $15.00
let shipping = LabeledPrice::new("Shipping", 500);  // $5.00
let discount = LabeledPrice::new("Discount", -200); // -$2.00
// Total shown to user: $18.00
```

Negative amounts create discount lines. The final total must be positive.

### ShippingOption

Groups one or more `LabeledPrice` items under a named shipping method:

```rust
use rust_tg_bot::raw::types::payment::shipping_option::ShippingOption;

let standard = ShippingOption::new(
    "standard",
    "Standard (5-7 days)",
    vec![LabeledPrice::new("Standard Shipping", 500)],
);

let express = ShippingOption::new(
    "express",
    "Express (1-2 days)",
    vec![
        LabeledPrice::new("Express Shipping", 1500),
        LabeledPrice::new("Insurance", 200),
    ],
);
```

## Telegram Stars

Telegram Stars are a digital currency that users can use to pay for digital goods. To accept Stars, use `"XTR"` as the currency and omit the provider token:

```rust
async fn send_stars_invoice(
    update: Arc<Update>,
    context: Context,
) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap();

    let prices = vec![
        serde_json::to_value(LabeledPrice::new("Premium Access", 100))
            .expect("price serialization"),
    ];

    context
        .bot()
        .send_invoice(
            chat_id,
            "Premium Access",
            "Unlock premium features for your account",
            "premium-payload",
            "XTR",    // Telegram Stars currency code
            prices,
        )
        .await
        .map_err(|e| HandlerError::Other(Box::new(e)))?;

    Ok(())
}
```

With Stars, the `provider_token` is not required -- Telegram handles the payment directly.

## Complete Example

```rust
use rust_tg_bot::ext::prelude::{
    Application, ApplicationBuilder, Arc, CommandHandler, Context,
    FnHandler, HandlerError, HandlerResult, MessageEntityType, Update,
};
use rust_tg_bot::raw::types::payment::labeled_price::LabeledPrice;
use rust_tg_bot::raw::types::payment::shipping_option::ShippingOption;

fn check_command(update: &Update, expected: &str) -> bool {
    let msg = match update.effective_message() {
        Some(m) => m,
        None => return false,
    };
    let text = match msg.text.as_deref() {
        Some(t) => t,
        None => return false,
    };
    msg.entities
        .as_ref()
        .and_then(|e| e.first())
        .map_or(false, |e| {
            e.entity_type == MessageEntityType::BotCommand
                && e.offset == 0
                && text[1..e.length as usize]
                    .split('@')
                    .next()
                    .unwrap_or("")
                    .eq_ignore_ascii_case(expected)
        })
}

// ... define start_callback, start_with_shipping, start_without_shipping,
//     shipping_callback, precheckout_callback, successful_payment_callback
//     (as shown above) ...

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let provider_token = std::env::var("PAYMENT_PROVIDER_TOKEN").unwrap();

    let app: Arc<Application> = ApplicationBuilder::new().token(token).build();

    app.add_handler(CommandHandler::new("start", start_callback), 0).await;

    // /shipping command
    {
        let pt = provider_token.clone();
        app.add_handler(
            FnHandler::new(
                |u| check_command(u, "shipping"),
                move |update, ctx| {
                    let pt = pt.clone();
                    async move { start_with_shipping(update, ctx, pt).await }
                },
            ),
            0,
        ).await;
    }

    // /noshipping command
    {
        let pt = provider_token.clone();
        app.add_handler(
            FnHandler::new(
                |u| check_command(u, "noshipping"),
                move |update, ctx| {
                    let pt = pt.clone();
                    async move { start_without_shipping(update, ctx, pt).await }
                },
            ),
            0,
        ).await;
    }

    app.add_handler(FnHandler::on_shipping_query(shipping_callback), 0).await;
    app.add_handler(FnHandler::on_pre_checkout_query(precheckout_callback), 0).await;
    app.add_handler(
        FnHandler::new(
            |u| {
                u.effective_message()
                    .and_then(|m| m.successful_payment.as_ref())
                    .is_some()
            },
            successful_payment_callback,
        ),
        0,
    ).await;

    println!("Payment bot is running. Press Ctrl+C to stop.");

    app.run_polling().await.unwrap();
}
```

## Next Steps

- [Inline Mode](inline-mode.md) -- inline mode can initiate payment flows.
- [Conversations](conversations.md) -- use a conversation handler for multi-step checkout.
