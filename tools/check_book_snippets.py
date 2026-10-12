#!/usr/bin/env python3
"""Compile the book's Rust snippets against the real crates.

`mdbook test` cannot do this and the reason is structural, not a matter of
configuration:

  * mdbook runs `rustdoc --test` on a generated crate and exposes no way to
    pass `--extern`, so `use rust_tg_bot::...` never resolves.
  * The obvious workaround does not exist either. rustdoc applies RUSTDOCFLAGS
    when *documenting* but not when *compiling doctests*, verified directly:

        RUSTDOCFLAGS="--extern rust_tg_bot=... -L dependency=..." \\
            rustdoc --test --edition 2021 probe.rs     # fails

        rustdoc --test --edition 2021 \\
            --extern rust_tg_bot=... -L dependency=... probe.rs   # passes

  * The book is deliberately fragmentary: most pages show an excerpt from the
    middle of a handler (`let chat_id = ...;`) that continues a binding
    introduced by an earlier block on the same page. Those cannot compile
    standalone, by design, and pretending otherwise would mean rewriting every
    snippet into a whole program.

So this walks the same blocks mdbook would, wraps each in a scratch crate that
depends on the workspace, and reports per page. Blocks the book marks
`rust,ignore` are skipped; so are fragments, which are reported as skipped
rather than failed so real breakage stays visible.

Usage:
    tools/check_book_snippets.py                 # every page
    tools/check_book_snippets.py conversations   # pages matching a substring
    tools/check_book_snippets.py --verbose       # show compiler output
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BOOK = ROOT / "book/src"
FACADE = ROOT / "crates/telegram-bot"

# One shared target dir: the scratch crates are disposable, the compiled
# dependency tree is not. Without this every snippet rebuilds all of `full`.
TARGET = Path(
    os.environ.get("BOOK_SNIPPET_TARGET", Path(tempfile.gettempdir()) / "book-snippet-target")
)

CARGO_TMPL = """[package]
name = "snippetcheck"
version = "0.0.0"
edition = "2021"

