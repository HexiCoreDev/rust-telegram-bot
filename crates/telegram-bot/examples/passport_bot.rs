//! Passport Bot -- demonstrates Telegram Passport data handling.
//!
//! This is the Rust port of Python's `passportbot.py`.
//!
//! Demonstrates:
//! - Receiving Telegram Passport data via `message.passport_data`
//! - Inspecting encrypted passport elements by type
//! - Logging phone numbers, emails, document types, and file info
//! - Handler structure for passport data updates
//!
//! Note: Actual decryption of passport data requires a private key and
//! cryptographic operations. This example shows the handler structure
//! and element inspection without performing real decryption.
//!
//! See https://telegram.org/blog/passport for info about Telegram Passport.
//!
//! # Usage
//!
//! ```sh
//! TELEGRAM_BOT_TOKEN="your-token-here" cargo run -p telegram-bot --example passport_bot
//! ```
use telegram_bot::ext::prelude::{
    Application, ApplicationBuilder, Arc, Context, FnHandler, HandlerResult, Update,
};

// ---------------------------------------------------------------------------
// Constants for passport element types
// ---------------------------------------------------------------------------

const PERSONAL_DETAILS: &str = "personal_details";
const PASSPORT: &str = "passport";
const DRIVER_LICENSE: &str = "driver_license";
const IDENTITY_CARD: &str = "identity_card";
const INTERNAL_PASSPORT: &str = "internal_passport";
const ADDRESS: &str = "address";
const UTILITY_BILL: &str = "utility_bill";
const BANK_STATEMENT: &str = "bank_statement";
const RENTAL_AGREEMENT: &str = "rental_agreement";
const PASSPORT_REGISTRATION: &str = "passport_registration";
const TEMPORARY_REGISTRATION: &str = "temporary_registration";
const PHONE_NUMBER: &str = "phone_number";
const EMAIL: &str = "email";

/// Types that contain encrypted data (personal_details, passport, etc.)
const DATA_TYPES: &[&str] = &[
    PERSONAL_DETAILS,
    PASSPORT,
    DRIVER_LICENSE,
    IDENTITY_CARD,
    INTERNAL_PASSPORT,
    ADDRESS,
];

/// Types that contain file arrays (utility_bill, bank_statement, etc.)
const FILE_TYPES: &[&str] = &[
    UTILITY_BILL,
    BANK_STATEMENT,
    RENTAL_AGREEMENT,
    PASSPORT_REGISTRATION,
    TEMPORARY_REGISTRATION,
];

/// Types that have a front side document.
const FRONT_SIDE_TYPES: &[&str] = &[PASSPORT, DRIVER_LICENSE, IDENTITY_CARD, INTERNAL_PASSPORT];

/// Types that have a reverse side document.
const REVERSE_SIDE_TYPES: &[&str] = &[DRIVER_LICENSE, IDENTITY_CARD];

/// Types that may have a selfie.
const SELFIE_TYPES: &[&str] = &[PASSPORT, DRIVER_LICENSE, IDENTITY_CARD, INTERNAL_PASSPORT];

/// Types that may have translations.
const TRANSLATION_TYPES: &[&str] = &[
    PASSPORT,
    DRIVER_LICENSE,
    IDENTITY_CARD,
    INTERNAL_PASSPORT,
    UTILITY_BILL,
    BANK_STATEMENT,
    RENTAL_AGREEMENT,
    PASSPORT_REGISTRATION,
    TEMPORARY_REGISTRATION,
];

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

/// Handle messages containing Telegram Passport data.
///
/// In a real implementation, you would:
/// 1. Read the private key from a file
/// 2. Decrypt credentials using `passport_data.credentials`
/// 3. Decrypt each element's data using the decrypted credentials
///
/// This example demonstrates the handler structure and element inspection.
async fn handle_passport_data(update: Arc<Update>, _context: Context) -> HandlerResult {
    let msg = update
        .effective_message()
        .expect("passport handler requires a message");

    let passport_data = match &msg.passport_data {
        Some(pd) => pd,
        None => return Ok(()),
    };

    println!(
        "Received Telegram Passport data with {} element(s)",
        passport_data.data.len()
    );

    // Note: In a real bot, you would decrypt the credentials first:
    //   let decrypted_credentials = decrypt_credentials(
    //       &passport_data.credentials,
    //       &private_key,
    //   );
    //   if decrypted_credentials.nonce != "thisisatest" { return Ok(()); }

    // Inspect each encrypted passport element.
    for element in &passport_data.data {
        let element_type = &element.element_type;

        match element_type.as_str() {
            PHONE_NUMBER => {
                println!(
                    "Phone: {}",
                    element.phone_number.as_deref().unwrap_or("[encrypted]")
                );
            }
            EMAIL => {
                println!(
                    "Email: {}",
                    element.email.as_deref().unwrap_or("[encrypted]")
                );
            }
            _ => {}
        }

        // Log data types (personal details, address, etc.)
        if DATA_TYPES.contains(&element_type.as_str()) {
            println!(
                "Element type '{}': has encrypted data = {}",
                element_type,
                element.data.is_some()
            );
        }

        // Log file types (utility bills, bank statements, etc.)
        if FILE_TYPES.contains(&element_type.as_str()) {
            let file_count = element.files.as_ref().map_or(0, |f| f.len());
            println!("Element type '{}': {} file(s)", element_type, file_count);
        }

        // Log front side availability
        if FRONT_SIDE_TYPES.contains(&element_type.as_str()) {
            println!(
                "Element type '{}': has front side = {}",
                element_type,
                element.front_side.is_some()
            );
        }

        // Log reverse side availability
        if REVERSE_SIDE_TYPES.contains(&element_type.as_str()) {
            println!(
                "Element type '{}': has reverse side = {}",
                element_type,
                element.reverse_side.is_some()
            );
        }

        // Log selfie availability
        if SELFIE_TYPES.contains(&element_type.as_str()) {
            println!(
                "Element type '{}': has selfie = {}",
                element_type,
                element.selfie.is_some()
            );
        }

        // Log translation availability
        if TRANSLATION_TYPES.contains(&element_type.as_str()) {
            let translation_count = element.translation.as_ref().map_or(0, |t| t.len());
            println!(
                "Element type '{}': {} translation file(s)",
                element_type, translation_count
            );
        }
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

    // Note: In a real bot, you would also load the private key:
    //   let private_key = std::fs::read("private.key")
    //       .expect("private.key must exist for Passport data decryption");

    let app: Arc<Application> = ApplicationBuilder::new().token(token).build();

    // Handle messages containing passport data.
    app.add_typed_handler(
        FnHandler::new(
            |u| {
                u.effective_message()
                    .and_then(|m| m.passport_data.as_ref())
                    .is_some()
            },
            handle_passport_data,
        ),
        0,
    )
    .await;

    println!("Passport bot is running. Press Ctrl+C to stop.");
    println!(
        "Note: This example logs passport data structure without decryption.\n\
         For real usage, provide a private key and implement decryption."
    );

    if let Err(e) = app.run_polling().await {
        eprintln!("Error running bot: {e}");
    }
}
