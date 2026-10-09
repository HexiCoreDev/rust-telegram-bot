#!/usr/bin/env python3
"""Method-level parity check: docs/telegram-bot-api.md vs crates/telegram-bot-raw.

For every method documented in the Bot API spec we locate the Rust entry point
(`Bot::<snake_name>` in bot_builders*.rs) and compare:
  * required parameters  -> positional args of the builder constructor
  * optional parameters  -> fields initialised to `None` in the returned builder
  * setters              -> `pub fn <param>` inside the builder's impl block

Reports: missing methods, missing/extra required params, missing optional params,
required/optional disagreement, and missing setters.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / "docs/telegram-bot-api.md"
BUILDER_FILES = sorted((ROOT / "crates/telegram-bot-raw/src").glob("bot_builders*.rs"))


# ------------------------------------------------------------------ spec side

def parse_spec(path: Path) -> tuple[dict[str, dict], dict[str, list[str]]]:
    text = path.read_text(encoding="utf-8")
    lines = text.split("\n")

    methods: dict[str, dict] = {}
    types: dict[str, list[str]] = {}

    i = 0
    cur_name = None
    cur_kind = None
    while i < len(lines):
        line = lines[i]
        m = re.match(r"^#### (.+)$", line)
        if m:
            cur_name = m.group(1).strip()
            cur_kind = None
        elif line.startswith("| Parameter | Type | Required"):
            cur_kind = "method"
            methods.setdefault(cur_name, {"required": [], "optional": []})
        elif line.startswith("| Field | Type | Description"):
            cur_kind = "type"
            types.setdefault(cur_name, [])
        elif line.startswith("|") and cur_kind and cur_name:
            cells = [c.strip() for c in line.strip().strip("|").split("|")]
            if len(cells) >= 3 and cells[0] not in ("Parameter", "Field", "---"):
                if cur_kind == "method":
                    bucket = "required" if cells[2].lower() == "yes" else "optional"
                    methods[cur_name][bucket].append(cells[0])
                else:
                    types[cur_name].append(cells[0])
        i += 1

    # de-duplicate: later sections (Updating messages, Stickers, ...) repeat methods
    for name, data in methods.items():
        for k in ("required", "optional"):
            seen, out = set(), []
            for p in data[k]:
                if p not in seen:
                    seen.add(p)
                    out.append(p)
            data[k] = out
    return methods, types


# ------------------------------------------------------------------ rust side

def camel_to_snake(name: str) -> str:
    s = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    return s.lower()


def load_rust() -> str:
    return "\n".join(p.read_text(encoding="utf-8") for p in BUILDER_FILES)


def match_brace(src: str, open_idx: int) -> int:
    depth = 0
    for k in range(open_idx, len(src)):
        if src[k] == "{":
            depth += 1
        elif src[k] == "}":
            depth -= 1
            if depth == 0:
                return k
    return -1


def parse_constructor(src: str, snake: str) -> tuple[list[str], list[str]] | None:
    """Return (positional_required, optional_None_fields) for Bot::<snake>()."""
    m = re.search(r"pub fn " + re.escape(snake) + r"\s*\(", src)
    if not m:
        return None
    sig_open = src.index("(", m.end() - 1)
    sig_close = match_paren(src, sig_open)
    sig = src[sig_open + 1 : sig_close]

    args = [a.strip() for a in split_top(sig) if a.strip()]
    args = [a for a in args if a not in ("&self", "self", "&'a self")]
    # keep only the short binding names: "chat_id: impl Into<ChatId>" -> "chat_id"
    required = []
    for a in args:
        head = a.split(":")[0].strip().lstrip("&").strip()
        if re.fullmatch(r"[a-z_][a-z0-9_]*", head):
            required.append(head)
        else:
            required.append(a)

    body_open = src.index("{", sig_close)
    body_close = match_brace(src, body_open)
    body = src[body_open:body_close]
    optional = re.findall(r"^\s*([a-z_][a-z0-9_]*)\s*:\s*None\s*,", body, re.M)
    return required, optional


def match_paren(src: str, open_idx: int) -> int:
    depth = 0
    for k in range(open_idx, len(src)):
        if src[k] == "(":
            depth += 1
        elif src[k] == ")":
            depth -= 1
            if depth == 0:
                return k
    return -1


def split_top(s: str) -> list[str]:
    out, depth, buf = [], 0, ""
    for ch in s:
        if ch in "<([{":
            depth += 1
        elif ch in ">)]}":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(buf)
            buf = ""
        else:
            buf += ch
    if buf.strip():
        out.append(buf)
    return out


def parse_setters(src: str, builder: str) -> set[str]:
    """`pub fn` names inside `impl<'a> Builder<'a> { ... }`."""
    m = re.search(r"impl(?:<[^>]*>)?\s+" + re.escape(builder) + r"(?:<[^>]*>)?\s*\{", src)
    if not m:
        return set()
    open_idx = src.index("{", m.end() - 1)
    close_idx = match_brace(src, open_idx)
    body = src[open_idx:close_idx]
    return set(re.findall(r"pub fn ([a-z_][a-z0-9_]*)\s*\(", body))


# ------------------------------------------------------------------ main

def main() -> None:
    methods, types = parse_spec(DOC)
    src = load_rust()

    missing_method = []
    missing_required = []
    missing_optional = []
    required_disagree = []
    missing_setter = []

    for name, data in sorted(methods.items()):
        snake = camel_to_snake(name)
        parsed = parse_constructor(src, snake)
        if parsed is None:
            missing_method.append(name)
            continue
        rust_req, rust_opt = parsed

        spec_req = set(data["required"])
        spec_opt = set(data["optional"])
        rust_req_s = set(rust_req)
        rust_opt_s = set(rust_opt)

        # spec-required params missing from the Rust constructor
        miss_req = sorted(spec_req - rust_req_s - rust_opt_s)
        if miss_req:
            missing_required.append((name, miss_req))
        # spec-optional params entirely absent
        miss_opt = sorted(spec_opt - rust_opt_s - rust_req_s)
        if miss_opt:
            missing_optional.append((name, miss_opt))
        # required in spec but modelled as Option in Rust, and vice versa
        req_as_opt = sorted(spec_req & rust_opt_s - rust_req_s)
        opt_as_req = sorted(spec_opt & rust_req_s - rust_opt_s)
        if req_as_opt or opt_as_req:
            required_disagree.append((name, req_as_opt, opt_as_req))

        builder = re.search(
            r"->\s*(" + re.escape(name[0].upper() + name[1:]) + r"Builder)\b", src[max(0, src.find("pub fn " + snake) - 400) : src.find("pub fn " + snake) + 600]
        )
        if builder:
            setters = parse_setters(src, builder.group(1))
            miss_set = sorted((spec_opt - rust_opt_s) & setters)
            if miss_set:
                missing_setter.append((name, miss_set))

    print(f"spec methods parsed      : {len(methods)}")
    print(f"spec types parsed        : {len(types)}")
    print()
    print(f"[1] methods with NO Rust entry point : {len(missing_method)}")
    for n in missing_method[:40]:
        print(f"      {n}")
    print(f"[2] spec-required params absent from constructor : {len(missing_required)}")
    for n, p in missing_required[:40]:
        print(f"      {n}: {p}")
    print(f"[3] spec-optional params absent from builder    : {len(missing_optional)}")
    for n, p in missing_optional[:60]:
        print(f"      {n}: {p}")
    print(f"[4] required/optional disagreement  : {len(required_disagree)}")
    for n, a, b in required_disagree[:40]:
        print(f"      {n}: spec-required modelled Option {a} | spec-optional modelled required {b}")
    print(f"[5] builder setters missing : {len(missing_setter)}")
    for n, p in missing_setter[:30]:
        print(f"      {n}: {p}")


if __name__ == "__main__":
    main()
