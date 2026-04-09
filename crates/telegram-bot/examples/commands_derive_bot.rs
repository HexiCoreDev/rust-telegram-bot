//! Example demonstrating the `BotCommands` derive macro.
//!
//! Run with:
//! ```sh
//! cargo run --example commands_derive_bot --features macros
//! ```

use rust_tg_bot::BotCommands;

/// The set of commands this bot understands.
#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display help text
    #[command(description = "Display help text")]
    Help,

    /// Start the bot
    #[command(description = "Start the bot")]
    Start,

    /// Set username
    #[command(description = "Set username")]
    Username(String),

    /// Set age
    #[command(description = "Set age")]
    Age(u32),

    /// Hidden admin command
    #[command(description = "Restart systems", hide)]
    Restart,
}

fn main() {
    // ---- descriptions() ----
    println!("=== Bot Help Text ===");
    println!("{}", Command::descriptions());
    println!();

    // ---- bot_commands() ----
    println!("=== setMyCommands payload ===");
    for cmd in Command::bot_commands() {
        println!("  /{} - {}", cmd.command, cmd.description);
    }
    println!();

    // ---- parse() ----
    let test_cases = [
        "/help",
        "/start",
        "/username Ferris",
        "/age 42",
        "/help@MyBot",
        "/help@OtherBot",
        "/unknown",
        "/restart",
        "/age not_a_number",
    ];

    println!("=== Parsing test cases ===");
    for input in &test_cases {
        let result = Command::parse(input, "MyBot");
        match result {
            Ok(cmd) => println!("  {input:30} => Ok({cmd:?})"),
            Err(e) => println!("  {input:30} => Err({e})"),
        }
    }
}
