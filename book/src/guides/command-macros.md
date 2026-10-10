# Command Macros

Requires the `macros` feature (on the facade crate `rust-tg-bot`).

Typing every `CommandHandler::new("start", start)` by hand gets tedious once a
bot has more than a handful of commands. `#[derive(BotCommands)]` generates the
descriptions Telegram shows in its command menu from a single enum.

## Defining Commands

```rust
use rust_tg_bot::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display help text
    #[command(description = "Display help text")]
    Help,

    /// Start the bot
    #[command(description = "Start the bot")]
    Start,

    /// Set your display name
    #[command(description = "Set your display name")]
    Name(String),

    /// Set your age
    #[command(description = "Set your age")]
    Age(u32),

    /// Restart the bot (hidden from the menu)
    #[command(description = "Restart systems", hide)]
    Restart,
}
```

Each variant becomes a Telegram command. The leading slash is implicit, so
`Start` is `/start`.

### Attributes

| Attribute | Applies to | Effect |
|---|---|---|
| `#[command(rename_rule = "...")]` | enum | How variant names become command names — `"lowercase"`, `"snake_case"`, `"kebab-case"`, `"UPPERCASE"`, or `"CamelCase"` |
| `#[command(description = "...")]` | variant | Text shown in the Telegram command menu |
| `#[command(hide)]` | variant | Registers the command but hides it from the menu |
| `#[command(alias = "...")]` | variant | An additional name for the same command |

The doc comment (`/// ...`) on a variant is used as the description when no
explicit `description` is given.

## Generating the Menu Text

`Command::descriptions()` renders the menu, which is usually the body of a
`/help` handler:

```rust
async fn help(update: Arc<Update>, context: Context) -> HandlerResult {
    let chat_id = update.effective_chat().map(|c| c.id).unwrap();
    context.bot()
        .send_message(chat_id, Command::descriptions())
        .await?;
    Ok(())
}
```

## Parsing Text Into a Command

The derive also generates `Command::parse(text, bot_name)`, which handles the
`/cmd@botname` form Telegram uses in groups and turns a string into an enum
value:

```rust,ignore
match Command::parse(text, "my_bot") {
    Ok(Command::Help) => { /* ... */ }
    Ok(Command::Name(name)) => { /* name holds the argument */ }
    Ok(Command::Age(raw)) => { /* raw is the argument as a String */ }
    Ok(_) => {}
    Err(e) => tracing::warn!("unrecognised command: {e}"),
}
```

Variants with typed fields are still constructed from the raw argument text, so
`Age(u32)` arrives as the string you passed in — convert it where you use it.

Note that the macro generates `parse` and `descriptions`, **not** a
`CommandHandler` per variant. Registration goes through the normal API:

```rust,ignore
app.add_handler(CommandHandler::new("help", help), 0).await;
app.add_handler(CommandHandler::new("start", start), 0).await;
```

See the [Command Bots](command-bots.md) guide for argument parsing.

## Running the Example

The full example, including the generated menu output, is in the repository:

```sh
cargo run --example commands_derive_bot --features macros
```

## Next Steps

For hand-written command parsing and argument handling, see
[Command Bots](command-bots.md). For the underlying handler API, see
[Handlers](../core-concepts/handlers.md).