[dependencies]
rust-tg-bot = { path = "%s", features = ["full", "persistence-json", "webhooks", "webhooks-tls", "macros"] }
# The BotCommands derive expands to `rust_tg_bot_raw::...` paths, so the raw
# crate must be a direct dependency for the macro to resolve.
rust-tg-bot-raw = { path = "%s" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
axum = "0.8"
sqlx = { version = "0.8", default-features = false }
rusqlite = "0.32"
""" % (FACADE, ROOT / "crates" / "telegram-bot-raw")

# Bindings the book's prose supplies. A snippet written as a handler body
# references `update` and `context`; declaring them lets the snippet be checked
# on its own terms.
BINDINGS = """#[allow(unused)]
async fn run(
    update: std::sync::Arc<rust_tg_bot::ext::prelude::Update>,
    context: rust_tg_bot::ext::prelude::CallbackContext,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
"""

PRELUDE = """#![allow(unused, dead_code, unused_imports)]
use std::collections::HashMap;
use std::time::Duration;
// The prelude already re-exports HandlerResult, Context, Arc, Update, the
// handler types and the filters. Importing them a second time by explicit path
// shadows the glob and produces spurious type mismatches.
use rust_tg_bot::ext::prelude::*;
// Types the book imports explicitly, which the prelude does not re-export.
// Without these, a snippet that correctly shows its own `use` line and one
// that omits it produce the identical error, and the tool cannot tell a real
// documentation gap from a harness limitation.
use rust_tg_bot::raw::types::payment::labeled_price::LabeledPrice;
use rust_tg_bot::raw::types::inline::input_text_message_content::InputTextMessageContent;
use rust_tg_bot::raw::types::inline::inline_keyboard_button::InlineKeyboardButton;
use rust_tg_bot::raw::types::inline::inline_keyboard_markup::InlineKeyboardMarkup;
use rust_tg_bot::raw::types::files::input_media::InputMedia;
use rust_tg_bot::raw::types::keyboard_button::KeyboardButton;
use rust_tg_bot::raw::types::reply_keyboard_markup::ReplyKeyboardMarkup;
use rust_tg_bot::raw::types::input_rich_message::InputRichMessage;
use rust_tg_bot::ext::persistence::sqlite::SqlitePersistence;
use rust_tg_bot::ext::persistence::redis::RedisPersistence;
use rust_tg_bot::ext::job_queue::{JobCallbackFn, JobContext, JobQueue};
use rust_tg_bot::raw::types::message_entity::MessageEntity;
use rust_tg_bot::raw::types::chat_member::ChatMember;
use rust_tg_bot::ext::persistence::postgres::PostgresPersistence;
"""

FENCE = re.compile(r"^```(rust[^\n]*)$", re.M)

# Types the Bot API defines. A snippet naming one of these is missing an import
# the reader would need -- a real gap -- whereas an unknown name is a fragment
# continuing an earlier block.
BOT_API_TYPES = {
    "LabeledPrice", "ShippingOption", "Invoice", "SuccessfulPayment",
    "LabeledPrice", "OrderInfo", "ShippingAddress", "InputMedia",
    "InputMediaPhoto", "InputMediaVideo", "InputPaidMedia",
    "PollOption", "MessageEntity", "InlineKeyboardMarkup", "ReplyKeyboardMarkup",
    "KeyboardButton", "MenuButton", "BotCommand", "BotCommandScope",
    "ReactionType", "ChatAdministratorRights", "InputFile", "WebhookInfo",
    "Sticker", "GameHighScore", "TransactionPartner", "RevenueWithdrawalState",
}


def blocks(text: str) -> list[tuple[int, str, bool]]:
    """(fence line, body, is_plain_rust) for each Rust block."""
    out = []
    for m in FENCE.finditer(text):
        start = m.end()
        end = text.find("\n```", start)
        if end == -1:
            continue
        out.append((text[: m.start()].count("\n") + 1, text[start + 1 : end], m.group(1).strip() == "rust"))
    return out


def check_one(body: str, workdir: Path) -> tuple[bool, str]:
    """Compile a snippet, trying a body-fragment wrapper before item scope."""
    (workdir / "src").mkdir(parents=True, exist_ok=True)
    (workdir / "Cargo.toml").write_text(CARGO_TMPL)

    # A snippet that already declares items must be compiled at module scope;
    # wrapping it in `async fn run()` would nest the declarations and leave the
    # wrapper's trailing `Ok(())` inside the snippet's own function.
    is_items = re.search(r"^\s*(pub\s+)?(async\s+)?(fn|struct|enum|impl|trait|mod)\b", body, re.M)
    # A snippet that already declares `main` is a whole program; wrapping it
    # would define `main` twice (E0428).
    has_main = re.search(r"^\s*(pub\s+)?(async\s+)?fn main\b", body, re.M)
    if has_main:
        attempts = [PRELUDE + body + "\n"]
    elif is_items:
        attempts = [PRELUDE + body + "\nfn main() {}\n", PRELUDE + BINDINGS + body + "\nOk(())\n}\nfn main() {}\n"]
    else:
        attempts = [PRELUDE + BINDINGS + body + "\nOk(())\n}\nfn main() {}\n", PRELUDE + body + "\nfn main() {}\n"]
    errors = []
    for attempt in attempts:
        (workdir / "src/main.rs").write_text(attempt)
        r = subprocess.run(
            ["cargo", "check", "--offline", "--quiet", "--jobs", "4"],
            cwd=workdir,
            capture_output=True,
            text=True,
            env={**os.environ, "CARGO_TARGET_DIR": str(TARGET)},
        )
        if r.returncode == 0:
            return True, ""
        errors.append(r.stderr or r.stdout)

    # Classify on EVERY attempt, not just the last one.
    #
    # Keeping only the final error was a real bug: for
    # `context.reply_text(&update, 42).await?;` the *first* attempt -- wrapped
    # in `async fn run(update, context)` -- fails with E0308, the genuine type
    # error. The second attempt, at module scope, then fails with a parse
    # error about `await` outside async, and that parse error is what got
    # reported. A fragment-shaped parse error from a fallback attempt masked the
    # real diagnosis from the attempt that actually had the right context.
    return False, "\n".join(errors)


# Bindings the prose introduces; a statement rooted at one of these is a body
# fragment rather than a real defect.
_RECEIVER = re.compile(r"^(context|update|app|bot|state|ctx)\b")

FRAGMENT_START = re.compile(
    r"^\s*(?:"
    r"let\b"                    # continues a function body
    r"|[{}]"                     # a bare block or closing brace
    r"|\.[A-Za-z_]"             # a continued method chain
    r"|\"[^\"]*\"\s*=>"       # a match arm
    r"|[A-Za-z_][\w:]*\s*=>"    # a binding pattern arm
    r"|[A-Za-z_][\w:]*\s*\?\s*=" # a struct field continuation
    r")",
    re.M,
)


def _bare_expression(line: str) -> bool:
    """True when a line is an expression or continued statement, not an item.

    An item always ends in `{`, `;`, or `=`. Everything else -- a call, a
    builder chain, a `&`/`|` filter expression -- belongs inside a body.
    """
    return not line.endswith(("{", ";", "=", "}"))


# Error codes that are ALWAYS a real defect, never a fragment.
#
# A fragment cannot compile because it is incomplete. Incompleteness shows up
# as a parse error, or as an unresolved *local* name -- never as a type
# mismatch, a missing trait impl, or a call to a method that does not exist.
# Those are the bugs this tool exists to find, so they are checked before any
# fragment reasoning runs.
HARD_ERRORS = (
    "E0308",  # mismatched types
    "E0277",  # trait bound not satisfied
    "E0599",  # no method named
    "E0433",  # unresolved crate/module
    "E0432",  # unresolved import
    "E0412",  # cannot find type
    "E0603",  # private item
    "E0369",  # binary operation cannot be applied
)

# A name the Bot API defines. If the book references one of these and rustc
# cannot find it, the reader would hit the same error -- it is a missing import
# in the book, not a fragment continuing earlier prose.
BOT_API_NAMES = {
    "LabeledPrice", "ShippingOption", "Invoice", "SuccessfulPayment",
    "InputMedia", "InputMediaPhoto", "InputMediaVideo", "InputPaidMedia",
    "PollOption", "MessageEntity", "InlineKeyboardMarkup",
    "ReplyKeyboardMarkup", "KeyboardButton", "MenuButton", "BotCommand",
    "BotCommandScope", "ReactionType", "ChatAdministratorRights",
    "InputFile", "WebhookInfo", "Sticker", "GameHighScore",
    "TransactionPartner", "RevenueWithdrawalState", "ShippingAddress",
}

# Shapes that cannot begin a compilation unit.
FRAGMENT_SHAPES = (
    "// ...",                 # the book's own elision marker
)


def has_hard_error(err: str) -> str | None:
    """Return the first hard error code found, or None."""
    for code in HARD_ERRORS:
        if f"error[{code}]" in err:
            return code
    return None


def is_elided(err: str) -> bool:
    """True when the only hard error is a `// ...` body standing in for code.

    The book writes handlers as `async fn h(..) -> HandlerResult { // ... }`.
    With the body elided the fn returns `()`, so rustc reports a type mismatch
    that is a property of the ellipsis, not of the snippet. Distinguish it from a
    real mismatch by the diagnostic shape: "found unit type `()`" against a
    `Result<` expectation, rather than a concrete type mismatch.
    """
    return ("found unit type `()`" in err) and ("expected `Result<" in err)


def is_fragment(body: str, err: str, page_text: str = "") -> bool:
    """True only when the snippet is provably incomplete rather than wrong.

    Order matters. A hard error -- a type mismatch, a missing method, an
    unresolved crate -- settles it: the snippet is wrong, not incomplete, and
    no amount of shape-matching may talk the tool out of reporting it. That is
    the failure mode that matters: a checker that swallows real errors is worse
    than no checker.

    Only after ruling those out do we ask whether the snippet is a fragment.
    """
    hard = has_hard_error(err)
    if hard:
        # A `// ...` elision can produce the only hard-looking error there is.
        if is_elided(err) and err.count("mismatched types") == 1 and "// ..." in body:
            return True
        return False

    # An unresolved name that this page defined earlier is a fragment. The
    # conversations guide defines `ConvStore` three blocks before using it; the
    # nested-conversations guide defines `TopState`, `MemberState`, and friends
    # the same way. A page scan distinguishes that from a name nobody defined.
    defined_here = set(re.findall(r"^(?:type|struct|enum|trait) (\w+)", page_text, re.M))
    for name in re.findall(r"cannot find (?:type|value|function|macro) `(\w+)`", err):
        if name in defined_here:
            return True
        if name in BOT_API_NAMES:
            return False

    for marker in FRAGMENT_SHAPES:
        if marker in body:
            return True

    # Unresolved *locals* come from an earlier block on the page: the migration
    # guide's `// Rust` halves reference bindings declared above them, and a
    # later snippet continues `chat_id`/`jq` from the block before it. Those are
    # fragments.
    #
    # An unresolved *type* is different. A type is either a library type the
    # reader must import, or one this page defined in an earlier block. Either
    # way the snippet is not self-contained, but unlike a local there is no way
    # to tell "continues the page" from "the book forgot the import" without
    # reading it -- so it is reported and reviewed, not silently skipped.
    if re.search(r"cannot find (?:value|function|macro|attribute) `\w+`", err):
        return True

    # A bare match arm, lone block, or continued chain cannot start a unit.
    first = next(
        (l for l in body.splitlines() if l.strip() and not l.strip().startswith("//")),
        "",
    )
    if re.match(r'^\s*(\{|\}|[.\w]*"\s*=>)', first):
        return True
    if first.rstrip().endswith(("(", ".")):
        return True

    # Nothing proves this is a fragment, so report it. Silence is the one
    # outcome that cannot be defended.
    return False


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("pages", nargs="*", help="page substrings to check")
    ap.add_argument("--verbose", action="store_true", help="print compiler output")
    args = ap.parse_args()

    pages = sorted(BOOK.rglob("*.md"))
    if args.pages:
        pages = [p for p in pages if any(a in str(p) for a in args.pages)]
    if not pages:
        print("no matching pages", file=sys.stderr)
        return 2

    total = compiled = fragments = 0
    failures: list[tuple[str, int, str]] = []

    for page in pages:
        rel = page.relative_to(BOOK)
        todo = [(ln, b) for ln, b, plain in blocks(page.read_text()) if plain]
        if not todo:
            continue
        page_ok = page_frag = 0
        with tempfile.TemporaryDirectory() as td:
            workdir = Path(td) / "snippet"
            page_text = page.read_text()
            for ln, body in todo:
                total += 1
                ok, err = check_one(body, workdir)
                if ok:
                    compiled += 1
                    page_ok += 1
                elif is_fragment(body, err, page_text):
                    fragments += 1
                    page_frag += 1
                else:
                    failures.append((str(rel), ln, err))
        print(f"[{'OK ' if not any(f[0] == str(rel) for f in failures) else 'FAIL'}] {rel}: "
              f"{page_ok} compile, {page_frag} fragment")

    print(f"\nblocks: {total}   compile: {compiled}   fragments: {fragments}   failed: {len(failures)}")
    for rel, ln, err in failures:
        first = next((l for l in err.splitlines() if l.startswith("error")), "compile failed")
        print(f"  {rel}:{ln}: {first[:140]}")
        if args.verbose:
            print("\n".join(err.splitlines()[:25]))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
