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
    attempts = (
        [PRELUDE + body + "\nfn main() {}\n", PRELUDE + BINDINGS + body + "\nOk(())\n}\nfn main() {}\n"]
        if is_items
        else [PRELUDE + BINDINGS + body + "\nOk(())\n}\nfn main() {}\n", PRELUDE + body + "\nfn main() {}\n"]
    )
    last = ""
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
        last = r.stderr or r.stdout
    return False, last


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


def is_fragment(body: str, err: str) -> bool:
    """True when the snippet is structurally a fragment, not real breakage.

    Decided from the snippet's *shape*, never from the error text. Matching on
    error strings looked appealing and was wrong: an injected
    `reply_text(&update, 42)` -- a genuine type error -- got absorbed as a
    fragment, because the fragment wrapper's second attempt also failed and its
    message happened to resemble a fragment failure. A snippet that starts with
    a real statement and contains a real error will always fail to compile.

    A fragment is one that cannot begin a compilation unit:
      * starts mid-body   -- `let x = ...;`, a bare `{`, a continued `.chain()`
      * is a match arm    -- `"back" => { ... }`
      * elides its body   -- the book's `// ...` marker
    """
    if "// ..." in body:
        return True
    # The first *statement* matters, not the first line: a snippet may open with
    # a comment before continuing mid-body.
    first = next(
        (l for l in body.splitlines() if l.strip() and not l.strip().startswith("//")),
        "",
    )
    # Anything that is not an item (fn/struct/enum/impl/use/const/static) and
    # does not end in `;`, `{`, or `}` at top level is a bare expression or a
    # continued statement -- the dominant fragment shape. The migration guide is
    # built from exactly these: `InlineKeyboardButton::callback("Click", "1")`,
    # `TEXT() & !COMMAND()`, `app.add_handler(..).await;`.
    stripped = first.strip()
    if stripped and not re.match(r"^(pub\s+)?(async\s+)?(fn|struct|enum|impl|trait|mod|use|const|static|type)\b", stripped):
        if FRAGMENT_START.match(first) or _bare_expression(stripped):
            return True

    # A statement whose receiver is a local introduced by an earlier block --
    # `app.add_handler(..).await;`, `app.bot().delete_webhook(false).await?` --
    # is a fragment even though it ends in `;`. The compiler named the missing
    # bindings, so use them rather than guessing from the text.
    for name in re.findall(r"cannot find value `(\w+)`", err):
        if name not in BOT_API_TYPES:
            return True

    # A statement opening with a lowercase path -- `app.add_handler(..)`,
    # `app.bot().delete_webhook(..)` -- is a handler-body fragment. Rust items
    # are UpperCamelCase by convention, so this does not swallow real code. It
    # matters because these fail at parse time, before name resolution, so
    # there is no "cannot find value" to read.
    # The receiver of a handler-body statement is one of the framework bindings the
    # book's own prose introduces: `context`, `update`, `app`, `bot`, `state`.
    # Those are fragments even when the line ends in `;`. Anything else is real
    # code and a failure.
    if re.match(r"^[a-z_]\w*(\.\w+|\s*\()", stripped) and _RECEIVER.match(stripped):
        return True

    # A `use` block followed by handler-body statements is still a fragment: it
    # has imports, then continues mid-function. Check the first *non-use* line.
    rest = [
        l for l in body.splitlines()
        if l.strip() and not l.strip().startswith("//") and not l.strip().startswith("use ")
    ]
    # Any line that continues a body makes the whole snippet a fragment, whether
    # or not it opened with a `use`.
    for line in rest:
        if FRAGMENT_START.match(line) or (
            not re.match(r"^(pub\s+)?(async\s+)?(fn|struct|enum|impl|trait|mod|use|const|static|type)\b", line.strip())
            and _bare_expression(line.strip())
        ):
            return True

    # A snippet whose first line is a bare expression -- `context.bot()`, with
    # the call continuing on following lines -- is a fragment too: it is a
    # statement that belongs inside a handler body.
    if first.rstrip().endswith(("(", ".")) or re.match(r"^[\w:]+(\.\w+\(\))+$", first.strip()):
        return True

    # A snippet that names a local introduced by an earlier block -- the
    # migration guide's `// Rust` halves reference the bindings from the
    # `// Python` block above them -- cannot compile alone. A name from the Bot
    # API is different: that is a missing import a reader would hit, so it
    # stays a failure.
    for name in re.findall(r"cannot find value `(\w+)`", err):
        if name not in BOT_API_TYPES:
            return True
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
            for ln, body in todo:
                total += 1
                ok, err = check_one(body, workdir)
                if ok:
                    compiled += 1
                    page_ok += 1
                elif is_fragment(body, err):
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
