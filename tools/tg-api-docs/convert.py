#!/usr/bin/env python3
"""Convert https://core.telegram.org/bots/api HTML to clean, compact GFM markdown.

Telegram-specific handling (a generic html2text pass mangles all of these):
  * heading anchors  -> explicit <a id="..."> so cross-references resolve everywhere
  * parameter tables -> unpadded GFM tables, pipes escaped, newlines collapsed
  * <em> param names -> *italic* (kept as-is; source uses <em>, not <code>)
  * <pre><code>      -> fenced blocks with a language hint when the class gives one
  * bare URLs         -> left verbatim (they are readable); <a href> -> [t](u)
  * literal '<' in text nodes -> &lt; (e.g. tg://user?id=<user_id>)
"""

from __future__ import annotations

import re
import sys
from datetime import date
from html import unescape

from urllib.parse import urljoin

from bs4 import BeautifulSoup, Comment, NavigableString, Tag

BASE = "https://core.telegram.org/bots/api"


def absolutize(url: str) -> str:
    """Telegram uses root-relative (/bots) and protocol-relative (//telegram.org/..)
    URLs. Both are broken in a standalone markdown file, so resolve them here."""
    if not url:
        return url
    if url.startswith("//"):
        return "https:" + url
    if url.startswith("/"):
        return "https://core.telegram.org" + url
    return url


# ---------------------------------------------------------------- inline text


def esc_text(s: str) -> str:
    """Escape a text node for markdown without mangling identifiers like chat_id."""
    s = unescape(s)
    s = s.replace("<", "&lt;").replace(">", "&gt;")
    return s


def collapse(s: str) -> str:
    return re.sub(r"[ \t\r\n]+", " ", s).strip()


def fence_for(code: str) -> str:
    longest = max((len(m) for m in re.findall(r"`+", code)), default=0)
    return "`" * max(1, longest + 1)


def inline(node) -> str:
    """Render inline HTML (em/strong/code/a/img/br) as markdown.

    Returns a string; every branch is self-contained so a node can never be
    emitted twice (an earlier shared-buffer version duplicated link/emph text).
    """
    if isinstance(node, Comment):
        return ""
    if isinstance(node, NavigableString):
        return esc_text(str(node))
    if not isinstance(node, Tag):
        return ""

    name = node.name
    cls = set(node.get("class") or [])

    if name in ("script", "style"):
        return ""
    # anchor glyphs Telegram injects into every heading
    if name == "a" and "anchor" in cls:
        return ""
    if name == "i" and "anchor-icon" in cls:
        return ""

    if name == "br":
        return "  \n"

    if name in ("em", "i", "var"):
        inner = collapse("".join(inline(c) for c in node.children))
        return f"*{inner}*" if inner else ""

    if name in ("strong", "b"):
        inner = collapse("".join(inline(c) for c in node.children))
        return f"**{inner}**" if inner else ""

    if name == "code" and (node.parent is None or node.parent.name != "pre"):
        inner = node.get_text()
        f = fence_for(inner)
        pad = " " if inner.startswith("`") or inner.endswith("`") else ""
        return f"{f}{pad}{inner}{pad}{f}"

    if name == "a":
        href = absolutize((node.get("href") or "").strip())
        inner = collapse("".join(inline(c) for c in node.children))
        if not href:
            return inner
        # internal links stay as [text](#anchor): every anchor is re-emitted as
        # <a id="..."> below, so they resolve inside the single-file markdown.
        return f"[{inner or href}]({href})"

    if name == "img":
        return f"![{node.get('alt') or ''}]({absolutize(node.get('src') or '')})"

    return "".join(inline(c) for c in node.children)


# ---------------------------------------------------------------- blocks

MD_LINK = re.compile(r"\[([^\]]*)\]\(([^)\s]+)\)")
MD_CODE = re.compile(r"(`+[^`]*`+)")


def protect(text: str) -> str:
    """Hide markdown links and code spans from table-cell escaping."""
    slots: list[str] = []

    def stash(m: re.Match) -> str:
        slots.append(m.group(0))
        return f"\x00{len(slots) - 1}\x00"

    text = MD_CODE.sub(stash, text)
    text = MD_LINK.sub(stash, text)
    return text, slots


def restore(text: str, slots: list[str]) -> str:
    return re.sub(r"\x00(\d+)\x00", lambda m: slots[int(m.group(1))], text)


def cell_text(td: Tag) -> str:
    raw = collapse("".join(inline(c) for c in td.children))
    raw, slots = protect(raw)
    raw = raw.replace("|", "\\|")
    return restore(raw, slots) or "—"


def render_table(tbl: Tag) -> str:
    rows = tbl.find_all("tr")
    if not rows:
        return ""
    grid: list[list[str]] = []
    for tr in rows:
        cells = tr.find_all(["th", "td"], recursive=False)
        if not cells:
            cells = tr.find_all(["th", "td"])
        grid.append([cell_text(c) for c in cells])

    width = max(len(r) for r in grid)
    header = grid[0] + [""] * (width - len(grid[0]))
    body = grid[1:] or [[]]
    body = [r + [""] * (width - len(r)) for r in body]

    out = ["| " + " | ".join(header) + " |", "|" + "|".join([" --- "] * width) + "|"]
    for r in body:
        out.append("| " + " | ".join(c or "—" for c in r) + " |")
    return "\n".join(out)


def render_list(lst: Tag, depth: int = 0) -> str:
    lines: list[str] = []
    indent = "  " * depth
    for li in lst.find_all("li", recursive=False):
        nested = [c for c in li.find_all(["ul", "ol"], recursive=False)]
        for n in nested:
            n.extract()
        body = collapse("".join(inline(c) for c in li.children))
        ordered = lst.name == "ol"
        first = f"{indent}{'1.' if ordered else '-'} {body}" if body else f"{indent}-"
        lines.append(first)
        for n in nested:
            sub = render_list(n, depth + 1)
            if sub:
                lines.append(sub)
    return "\n".join(l for l in lines if l.strip())


