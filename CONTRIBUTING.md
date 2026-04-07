# Contributing to rust-telegram-bot

Thank you for considering a contribution. This document covers the process for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```sh
   git clone https://github.com/<your-username>/rust-telegram-bot.git
   cd rust-telegram-bot
   ```
3. Create a branch:
   ```sh
   git checkout -b feature/your-feature-name
   ```

## Development Environment

**Required:**
- Rust 1.75 or later (install via [rustup](https://rustup.rs))
- A Telegram bot token for running examples (get one from [@BotFather](https://t.me/BotFather))

**Recommended tools:**
```sh
rustup component add clippy rustfmt
cargo install cargo-nextest   # faster test runner
cargo install cargo-tarpaulin # code coverage
```

## Project Structure

```
crates/
  telegram-bot-raw/   # Pure API types and methods
  telegram-bot-ext/   # Application framework (handlers, filters, persistence, job queue)
  telegram-bot/       # Facade crate with re-exports and examples
```

When making changes, identify which crate the change belongs to:
- New Bot API types or methods go in `telegram-bot-raw`
- Handler, filter, persistence, or scheduling changes go in `telegram-bot-ext`
- Examples and integration tests go in `telegram-bot`

## Code Standards

### Formatting

Run `cargo fmt` before every commit:

```sh
cargo fmt --all
```

### Linting

All code must pass clippy with no warnings:

```sh
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Testing

All tests must pass:

```sh
cargo test --workspace --all-features
```

If you are adding a new feature, add tests for it. Aim for meaningful coverage of the behavior you are adding, including edge cases.

### Documentation

All public items must have doc comments. Include a brief description and, where practical, a usage example:

```rust
/// Schedule a job that runs once after `delay`.
///
/// Returns a [`Job`] handle that can be used to cancel the job
/// before it fires.
///
/// # Example
///
/// ```rust,ignore
/// let job = jq.run_once("greeting", Duration::from_secs(5), callback, None, None, None).await;
/// job.schedule_removal(); // cancel before it fires
/// ```
pub async fn run_once(/* ... */) -> Job {
```

## Commit Messages

Use clear, descriptive commit messages:

```
feat(ext): add ChatBoostHandler for boost update processing

fix(raw): correct deserialization of optional poll fields

docs: add persistence backend comparison to docs

test(ext): add property tests for filter composition

refactor(raw): extract shared pagination logic into helper
```

Prefix with the scope (`raw`, `ext`, `bot`, `docs`, `ci`, `test`) when possible.

## Pull Request Process

1. Ensure your branch is up to date with `main`
2. Run the full check suite:
   ```sh
   cargo fmt --all --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace --all-features
   ```
3. Write a clear PR description explaining what changed and why
4. Link any related issues
5. Keep PRs focused -- one logical change per PR

## Adding a New Handler

If you are porting a handler from python-telegram-bot or adding a new one:

1. Create the handler file in `crates/telegram-bot-ext/src/handlers/`
2. Implement the `Handler` trait from `handlers::base`
3. Add the module to `handlers/mod.rs`
4. Add tests in the same file under `#[cfg(test)]`
5. Add documentation in `docs/handlers.md`
6. If applicable, add an example in `crates/telegram-bot/examples/`

## Adding a New Filter

1. Create the filter file in `crates/telegram-bot-ext/src/filters/` or add to an existing module
2. Implement the `Filter` trait from `filters::base`
3. Add the module/re-export to `filters/mod.rs`
4. Add tests with representative `serde_json::json!()` update fixtures
5. Document the filter in `docs/filters.md`

## Adding a Bot API Type or Method

1. Create or update the type file in `crates/telegram-bot-raw/src/types/`
2. Add the method to `crates/telegram-bot-raw/src/bot.rs`
3. Ensure all fields use `Option<T>` for optional parameters
4. Derive `Debug`, `Clone`, `Serialize`, `Deserialize` where appropriate
5. Add re-exports to `types/mod.rs`

## Reporting Issues

When reporting a bug:
- Include the Rust version (`rustc --version`)
- Include the relevant error output
- Provide a minimal reproduction if possible
- Note which Bot API version you are targeting

For feature requests:
- Describe the use case
- If this exists in python-telegram-bot, link to the relevant documentation
- Suggest an API design if you have one in mind

## Code of Conduct

Be respectful. Be constructive. We are all here to build something useful.

## License

By contributing, you agree that your contributions will be licensed under the LGPL-3.0 license.
