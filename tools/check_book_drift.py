#!/usr/bin/env python3
"""Flag book snippets that have drifted away from the example they were copied from.

Most snippets in the book are deliberately partial -- they show one concept,
not a whole program -- and those are fine. The risk is the opposite case: a
snippet that was pasted from an example and has since been edited in the book
while the example moved on, or vice versa. A reader then copies code that no
longer matches anything that compiles, and nothing catches it.

This compares every ```rust block against every example and reports a snippet
only when it is *similar but not identical* to part of an example. High
similarity with a real difference is the signature of a stale copy; a snippet
that matches exactly, or barely matches at all, is left alone.

Run with --explain to see the differing lines.
"""

from __future__ import annotations

import argparse
import difflib
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BOOK = ROOT / "book/src"
EXAMPLES = ROOT / "crates/telegram-bot/examples"

# Similarity at or above which we call two blocks "the same thing".
SIMILAR_ENOUGH = 0.60
# ...but below which they are just unrelated code that happens to share words.
ALSO_SIMILAR = 0.35
# Ignore short lines: `});` and `Ok(())` match everything.
MIN_LINE_LEN = 12

# A run of consecutive identical lines that counts as "copied".
MIN_SHARED_RUN = 3
# At least this many copied lines before a snippet counts as a copy at all.
MIN_SHARED_LINES = 8
# Fraction of the snippet that must be copied for this to be a copy rather than
# a coincidence. Snippets illustrating one idea sit well below this.
COVERAGE_MIN = 0.60
# Edited lines inside a copied region before we call it drift. One deliberate
# simplification is a normal editorial choice; several means a copy has fallen
# behind.
MIN_DRIFTED_LINES = 2


def blocks(text: str) -> list[str]:
    return re.findall(r"```rust\n(.*?)```", text, re.S)


def lines_of(block: str) -> list[str]:
    return [
        l.strip()
        for l in block.splitlines()
        if len(l.strip()) >= MIN_LINE_LEN and not l.strip().startswith("//")
    ]


def analyse(snippet: list[str], example: list[str]) -> tuple[float, int, list[str]] | None:
    """Describe how much of a snippet is copied from an example.

    Returns (coverage, drifted_lines, drifted_text), or None when the snippet is
    not a copy at all.

    Drift is modelled as: most of the snippet's lines exist verbatim in the
    example (a copy), yet some lines inside that copied region do not (someone
    edited one side). A snippet that either matches cleanly or shares almost
    nothing is not drift, and is ignored.
    """
    if len(snippet) < 4:
        return None

    matcher = difflib.SequenceMatcher(None, snippet, example, autojunk=False)
    blocks_ = [b for b in matcher.get_matching_blocks() if b.size >= MIN_SHARED_RUN]
    matched = sum(b.size for b in blocks_)
    coverage = matched / len(snippet)
    if coverage < COVERAGE_MIN or matched < MIN_SHARED_LINES:
        return None

    # Everything between the first and last matched run: the copied region.
    start = blocks_[0].a
    end = max(b.a + b.size for b in blocks_)
    covered = {i for b in blocks_ for i in range(b.a, b.a + b.size)}
    drifted = [(i, snippet[i]) for i in range(start, end) if i not in covered]
    if len(drifted) < MIN_DRIFTED_LINES:
        return None
    return coverage, len(drifted), [t for _, t in drifted]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--explain", action="store_true", help="print differing lines")
    args = ap.parse_args()

    examples = {p.stem: lines_of(p.read_text()) for p in EXAMPLES.glob("*.rs")}
    findings: list[tuple[str, str, float, list[str]]] = []

    for page in sorted(BOOK.rglob("*.md")):
        rel = page.relative_to(BOOK)
        for snippet in blocks(page.read_text()):
            s_lines = lines_of(snippet)
            best = None
            for name, e_lines in examples.items():
                result = analyse(s_lines, e_lines)
                if result and (best is None or result[0] > best[1][0]):
                    best = (name, result)
            if best:
                name, (coverage, _, drifted) = best
                findings.append((str(rel), name, coverage, drifted))

    if not findings:
        print("no drifting snippets found")
        return 0

    print(f"{len(findings)} snippet(s) look like stale copies of an example:\n")
    for page, name, coverage, drifted in sorted(findings, key=lambda f: -f[2]):
        print(f"  {page}  ~  {name}  ({coverage:.0%} copied, {len(drifted)} line(s) differ)")
        if args.explain:
            for line in drifted[:8]:
                print(f"      {line}")
            print()

    print(
        "Fix by making the snippet match the example, or by trimming it so it\n"
        "demonstrates the concept rather than copying the program."
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