def render_blockquote(bq: Tag) -> str:
    sub: list[str] = []
    render_children(bq, sub)
    inner = normalize(sub)
    quoted = [("> " + l).rstrip() for l in inner.split("\n")]
    while quoted and quoted[-1] == ">":
        quoted.pop()
    return "\n".join(quoted)


def heading_id(tag: Tag) -> str | None:
    a = tag.find("a", class_="anchor")
    return a.get("name") if a else None


def slug(text: str) -> str:
    s = re.sub(r"[^\w\- ]+", "", text.lower())
    return re.sub(r"\s+", "-", s.strip())


HEADINGS: list[tuple[int, str, str]] = []


BR = "\n"


def render_paragraph(node: Tag) -> str:
    """Render a <p>, keeping <br>-separated lines as markdown hard breaks.

    Telegram encodes a lot of pseudo-lists as `<p>text:<br/>- a<br/>- b</p>`,
    so collapsing the whole paragraph into one line would destroy real structure.
    """
    parts: list[str] = []
    buf: list[str] = []
    for child in node.children:
        if isinstance(child, Tag) and child.name == "br":
            parts.append("".join(buf))
            buf = []
            continue
        buf.append(inline(child))
    parts.append("".join(buf))

    lines = [collapse(p) for p in parts]
    lines = [l for l in lines if l]
    if not lines:
        return ""
    return ("  " + BR).join(lines)


def render_children(node: Tag, out: list[str]) -> None:
    for child in node.children:
        if isinstance(child, Comment):
            continue
        if isinstance(child, NavigableString):
            txt = collapse(str(child))
            if txt:
                out.append(txt + "\n")
            continue
        if not isinstance(child, Tag):
            continue

        name = child.name

        if name in ("script", "style"):
            continue
        if name == "i" and "anchor-icon" in set(child.get("class") or []):
            continue

        if re.fullmatch(r"h[1-6]", name):
            level = int(name[1])
            hid = heading_id(child)
            text = collapse("".join(inline(c) for c in child.children))
            if level in (3, 4) and text:
                HEADINGS.append((level, text, hid or slug(text)))
            out.append(f"\n{'#' * level} {text}\n")
            if hid:
                out.append(f'<a id="{hid}"></a>\n')
            continue

        if name == "table":
            t = render_table(child)
            if t:
                out.append("\n" + t + "\n")
            continue

        if name in ("ul", "ol"):
            lst = render_list(child)
            if lst:
                out.append("\n" + lst + "\n")
            continue

        if name == "blockquote":
            out.append("\n" + render_blockquote(child) + "\n")
            continue

        if name == "pre":
            code = child.get_text()
            lang = ""
            c = child.find("code")
            if c:
                cls = " ".join(c.get("class") or [])
                m = re.search(r"language-([\w+-]+)", cls)
                if m:
                    lang = m.group(1)
            # the MarkdownV2 sample embeds ``` inside its own body, so the fence
            # has to be longer than the longest backtick run in the content
            longest = max((len(m) for m in re.findall(r"`+", code)), default=0)
            fence = "`" * max(3, longest + 1)
            out.append(f"\n{fence}{lang}\n{code.rstrip()}\n{fence}\n")
            continue

        if name == "hr":
            out.append("\n---\n")
            continue

        if name in ("p", "span", "i", "b", "div"):
            # containers: <br>-separated lines become markdown hard breaks
            txt = render_paragraph(child) or collapse(inline(child))
        elif name in ("em", "strong", "code", "a", "br"):
            # these carry their own syntax (href, fence, emphasis) -> render whole
            txt = collapse(inline(child))
        else:
            continue
        if txt:
            out.append("\n" + txt + "\n")
        continue

        # containers: recurse
        render_children(child, out)


def normalize(lines: list[str]) -> str:
    md = "".join(lines)
    md = re.sub(r"\n{3,}", "\n\n", md)
    # strip trailing whitespace, but never the two spaces that encode a hard break
    md = re.sub(r"[ \t]{3,}\n", "\n", md)
    md = re.sub(r"(?<!\n)\n(?!\n)", "\n", md)
    return md.strip() + "\n"


def main() -> None:
    src = sys.argv[1]
    dst = sys.argv[2]
    scraped = sys.argv[3] if len(sys.argv) > 3 else date.today().isoformat()
    soup = BeautifulSoup(open(src, encoding="utf-8").read(), "lxml")
    content = soup.find(id="dev_page_content")
    if content is None:
        raise SystemExit("dev_page_content not found")

    out: list[str] = []
    render_children(content, out)
    body = normalize(out)

    # table of contents from h3/h4 (mirrors the page's own section structure)
    toc = [
        f"{'  ' if lvl == 4 else ''}- [{text}](#{anchor})"
        for lvl, text, anchor in HEADINGS
    ]

    header = f"""---
title: "Telegram Bot API"
source: "https://core.telegram.org/bots/api"
bot_api_version: "10.4"
scraped: "{scraped}"
converter: "tools/tg-api-docs/convert.py (BeautifulSoup + lxml, Telegram-aware)"
---

# Telegram Bot API

## Table of Contents

{chr(10).join(toc)}

---

"""

    open(dst, "w", encoding="utf-8").write(header + body)
    print(
        f"wrote {dst}: {len(header + body)} bytes, "
        f"{len(toc)} toc entries, {len(HEADINGS)} headings"
    )


if __name__ == "__main__":
    main()
