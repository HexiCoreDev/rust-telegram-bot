# rust-tg-bot Makefile
# Usage: make <target>

SHELL := /bin/bash
.DEFAULT_GOAL := help

# Extract workspace version from Cargo.toml
VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')

# ── Build ──────────────────────────────────────────────────────────

.PHONY: build build-release check

build: ## Build all crates (debug)
	cargo build --workspace

build-release: ## Build all crates (release, optimized)
	cargo build --workspace --release

check: ## Type-check without building
	cargo check --workspace

# ── Test ───────────────────────────────────────────────────────────

.PHONY: test test-all test-verbose

test: ## Run all tests
	cargo test --workspace

test-all: ## Run all tests with all features enabled
	cargo test --workspace --all-features

test-verbose: ## Run tests with output shown
	cargo test --workspace -- --nocapture

# ── Code Quality ───────────────────────────────────────────────────

.PHONY: clippy fmt fmt-check lint

clippy: ## Run clippy lints
	cargo clippy --workspace -- -D warnings

fmt: ## Format all code
	cargo fmt --all

fmt-check: ## Check formatting without modifying
	cargo fmt --all -- --check

lint: clippy fmt-check ## Run all lints (clippy + format check)

# ── Coverage ───────────────────────────────────────────────────────

.PHONY: coverage coverage-html

coverage: ## Generate coverage report (terminal summary)
	cargo tarpaulin --workspace --timeout 120

coverage-html: ## Generate HTML coverage report and open it
	cargo tarpaulin --workspace --timeout 120 --out html
	@echo "Opening tarpaulin-report.html"
	@xdg-open tarpaulin-report.html 2>/dev/null || open tarpaulin-report.html 2>/dev/null || true

# ── Documentation ──────────────────────────────────────────────────

.PHONY: doc doc-open book book-serve

doc: ## Generate API docs
	cargo doc --workspace --no-deps

doc-open: ## Generate and open API docs in browser
	cargo doc --workspace --no-deps --open

book: ## Build the mdBook guide
	mdbook build book

book-serve: ## Serve mdBook locally with live reload
	mdbook serve book --open

# ── Examples ───────────────────────────────────────────────────────

.PHONY: echo inline-keyboard timer conversation raw-api error-handler
.PHONY: poll-bot inline-bot deep-linking context-types chat-member
.PHONY: webhook custom-webhook bench-bot commands-derive
.PHONY: inline-keyboard2 conversation2 nested-conversation
.PHONY: persistent-conversation payment passport webapp
.PHONY: arbitrary-callback list-examples

# Run any example: make example NAME=echo_bot
.PHONY: example
example: ## Run an example by name (make example NAME=echo_bot)
	cargo run -p rust-tg-bot --example $(NAME)

# Shortcut targets for common examples
echo: ## Run echo_bot example
	cargo run -p rust-tg-bot --example echo_bot

inline-keyboard: ## Run inline_keyboard example
	cargo run -p rust-tg-bot --example inline_keyboard

timer: ## Run timer_bot example (requires job-queue feature)
	cargo run -p rust-tg-bot --example timer_bot --features job-queue

conversation: ## Run conversation_bot example
	cargo run -p rust-tg-bot --example conversation_bot

raw-api: ## Run raw_api_bot example
	cargo run -p rust-tg-bot --example raw_api_bot

error-handler: ## Run error_handler_bot example
	cargo run -p rust-tg-bot --example error_handler_bot

poll-bot: ## Run poll_bot example
	cargo run -p rust-tg-bot --example poll_bot

inline-bot: ## Run inline_bot example
	cargo run -p rust-tg-bot --example inline_bot

deep-linking: ## Run deep_linking example
	cargo run -p rust-tg-bot --example deep_linking

context-types: ## Run context_types_bot example
	cargo run -p rust-tg-bot --example context_types_bot

chat-member: ## Run chat_member_bot example
	cargo run -p rust-tg-bot --example chat_member_bot

webhook: ## Run webhook_bot example (requires webhooks feature)
	cargo run -p rust-tg-bot --example webhook_bot --features webhooks

custom-webhook: ## Run custom_webhook_bot example (requires webhooks feature)
	cargo run -p rust-tg-bot --example custom_webhook_bot --features webhooks

bench-bot: ## Run bench_bot example (requires webhooks feature)
	cargo run -p rust-tg-bot --example bench_bot --features webhooks

commands-derive: ## Run commands_derive_bot example (requires macros feature)
	cargo run -p rust-tg-bot --example commands_derive_bot --features macros

