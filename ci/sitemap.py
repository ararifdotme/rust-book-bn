#!/usr/bin/env python3
"""Write sitemap.xml for the built book.

Usage: python3 ci/sitemap.py <book-dir> <base-url>
Example: python3 ci/sitemap.py book https://rust-bangla.ararif.me

Lists every top-level HTML page in <book-dir>, except:
- redirect stubs (pages with a meta refresh, from [output.html.redirect]),
- 404.html, toc.html and print.html (the whole book on one page),
- the first page in src/SUMMARY.md, because mdBook also serves it as
  index.html and the site root ("/") is listed instead.
"""

import re
import sys
from pathlib import Path
from xml.sax.saxutils import escape

SKIP = {"404.html", "toc.html", "print.html"}


def first_summary_page():
    summary = Path("src/SUMMARY.md").read_text(encoding="utf-8")
    m = re.search(r"\]\(([^)]+)\.md\)", summary)
    return f"{m.group(1)}.html" if m else None


def main():
    book_dir, base = Path(sys.argv[1]), sys.argv[2].rstrip("/")
    skip = SKIP | {first_summary_page()}

    urls = [f"{base}/"]
    for page in sorted(book_dir.glob("*.html")):
        if page.name == "index.html" or page.name in skip:
            continue
        if 'http-equiv="refresh"' in page.read_text(encoding="utf-8"):
            continue
        urls.append(f"{base}/{page.name}")

    lines = ['<?xml version="1.0" encoding="UTF-8"?>',
             '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">']
    lines += [f"  <url><loc>{escape(u)}</loc></url>" for u in urls]
    lines.append("</urlset>")
    (book_dir / "sitemap.xml").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"sitemap.xml: {len(urls)} URLs")


if __name__ == "__main__":
    main()
