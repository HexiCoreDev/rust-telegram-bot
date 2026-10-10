#!/usr/bin/env python3
"""Check that book/src/examples.md matches the examples that actually exist.

An index that lists the wrong set of examples is worse than no index, because
readers trust it. This verifies three things against the filesystem and
Cargo.toml rather than against prose:

  * every example file appears in the index
  * every example named in the index exists on disk
  * the features and environment variables listed for each match reality

Exits non-zero on any mismatch.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INDEX = ROOT / "book/src/examples.md"
EXAMPLES_DIR = ROOT / "crates/telegram-bot/examples"
CARGO_MANIFEST = ROOT / "crates/telegram-bot/Cargo.toml"


def required_features() -> dict[str, list[str]]:
    out: dict[str, list[str]] = {}
    for block in CARGO_MANIFEST.read_text().split("[[example]]")[1:]:
        name = re.search(r'name\s*=\s*"([^"]+)"', block)
        if not name:
            continue
        feats = re.search(r"required-features\s*=\s*\[([^\]]*)\]", block)
        out[name.group(1)] = re.findall(r'"([^"]+)"', feats.group(1)) if feats else []
    return out


def main() -> int:
    if not INDEX.exists():
        print(f"missing {INDEX}", file=sys.stderr)
        return 1

    text = INDEX.read_text()
    on_disk = {p.stem for p in EXAMPLES_DIR.glob("*.rs")}
    feats = required_features()

    # Rows look like: | [`name`](url) | ... | `feat` | `ENV` |
    listed: dict[str, tuple[list[str], list[str]]] = {}
    for line in text.splitlines():
        m = re.match(r"\|\s*\[`([a-z_0-9]+)`\]\(.*?\)\s*\|(.*)$", line)
        if not m:
            continue
        name, rest = m.group(1), m.group(2)
        cells = [c.strip() for c in rest.split("|")]
        feature_cell = cells[1] if len(cells) > 1 else ""
        env_cell = cells[2] if len(cells) > 2 else ""
        listed_features = re.findall(r"`([^`]+)`", feature_cell)
        listed_envs = re.findall(r"`([A-Z_]+)`", env_cell)
        listed[name] = (listed_features, listed_envs)

    problems: list[str] = []

    missing = sorted(on_disk - set(listed))
    if missing:
        problems.append(f"not listed in the index: {', '.join(missing)}")

    phantom = sorted(set(listed) - on_disk)
    if phantom:
        problems.append(f"listed but no such example file: {', '.join(phantom)}")

    for name in sorted(on_disk & set(listed)):
        src = (EXAMPLES_DIR / f"{name}.rs").read_text()
        actual_envs = set(re.findall(r'env::var\("([A-Z_]+)"\)', src))
        listed_envs = set(listed[name][1])

        # TELEGRAM_BOT_TOKEN is documented once at the top of the page.
        for env in sorted(actual_envs - listed_envs - {"TELEGRAM_BOT_TOKEN"}):
            problems.append(f"{name}: index omits required env var {env}")
        for env in sorted(listed_envs - actual_envs):
            problems.append(f"{name}: index lists {env}, which the example never reads")

        actual_feats = set(feats.get(name, []))
        listed_feats = set(listed[name][0])
        for feat in sorted(actual_feats - listed_feats):
            problems.append(f"{name}: index omits required feature {feat}")
        for feat in sorted(listed_feats - actual_feats):
            problems.append(f"{name}: index lists feature {feat}, which is not required")

    if problems:
        print(f"examples.md is out of sync ({len(problems)} problem(s)):")
        for p in problems:
            print(f"  - {p}")
        return 1

    print(f"examples.md matches all {len(on_disk)} examples")
    return 0


if __name__ == "__main__":
    sys.exit(main())
