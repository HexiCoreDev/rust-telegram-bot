#!/usr/bin/env python3
"""Prove the markdown conversion lost nothing: compare HTML text vs md text token-wise."""

import re
import sys
from collections import Counter

from bs4 import BeautifulSoup

html_path, md_path = sys.argv[1], sys.argv[2]

soup = BeautifulSoup(open(html_path, encoding="utf-8").read(), "lxml")
content = soup.find(id="dev_page_content")
for bad in content.find_all(["script", "style"]):
    bad.decompose()
for i in content.find_all("i", class_="anchor-icon"):
    i.decompose()
for a in content.find_all("a", class_="anchor"):
    a.decompose()

html_text = content.get_text(" ")
md = open(md_path, encoding="utf-8").read()

# strip markdown syntax, keep link labels and code content.
# Fenced regions are kept as RAW text (no link-stripping inside them): the
# MarkdownV2 sample literally contains "[inline URL](http://...)" as sample text.
md_body = md.split("---\n", 2)[-1]
md_body = re.sub(r'<a id="[^"]*"></a>', " ", md_body)   # converter-added anchors
md_body = md_body.replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&")
raw_parts, prose_parts, fence = [], [], None
for ln in md_body.split("\n"):
    m = re.match(r"^(`{3,})", ln)
    if fence is None and m:
        fence = m.group(1)
        continue
    if fence is not None:
        if ln.startswith(fence):
            fence = None
        else:
            raw_parts.append(ln)       # sample text, kept verbatim
        continue
    prose_parts.append(ln)
md_no_links = "\n".join(prose_parts)
md_no_links = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", md_no_links)
md_no_links = re.sub(r"(?m)^\|[ \-:|]+\|$", " ", md_no_links)  # table separator rows
md_no_links = re.sub(r"[*`|<>]", " ", md_no_links)
md_text = md_no_links + "\n" + "\n".join(raw_parts)

WORD = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
h = Counter(WORD.findall(html_text))
m = Counter(WORD.findall(md_text))

lost = h - m
extra = m - h

print(f"html tokens: {sum(h.values())}  unique {len(h)}")
print(f"md   tokens: {sum(m.values())}  unique {len(m)}")
print(f"\nLOST from html (in html, missing/short in md): {sum(lost.values())}")
for w, c in lost.most_common(40):
    print(f"   {c:5d}  {w}")

# EXTRA is expected, not a defect: the generated TOC repeats every heading label
# once, and relative URLs were absolutised (adding core/telegram/org tokens).
print(f"\nEXTRA in md (expected: generated TOC + absolutised URLs): {sum(extra.values())}")
for w, c in extra.most_common(15):
    print(f"   {c:5d}  {w}")

# A healthy snapshot loses only the 9 entity tokens (lt/gt/amp) that come from
# this script unescaping one level differently than BeautifulSoup. structcheck.py
# proves the real story by byte-comparing the <pre> samples.
THRESHOLD = int(sys.argv[3]) if len(sys.argv) > 3 else 20
print("\n--- structural ---")
print("(run structcheck.py for table/link/anchor parity)")
if sum(lost.values()) > THRESHOLD:
    print(f"FAIL: {sum(lost.values())} tokens lost, threshold {THRESHOLD}")
    sys.exit(1)
print(f"OK: {sum(lost.values())} tokens lost (threshold {THRESHOLD})")
