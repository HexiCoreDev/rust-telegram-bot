#!/usr/bin/env python3
"""Type-check the book's ```rust blocks against the real crates.

`mdbook test` cannot do this: its rustdoc backend runs at edition 2015 and has
no linkage to workspace crates, so every block fails for reasons unrelated to
its content. This wraps each block in a generated crate instead, so a block
that names a non-existent method or has the wrong arity fails to compile.

Blocks that are deliberate fragments are skipped -- the book marks these with
an info string (`rust,ignore`) or they sit in a page section we have not
audited. Report per page so drift is visible in review.
"""

from __future__ import annotations

import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path("/home/iam/devcode/projects/experiments/portfolio/rust-telegram-bot")
BOOK = ROOT / "book/src"
FACADE = ROOT / "crates/telegram-bot"

# Scratch crates are built here; override if /tmp is tight on space.
SCRATCH = os.environ.get("BOOK_CHECK_SCRATCH", tempfile.gettempdir())

CARGO_TMPL = """[package]
name = "bookcheck"
version = "0.0.0"
edition = "2021"

[dependencies]
rust-tg-bot = { path = "%s", features = ["full"] }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
""" % FACADE

# The book is deliberately fragmentary: most pages show an excerpt from the
# middle of a handler (`let chat_id = ...;`) that cannot compile standalone.
# Those pages are audited by reading, and any specific claim gets compiled in a
# scratch crate when it is changed.
#
# PAGES_CHECKED names pages where a whole ```rust block must compile on its own.
# Add a page here only once every block on it is self-contained.
#
# None yet: every page currently carries at least one block that continues the
# manual pattern introduced further down (a `ConvStore` defined in a previous
# block, a `#[tokio::main]` block that is already a program). Use --snippet to
# machine-check an individual block when changing a claim:
#
#   uv run tools/tg-api-docs/check_book.py --snippet book/src/guides/conversations.md --line 12
PAGES_CHECKED: set[str] = set()

ITEM = re.compile(r"^```(rust[^\n]*)$", re.M)


def blocks(text: str) -> list[tuple[int, str, bool]]:
    """(opening-fence line, body, is_plain_rust) for every rust block.

    The reported line is the opening fence, so `--line` takes that number.
    """
    out = []
    for m in ITEM.finditer(text):
        tag = m.group(1).strip()
        start = m.end()
        end = text.index("\n```", start)
        body = text[start + 1 : end]
        out.append((text[: m.start()].count("\n") + 1, body, tag == "rust"))
    return out


def cargo_clean(workdir: Path) -> None:
    if (workdir / "Cargo.toml").exists():
        subprocess.run(
            ["cargo", "clean", "--offline"], cwd=workdir, capture_output=True
        )


