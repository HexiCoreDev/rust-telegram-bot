#!/usr/bin/env python3
"""Structural parity check: tables, links, images, code fences, anchors.

Fence-aware so literal '|' or '[' inside code samples are not counted as markup.
"""

import re
import sys

from bs4 import BeautifulSoup

html_path, md_path = sys.argv[1], sys.argv[2]
md = open(md_path, encoding="utf-8").read()
body = md.split("---\n", 2)[-1]
lines = body.split("\n")

# drop fenced regions so sample code cannot be mistaken for markup
outside, fence = [], None
for ln in lines:
    m = re.match(r"^(`{3,})", ln)
    if fence is None and m:
        fence = m.group(1)
        continue
    if fence is not None:
        if ln.startswith(fence):
            fence = None
        continue
    outside.append(ln)
outside_text = "\n".join(outside)

soup = BeautifulSoup(open(html_path, encoding="utf-8").read(), "lxml")
c = soup.find(id="dev_page_content")

# --- tables, in document order ---
blocks, cur = [], []
for ln in outside:
    if ln.startswith("|"):
        cur.append(ln)
    elif cur:
        blocks.append(cur)
        cur = []
if cur:
    blocks.append(cur)
tabs = c.find_all("table")

mismatch = []
for i, (b, t) in enumerate(zip(blocks, tabs)):
    md_rows = len(b) - 1  # minus separator
    html_rows = len(t.find_all("tr"))
    md_cols = len(b[0].strip("|").split("|"))
    html_cols = len(t.find("tr").find_all(["th", "td"]))
    if md_rows != html_rows or md_cols != html_cols:
        mismatch.append((i, md_rows, html_rows, md_cols, html_cols))

print("tables          md=%-5d html=%-5d  mismatched=%d" % (len(blocks), len(tabs), len(mismatch)))
for x in mismatch[:10]:
    print("   block#%d md_rows=%d html_rows=%d md_cols=%d html_cols=%d" % x)

# --- links ---
anchors_html = [a for a in c.find_all("a")]
internal_html = [a for a in anchors_html if (a.get("href") or "").startswith("#")]
external_html = [
    a for a in anchors_html if (a.get("href") or "") and not a["href"].startswith("#")
]
md_int = len(re.findall(r"\]\(#", outside_text))
md_ext = len(re.findall(r"\]\((?!#)", outside_text))
print("internal links  md=%-5d html=%-5d  (md includes the 615-entry TOC)" % (md_int, len(internal_html)))
print("external links  md=%-5d html=%-5d" % (md_ext, len(external_html)))

defined = set(re.findall(r'<a id="([^"]+)"></a>', body))
targets = set(re.findall(r"\]\(#([^)]+)\)", body))
dead = sorted(t for t in targets if t not in defined)
print("anchors         md=%-5d html=%-5d  dead internal links=%d %s" % (
    len(defined), len([a for a in anchors_html if a.get("name")]), len(dead), dead[:10]))

# --- images, code ---
imgs_html = [i for i in c.find_all("img")]
print("images          md=%-5d html=%-5d" % (
    len(re.findall(r"!\[", outside_text)), len(imgs_html)))
print("code fences     md=%-5d html(pre)=%-5d" % (
    len(re.findall(r"^`{3,}", body, re.M)), len(c.find_all("pre"))))
print("inline code     md=%-5d html=%-5d (html count includes the 9 <pre><code>)" % (
    len(re.findall(r"(?<!`)`[^`\n]+`(?!`)", outside_text)), len(c.find_all("code"))))
for lvl in (3, 4, 5, 6):
    print("h%d              md=%-5d html=%-5d" % (
        lvl, len(re.findall(r"^#{" + str(lvl) + r"} ", outside_text, re.M)),
        len(c.find_all("h" + str(lvl)))))
print("list items      md=%-5d html=%-5d (md includes the 615-entry TOC)" % (
    len(re.findall(r"^\s*- ", outside_text, re.M)), len(c.find_all("li"))))
