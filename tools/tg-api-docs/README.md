# Telegram Bot API doc tooling

Reproducible scrape of <https://core.telegram.org/bots/api> into
`docs/telegram-bot-api.md`, plus the two verifiers that prove the conversion is
lossless. Use this when a new Bot API release lands so the parity audit in
`docs/api-10.4-gap.md` has a trustworthy source of truth.

## Usage

```bash
# 1. fetch the page (the docs are server-rendered; no browser needed)
curl -sS -L -A "Mozilla/5.0" -o /tmp/api.html https://core.telegram.org/bots/api

# 2. convert to markdown
uv run --with beautifulsoup4 --with lxml \
  tools/tg-api-docs/convert.py /tmp/api.html docs/telegram-bot-api.md

# 3. verify (both must report zero unexplained loss)
uv run --with beautifulsoup4 --with lxml \
  tools/tg-api-docs/verify.py /tmp/api.html docs/telegram-bot-api.md
uv run --with beautifulsoup4 --with lxml \
  tools/tg-api-docs/structcheck.py /tmp/api.html docs/telegram-bot-api.md
```

`make tg-api-docs` runs all four steps.

## Why a custom converter

A generic `html2text` pass mangles this page in ways that cost real content:

- **Anchor glyphs.** Every heading starts with `<a class="anchor"><i class="anchor-icon">`.
  Naive converters emit a stray link per heading; we drop the glyph and re-emit the
  real target as `<a id="...">`, so all 2,222 cross-references resolve inside the
  single markdown file (verified: 0 dead links).
- **Parameter tables.** 544 tables whose rows must become compact, unpadded GFM
  rows with `|` escaped. Padding alone inflated the previous snapshot to 1.55 MB.
- **`<br>`-encoded lists.** Telegram writes pseudo-lists as
  `<p>text:<br/>- a<br/>- b</p>`; collapsing whitespace silently merges them into
  one paragraph.
- **Code fences.** The MarkdownV2 sample embeds ``` ``` ``` inside its own body, so
  the fence must be longer than the longest backtick run in the content.
- **Root-relative URLs.** `/bots` and `//telegram.org/...` are resolved against
  `https://core.telegram.org`, otherwise every off-page link breaks outside the site.

## Verification

`verify.py` compares the token multiset of the HTML text against the markdown
(fence-aware, so literal markup inside code samples is not mistaken for real
links). `structcheck.py` compares structure: tables row-by-row and column-by-column,
link and anchor inventories, images, fences, inline code, and heading levels.

Current snapshot, scraped 2026-11-19 against Bot API 10.4 (released 2026-11-09):

| Check | Result |
|---|---|
| Tables | 572 / 572, 0 row/column mismatches |
| Headings | h3 14/14 · h4 633/633 · h6 7/7 |
| Internal links | 2,316 (1,701 content + 615 TOC) |
| Anchors | 654 / 654, 0 dead links |
| Inline code | 331 / 331 |
| Images | 95 (7 decorative HTML-only images) |
| `<pre>` samples | 9 / 9 byte-identical |
| Vocabulary vs HTML | 3,904 unique tokens both sides |

`verify.py` reports 9 "lost" tokens (`lt`, `gt`, `amp`). These are an artifact of
the verifier unescaping entities one level differently than BeautifulSoup, not a
conversion loss — `structcheck.py` confirms all 9 `<pre>` blocks are byte-identical
to the source.
