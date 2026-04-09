# Contributing to rust-tg-bot

Welcome, and thank you for your interest in contributing. Whether you are fixing a typo, reporting a bug, adding a feature, or improving documentation, your contribution is valued.

This guide covers everything you need to know to contribute effectively.

---

## Table of Contents

- [Reporting Bugs](#reporting-bugs)
- [Requesting Features](#requesting-features)
- [Development Setup](#development-setup)
- [Architecture Overview](#architecture-overview)
- [Code Style](#code-style)
- [Testing Guidelines](#testing-guidelines)
- [Documentation Guidelines](#documentation-guidelines)
- [Commit Message Convention](#commit-message-convention)
- [Pull Request Process](#pull-request-process)
- [Adding a New Handler](#adding-a-new-handler)
- [Adding a New Filter](#adding-a-new-filter)
- [Adding a Bot API Type or Method](#adding-a-bot-api-type-or-method)
- [Code of Conduct](#code-of-conduct)
- [License](#license)

---

## Reporting Bugs

If you find a bug, please open an issue on [GitHub Issues](https://github.com/HexiCoreDev/rust-telegram-bot/issues) with the following information:

- **Rust version**: output of `rustc --version`
- **Crate version**: the version of `rust-tg-bot` you are using
- **Operating system**: platform and version
- **Bot API version**: which Telegram Bot API version you are targeting
- **Description**: what you expected to happen versus what actually happened
- **Minimal reproduction**: the smallest code example that triggers the bug
- **Error output**: full compiler or runtime error messages

If you believe you have found a security vulnerability, do **not** open a public issue. Instead, follow the instructions in [SECURITY.md](SECURITY.md).

## Requesting Features

Feature requests are welcome. When opening a feature request issue, please include:

- **Use case**: describe the problem you are trying to solve
- **Proposed API**: suggest how the feature might look to a user of the library
- **Python equivalent**: if this feature exists in [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot), link to the relevant documentation or source
- **Alternatives considered**: mention any workarounds you have tried

---

## Development Setup

### Prerequisites

- **Rust 1.75 or later** -- install via [rustup](https://rustup.rs):
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **A Telegram bot token** -- create one via [@BotFather](https://t.me/BotFather) for running examples and integration tests

### Clone and Build

```sh
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/<your-username>/rust-telegram-bot.git
cd rust-tg-bot

# Build the entire workspace
cargo build --workspace --all-features

# Run the test suite
cargo test --workspace --all-features
```

### Recommended Tools

```sh
# Install formatting and linting components
rustup component add clippy rustfmt

# Faster test runner (optional but recommended)
cargo install cargo-nextest

# Code coverage (optional)
cargo install cargo-tarpaulin
```

### Running Examples

```sh
TELEGRAM_BOT_TOKEN="your-token" cargo run -p rust-tg-bot --example echo_bot
```

See the `crates/telegram-bot/examples/` directory for all 20 example bots.

---

## Architecture Overview

The library is organized as a Cargo workspace with three crates:

```
rust-telegram-bot/
  crates/
    telegram-bot-raw/     # Pure API types and methods
    telegram-bot-ext/     # Application framework (handlers, filters, persistence, job queue)
    telegram-bot/         # Facade crate -- re-exports both for convenience
```

### Which crate do I modify?

| I want to...                                          | Modify this crate    |
|-------------------------------------------------------|----------------------|
| Add or fix a Bot API type (e.g., `Message`, `Update`) | `rust-tg-bot-raw`   |
| Add or fix a Bot API method (e.g., `send_message`)    | `rust-tg-bot-raw`   |
| Add a handler (e.g., `PollHandler`)                   | `rust-tg-bot-ext`   |
| Add a filter (e.g., `PREMIUM_USER`)                   | `rust-tg-bot-ext`   |
| Modify persistence, job queue, or application logic   | `rust-tg-bot-ext`   |
| Add or update an example bot                          | `rust-tg-bot`       |
| Update re-exports or the facade                       | `rust-tg-bot`       |

### Key Directories

```
crates/telegram-bot-raw/src/
  types/          # All Telegram API types (Message, Update, User, Chat, etc.)
  bot.rs          # Bot struct with all API methods as builder-returning functions
  constants.rs    # Typed enums (ParseMode, ChatType, ChatAction, etc.)

crates/telegram-bot-ext/src/
  handlers/       # Handler implementations (CommandHandler, MessageHandler, etc.)
  filters/        # Filter implementations (TEXT, COMMAND, PHOTO, etc.)
  persistence/    # Persistence backends (JSON, SQLite, trait definition)
  job_queue/      # Job scheduling (once, repeating, daily, monthly)
  application.rs  # Application struct and update processing loop
  builder.rs      # ApplicationBuilder with typestate pattern
  context.rs      # CallbackContext -- the Context passed to handler functions
  prelude.rs      # Convenient re-exports for user code

crates/telegram-bot/
  src/lib.rs      # Facade: re-exports raw and ext
  examples/       # 20 complete example bots
```

---

## Code Style

### Zero Warnings Policy

All code must compile with zero warnings. This is enforced in CI and is non-negotiable.

### Formatting

Run `cargo fmt` before every commit:

```sh
cargo fmt --all
```

The project uses the default `rustfmt` configuration. Do not add a `rustfmt.toml` with custom settings.

### Linting

All code must pass clippy with no warnings:

```sh
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Naming Conventions

- Types: `PascalCase` (e.g., `CommandHandler`, `MessageFilter`)
- Functions and methods: `snake_case` (e.g., `send_message`, `run_polling`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_PORT`)
- Filter constructor functions: `SCREAMING_SNAKE_CASE` with `()` (e.g., `TEXT()`, `COMMAND()`, `PHOTO()`) -- this matches the python-telegram-bot convention and is explicitly allowed via `#[allow(non_snake_case)]`
- Feature flags: `kebab-case` (e.g., `persistence-json`, `job-queue`)

### Error Handling

- Use `thiserror` for defining error types
- Propagate errors with `?` wherever possible
- Avoid `unwrap()` in library code -- use it only in examples where the intent is clear and documented with a comment

### Dependencies

- Do not add new dependencies without discussion in an issue or PR
- Prefer workspace dependencies declared in the root `Cargo.toml`
- Keep optional dependencies behind feature flags

---

## Testing Guidelines

### Running Tests

```sh
# Run all tests
cargo test --workspace --all-features

# Run tests for a specific crate
cargo test -p rust-tg-bot-ext --all-features

# Run a specific test
cargo test -p rust-tg-bot-ext --all-features -- test_name

# With cargo-nextest (faster)
cargo nextest run --workspace --all-features
```

### Writing Tests

- Place unit tests in the same file under `#[cfg(test)]` modules
- Use `serde_json::json!()` to construct test `Update` fixtures
- Test both the success path and the edge cases
- For filters, test that the filter matches updates it should match and rejects updates it should not match
- For handlers, test the predicate function (`check`) and the callback logic separately where possible
- Name test functions descriptively: `test_text_filter_matches_text_message`, not `test_1`

### Test Coverage

If you are adding a new feature, include tests that cover:

1. The primary behavior (happy path)
2. Edge cases (empty input, missing fields, boundary values)
3. Error conditions (invalid input, network failures where applicable)

---

## Documentation Guidelines

### Doc Comments

All public items must have doc comments (`///`). Include:

- A one-line summary
- A longer description if the item is non-obvious
- A `# Example` section with a code snippet where practical

```rust
/// Schedule a job that runs once after `delay`.
///
/// Returns a [`Job`] handle that can be used to cancel the job
/// before it fires.
///
/// # Example
///
/// ```rust,ignore
/// let job = jq.once(callback, Duration::from_secs(5))
///     .name("reminder")
///     .start()
///     .await;
/// job.schedule_removal(); // cancel before it fires
/// ```
pub async fn once(/* ... */) -> Job {
```

### Documentation Files

- Long-form documentation lives in the `docs/` directory
- If you add a new feature category (e.g., a new persistence backend), add a corresponding documentation file
- Keep code examples in documentation files in sync with the actual API

---

## Commit Message Convention

Use the following format:

```
<type>(<scope>): <short description>

<optional body>
```

### Types

| Type       | When to use                                    |
|------------|------------------------------------------------|
| `feat`     | A new feature                                  |
| `fix`      | A bug fix                                      |
| `docs`     | Documentation changes only                     |
| `test`     | Adding or updating tests                       |
| `refactor` | Code restructuring without behavior change     |
| `perf`     | Performance improvement                        |
| `ci`       | CI/CD configuration changes                    |
| `chore`    | Maintenance tasks (dependency updates, etc.)   |

### Scopes

Use the crate name as the scope when the change is crate-specific:

```
feat(ext): add ChatBoostHandler for boost update processing
fix(raw): correct deserialization of optional poll fields
docs: add persistence backend comparison to docs
test(ext): add property tests for filter composition
refactor(raw): extract shared pagination logic into helper
chore: update tokio to 1.38
```

### Rules

- Use imperative mood: "add feature", not "added feature" or "adds feature"
- Keep the first line under 72 characters
- Reference related issues in the body: `Closes #42`

---

## Pull Request Process

### Before You Start

1. Check existing issues and PRs to avoid duplicate work
2. For non-trivial changes, open an issue first to discuss the approach

### Creating a PR

1. **Fork** the repository and clone your fork
2. **Branch** from `main`:
   ```sh
   git checkout -b feat/your-feature-name
   ```
3. **Develop** your change, following the code style and testing guidelines above
4. **Run the full check suite** before pushing:
   ```sh
   cargo fmt --all --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace --all-features
   ```
5. **Push** your branch and open a pull request against `main`
6. **Write a clear PR description** that explains:
   - What changed and why
   - How to test the change
   - Any breaking changes
7. **Link related issues** (e.g., "Closes #42")

### PR Guidelines

- **One logical change per PR.** If you fix a bug and add a feature, submit two PRs.
- **Keep PRs small.** Large PRs are harder to review and more likely to have merge conflicts. If a feature is large, break it into incremental PRs.
- **Respond to review feedback** promptly and push new commits (do not force-push during review).
- **All CI checks must pass** before a PR can be merged.

---

## Adding a New Handler

If you are porting a handler from python-telegram-bot or creating a new one:

1. Create the handler file in `crates/telegram-bot-ext/src/handlers/`
2. Implement the `Handler` trait from `handlers::base`
3. Add the module to `handlers/mod.rs`
4. Add tests in the same file under `#[cfg(test)]`
5. Re-export the handler in `prelude.rs` if it is commonly used
6. Document the handler in `docs/handlers.md`
7. If applicable, add an example in `crates/telegram-bot/examples/`

## Adding a New Filter

1. Create the filter in `crates/telegram-bot-ext/src/filters/` (new file or existing module)
2. Implement the `Filter` trait from `filters::base`
3. Add the module or re-export to `filters/mod.rs`
4. Add a `SCREAMING_SNAKE_CASE` constructor function in `prelude.rs`
5. Write tests with representative `serde_json::json!()` update fixtures
6. Document the filter in `docs/filters.md`

## Adding a Bot API Type or Method

1. Create or update the type in `crates/telegram-bot-raw/src/types/`
2. Add the method to `crates/telegram-bot-raw/src/bot.rs` with a builder return type
3. Ensure all optional fields use `Option<T>` and are annotated with `#[serde(skip_serializing_if = "Option::is_none")]`
4. Derive `Debug`, `Clone`, `Serialize`, `Deserialize` as appropriate
5. Add re-exports to `types/mod.rs`
6. Add tests for serialization and deserialization

---

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold a welcoming, inclusive, and harassment-free community.

For enforcement concerns, contact [[judechinedu122@gmail.com](mailto:judechinedu122@gmail.com)](mailto:[judechinedu122@gmail.com](mailto:judechinedu122@gmail.com)).

## License

By contributing to rust-tg-bot, you agree that your contributions will be licensed under the [GNU Lesser General Public License v3.0](LICENSE).

---

Thank you for helping make rust-tg-bot better. We look forward to your contributions.