persistent-conversation: ## Run persistent_conversation_bot (requires persistence-json)
	cargo run -p rust-tg-bot --example persistent_conversation_bot --features persistence-json

list-examples: ## List all available examples
	@echo "Available examples:"
	@ls crates/telegram-bot/examples/*.rs | xargs -I{} basename {} .rs | sort | sed 's/^/  /'

# ── Release ────────────────────────────────────────────────────────

.PHONY: tag release pre-release publish-dry

tag: ## Create a git tag for current version (v1.0.0-beta.3)
	@echo "Tagging v$(VERSION)"
	git tag -a "v$(VERSION)" -m "Release v$(VERSION)"
	@echo "Run 'make push-tag' to push"

push-tag: ## Push the latest tag to origin
	git push origin "v$(VERSION)"

release: lint test tag push-tag ## Full release: lint + test + tag + push
	@echo "Release v$(VERSION) pushed. CI will handle the rest."

pre-release: lint test ## Pre-release check: lint + test (no tag/push)
	@echo "All checks passed. Ready to release v$(VERSION)"

publish-dry: ## Dry-run crates.io publish (no actual upload)
	cargo publish -p rust-tg-bot-raw --dry-run
	cargo publish -p rust-tg-bot-macros --dry-run
	cargo publish -p rust-tg-bot-ext --dry-run
	cargo publish -p rust-tg-bot --dry-run

# ── Git Workflow ───────────────────────────────────────────────────

.PHONY: merge-to-main

merge-to-main: pre-release ## Merge dev into main and push (triggers crates.io publish)
	git checkout main
	git merge dev --no-ff -m "Merge branch 'dev' into main — v$(VERSION) release"
	git push origin main
	git checkout dev
	@echo "Merged to main. CI will publish to crates.io."

# ── Benchmarks ─────────────────────────────────────────────────────

.PHONY: bench bench-rtb bench-teloxide bench-ptb bench-all bench-build bench-memory

bench: bench-rtb ## Alias for bench-rtb

bench-rtb: ## Run RTB memory benchmark (needs TELEGRAM_BOT_TOKEN + WEBHOOK_URL)
	./benchmarks/run_benchmark.sh rtb

bench-teloxide: ## Run teloxide memory benchmark
	./benchmarks/run_benchmark.sh teloxide

bench-ptb: ## Run PTB memory benchmark
	./benchmarks/run_benchmark.sh ptb

bench-all: ## Run all three benchmarks sequentially
	@echo "=== RTB ===" && ./benchmarks/run_benchmark.sh rtb 20
	@echo ""
	@echo "=== teloxide ===" && ./benchmarks/run_benchmark.sh teloxide 20
	@echo ""
	@echo "=== PTB ===" && ./benchmarks/run_benchmark.sh ptb 20

bench-build: ## Build all benchmark bots (release mode)
	cargo build --release -p rust-tg-bot --example bench_bot --features webhooks
	cd benchmarks/teloxide && cargo build --release
	@echo "PTB needs: pip install 'python-telegram-bot[webhooks]' starlette uvicorn"

bench-memory: build-release ## Measure RTB memory footprint without running a bot
	@echo "Example binary sizes (release, stripped):"
	@cargo build --release -p rust-tg-bot --example echo_bot 2>/dev/null
	@cargo build --release -p rust-tg-bot --example bench_bot --features webhooks 2>/dev/null
	@ls -lh target/release/examples/echo_bot target/release/examples/bench_bot 2>/dev/null || true
	@echo ""
	@echo "To measure RSS, run a bot and check: ps -o rss,vsz,comm -p <PID>"

# ── Cleanup ────────────────────────────────────────────────────────

.PHONY: clean clean-all

clean: ## Remove build artifacts
	cargo clean

clean-all: clean ## Remove build artifacts + coverage + docs
	rm -rf tarpaulin-report.html cobertura.xml lcov.info coverage/
	rm -rf book/book/

# ── Info ───────────────────────────────────────────────────────────

.PHONY: version size help

version: ## Show current workspace version
	@echo "v$(VERSION)"

size: build-release ## Show release binary size
	@echo "Release binary sizes:"
	@find target/release -maxdepth 1 -type f -executable 2>/dev/null | xargs ls -lh 2>/dev/null || echo "  (no binaries — this is a library crate)"
	@echo ""
	@echo "Example binary sizes (if built):"
	@find target/release/examples -maxdepth 1 -type f -executable 2>/dev/null | head -5 | xargs ls -lh 2>/dev/null || echo "  Run 'cargo build -p rust-tg-bot --examples --release' first"

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-24s\033[0m %s\n", $$1, $$2}'
