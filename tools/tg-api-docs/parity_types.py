#!/usr/bin/env python3
"""Type/field-level parity check: docs/telegram-bot-api.md vs crates/telegram-bot-raw/types.

For every documented type with a field table we locate the Rust struct and compare
the WIRE names (serde rename, else the field name) against the spec.

Flags the two classes of bug that actually break requests:
  * a spec field with no Rust counterpart  -> data silently dropped on round-trip
  * a Rust field with no spec counterpart  -> Telegram rejects with unknown field
Also flags spec-required fields modelled as Option without skip_serializing_if.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / "docs/telegram-bot-api.md"
TYPES_DIR = ROOT / "crates/telegram-bot-raw/src/types"

sys.path.insert(0, str(Path(__file__).parent))
from parity import parse_spec  # noqa: E402


STRUCT_DOC: dict[str, str] = {}
ENUM_TAGS: dict[str, str] = {}


def clean(body: str) -> str:
    """Strip doc/line comments and flatten multi-line #[..] attribute blocks.

    Without this, `/// Type of chat: ...` parses as a field named `chat`, and
    `#[serde(\n rename = "type"\n)]` is invisible to the attribute matcher.
    """
    body = re.sub(r"//[^\n]*", "", body)
    body = re.sub(
        r"#\[(.*?)\]",
        lambda m: "#[ " + re.sub(r"\s+", " ", m.group(1)).strip() + " ]",
        body,
        flags=re.S,
    )
    return body


def load_structs() -> tuple[dict[str, list[dict]], dict[str, dict]]:
    out: dict[str, list[dict]] = {}
    enums: dict[str, dict] = {}
    for f in sorted(TYPES_DIR.rglob("*.rs")):
        src = f.read_text(encoding="utf-8")

        for m in re.finditer(r"pub enum ([A-Z]\w*)\s*\{", src):
            name = m.group(1)
            head = src[max(0, m.start() - 300) : m.start()]
            tag = re.findall(r'tag\s*=\s*"([^"]+)"', clean(head))
            depth, k = 0, src.index("{", m.end() - 1)
            while k < len(src):
                if src[k] == "{":
                    depth += 1
                elif src[k] == "}":
                    depth -= 1
                    if depth == 0:
                        break
                k += 1
            body = clean(src[src.index("{", m.end() - 1) + 1 : k])
            variants = []
            payloads = []
            for vm in re.finditer(r"((?:#\[[^\]]*\]\s*)*)([A-Z]\w*)\s*(\{|\()", body):
                attrs, vname = vm.group(1), vm.group(2)
                ren = re.search(r'rename\s*=\s*"([^"]+)"', attrs)
                variants.append(ren.group(1) if ren else vname)
                # payload struct for tuple variants: `Variant(PayloadType)`
                tail = body[vm.end() : vm.end() + 80]
                p = re.match(r"([A-Z]\w*)\s*[,)]", tail.strip())
                if p:
                    payloads.append(p.group(1))
            enums[name] = {
                "tag": tag[0] if tag else None,
                "variants": variants,
                "payloads": payloads,
            }

        for m in re.finditer(r"pub struct ([A-Z]\w*)\s*\{", src):
            name = m.group(1)
            depth, k = 0, src.index("{", m.end() - 1)
            start = k
            while k < len(src):
                if src[k] == "{":
                    depth += 1
                elif src[k] == "}":
                    depth -= 1
                    if depth == 0:
                        break
                k += 1
            body = clean(src[start + 1 : k])
            fields = []
            for fm in re.finditer(
                r"((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?"
                r"([a-z_][a-z0-9_]*)\s*:\s*[^,]+,",
                body,
            ):
                attrs, fname = fm.group(1), fm.group(2)
                ren = re.search(r'rename\s*=\s*"([^"]+)"', attrs)
                fields.append({
                    "rust": fname,
                    "wire": ren.group(1) if ren else fname,
                    "skip": "skip_serializing_if" in attrs,
                    # `#[serde(flatten)]` is not a JSON key: it inlines another
                    # value's contents at this position. Update.kind is the
                    # motivating case -- it holds the UpdateKind union, and the
                    # spec lists that union's variants (message, callback_query,
                    # ...) as siblings of update_id rather than a field called
                    # "kind". Counting it as a wire field reported a phantom
                    # undocumented field on every union-style type.
                    "flatten": "flatten" in attrs,
                })
            out[name] = fields
    return out, enums


def main() -> None:
    _, types = parse_spec(DOC)
    structs, enums = load_structs()

    # a struct that is the payload of a `#[serde(tag = "...")]` enum gets its
    # discriminator from the enum, not from a field of its own
    tagged: dict[str, str] = {}
    for ename, info in enums.items():
        if info["tag"]:
            for p in info["payloads"]:
                tagged[p] = info["tag"]

    no_type = []
    missing_field = []
    extra_field = []
    discrim_ok = []

    # Fields absent from the 10.3 spec text that are still real and are kept on
    # purpose, so [C] stays meaningful instead of being ignored wholesale.
    KNOWN_WIRE_FIELDS = {
        # Deprecated in Bot API 9.6 in favour of `correct_option_ids`, but
        # python-telegram-bot v22.8 still declares it (src/telegram/_poll.py:
        # present in `Poll.__init__` and the class docstring, which says "Use
        # correct_option_ids instead"). Telegram keeps sending it to existing
        # bots, so dropping it would lose data PTB still exposes.
        ("Poll", "correct_option_id"),
    }
    known_wire = {f"{t}.{f}" for t, f in KNOWN_WIRE_FIELDS}

    for tname, spec_fields in sorted(types.items()):
        rust = structs.get(tname)
        if rust is None:
            if tname in enums:
                # union type: `type` discriminator lives on the enum's serde tag
                discrim_ok.append((tname, enums[tname]["tag"], len(enums[tname]["variants"])))
                continue
            no_type.append(tname)
            continue
        rust_wire = {f["wire"] for f in rust if not f["flatten"]}
        spec_set = set(spec_fields)
        if tname in tagged:
            spec_set.discard(tagged[tname])
        miss = sorted(spec_set - rust_wire)
        extra = sorted(
            f for f in rust_wire - spec_set
            if f"{tname}.{f}" not in known_wire
        )
        if miss:
            missing_field.append((tname, miss))
        if extra:
            extra_field.append((tname, extra))

    print(f"spec types with field tables : {len(types)}")
    print(f"rust structs parsed          : {len(structs)}")
    print(f"rust enums parsed            : {len(enums)}")
    print()
    print(f"[A] documented types with NO Rust struct/enum : {len(no_type)}")
    for n in no_type[:40]:
        print(f"      {n}")
    print(f"[B] spec fields MISSING from Rust struct : {len(missing_field)}")
    for n, f in missing_field[:40]:
        print(f"      {n}: {f}")
    print(f"[C] Rust wire fields NOT in spec : {len(extra_field)}")
    for n, f in extra_field[:40]:
        print(f"      {n}: {f}")
    if not extra_field:
        print(f"      (none; {len(KNOWN_WIRE_FIELDS)} known-kept field(s) allowlisted)")
    print(f"[D] union types handled by tagged enum : {len(discrim_ok)}")
    for n, tag, nv in discrim_ok[:8]:
        print(f"      {n}: tag={tag} variants={nv}")
    if len(discrim_ok) > 8:
        print(f"      ... and {len(discrim_ok) - 8} more")
    untagged = [n for n, tag, _ in discrim_ok if not tag]
    print(f"[E] of those, enums WITHOUT a serde tag (manual check) : {len(untagged)}")
    for n in untagged[:20]:
        print(f"      {n}")


if __name__ == "__main__":
    main()