def check_one(body: str, workdir: Path) -> tuple[bool, str]:
    """Compile `body` inside `async fn run()`, falling back to item scope.

    Book snippets are usually handler-body fragments (`let x = ...;`), which are
    only valid inside a function. Wrapping first handles those; if the snippet
    is already items (imports, fns, derives), the wrapper fails to parse and we
    retry at module scope.
    """
    (workdir / "src").mkdir(parents=True, exist_ok=True)
    (workdir / "Cargo.toml").write_text(CARGO_TMPL)

    # Handler-body fragments reference `update` and `context`, which the book's
    # prose supplies. Declare them so the fragment can be checked.
    bindings = (
        "#[allow(unused)]\n"
        "async fn run(\n"
        "    update: std::sync::Arc<Update>,\n"
        "    context: CallbackContext,\n"
        ") -> Result<(), Box<dyn std::error::Error + Send + Sync>> {\n"
    )
    prelude = (
        "use std::collections::HashMap;\n"
        "use std::hash::Hash;\n"
        "use std::time::Duration;\n"
        "use rust_tg_bot::ext::prelude::*;\n"
        "use rust_tg_bot::ext::handlers::base::{FnHandler, Handler, HandlerResult};\n"
        "use rust_tg_bot::ext::handlers::message::MessageHandler;\n"
        "use rust_tg_bot::ext::handlers::command::CommandHandler;\n"
        "use rust_tg_bot::ext::handlers::callback_query::CallbackQueryHandler;\n"
        "use rust_tg_bot::ext::handlers::inline_query::InlineQueryHandler;\n"
        "use rust_tg_bot::ext::handlers::conversation::{\n"
        "    ConversationHandler, ConversationResult, ConversationStepHandler,\n"
        "};\n"
        "use rust_tg_bot::ext::application::{Application, HandlerError};\n"
        "use rust_tg_bot::ext::builder::ApplicationBuilder;\n"
        "use rust_tg_bot::ext::context::{CallbackContext, DataReadGuard, DataWriteGuard};\n"
    )
    attempts = [
        prelude + bindings + body + "\nOk(())\n}\nfn main() {}\n",
        prelude + body + "\nfn main() {}\n",
        "use rust_tg_bot::ext::prelude::*;\n" + body + "\nfn main() {}\n",
    ]
    last = ""
    for attempt in attempts:
        (workdir / "src/main.rs").write_text(attempt)
        r = subprocess.run(
            ["cargo", "check", "--offline", "--jobs", "4", "--quiet"],
            cwd=workdir,
            capture_output=True,
            text=True,
        )
        if r.returncode == 0:
            return True, ""
        last = r.stderr or r.stdout
    return False, last
    r = subprocess.run(
        ["cargo", "check", "--offline", "--jobs", "4", "--quiet"],
        cwd=workdir,
        capture_output=True,
        text=True,
    )
    return r.returncode == 0, (r.stderr or r.stdout)


def main() -> None:
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    only = args[0] if args else None
    # `--snippet FILE` compiles one block by line number, for auditing a single
    # claim on a fragmentary page:
    #   uv run tools/tg-api-docs/check_book.py --snippet book/src/... --line 123
    if "--snippet" in sys.argv:
        i = sys.argv.index("--snippet")
        target = Path(sys.argv[i + 1])
        line = int(sys.argv[sys.argv.index("--line") + 1]) if "--line" in sys.argv else None
        text = target.read_text(encoding="utf-8")
        for ln, body, plain in blocks(text):
            if plain and (line is None or ln == line):
                with tempfile.TemporaryDirectory(dir=SCRATCH) as td:
                    ok, err = check_one(body, Path(td) / "page")
                    cargo_clean(Path(td) / "page")
                print(f"line {ln}: {'COMPILES' if ok else 'FAILS'}")
                if not ok:
                    print("\n".join(err.splitlines()[:20]))
                sys.exit(0 if ok else 1)
        sys.exit(f"no plain ```rust block at line {line} in {target}")

    pages = sorted(BOOK.rglob("*.md"))
    if only:
        pages = [p for p in pages if only in str(p)]

    total = passed = failed = 0
    for page in pages:
        rel = page.relative_to(BOOK)
        if PAGES_CHECKED and str(rel) not in PAGES_CHECKED:
            continue
        text = page.read_text(encoding="utf-8")
        todo = [(ln, b, plain) for ln, b, plain in blocks(text) if plain]
        if not todo:
            continue
        results = []
        # One scratch dir per page, not per block: each `cargo check` otherwise
        # recompiles the whole dependency tree and exhausts the disk quota.
        with tempfile.TemporaryDirectory(dir=SCRATCH) as td:
            page_dir = Path(td) / "page"
            for i, (ln, body, _) in enumerate(todo):
                ok, err = check_one(body, page_dir)
                results.append((ln, ok, err))
            cargo_clean(page_dir)
        bad = [(ln, err) for ln, ok, err in results if not ok]
        total += len(results)
        passed += len(results) - len(bad)
        failed += len(bad)
        status = "OK " if not bad else "FAIL"
        print(f"[{status}] {rel}: {len(results) - len(bad)}/{len(results)} blocks compile")
        for ln, err in bad:
            first = next(
                (l for l in err.splitlines() if l.startswith("error")), "compile failed"
            )
            print(f"        line {ln}: {first[:120]}")

    print(f"\nblocks: {total}   compile: {passed}   failed: {failed}")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
