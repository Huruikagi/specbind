"""Build both languages and publish a single sitemap with translation links."""

import gzip
import subprocess
import sys
from pathlib import Path
from xml.etree import ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
SITE = ROOT / "site"
EN_URL = "https://huruikagi.github.io/specbind/"
JA_URL = EN_URL + "ja/"
SITEMAP = "{http://www.sitemaps.org/schemas/sitemap/0.9}"
XHTML = "{http://www.w3.org/1999/xhtml}"


def sitemap_pages(sitemap: Path, base_url: str) -> dict[str, ET.Element]:
    pages = {}
    for entry in ET.parse(sitemap).getroot().findall(SITEMAP + "url"):
        url = entry.findtext(SITEMAP + "loc", "")
        if not url.startswith(base_url):
            raise ValueError(f"Unexpected URL in {sitemap}: {url!r}")
        relative_path = url.removeprefix(base_url)
        if relative_path in pages:
            raise ValueError(f"Duplicate URL in {sitemap}: {url}")
        pages[relative_path] = entry
    if "" not in pages:
        raise ValueError(f"Missing home page in {sitemap}: {base_url}")
    return pages


def combine_sitemaps() -> None:
    sitemap = SITE / "sitemap.xml"
    en = sitemap_pages(sitemap, EN_URL)
    ja = sitemap_pages(SITE / "ja" / "sitemap.xml", JA_URL)
    if en.keys() != ja.keys():
        raise ValueError(
            f"Missing translations. English: {sorted(ja.keys() - en.keys())}. "
            f"Japanese: {sorted(en.keys() - ja.keys())}."
        )

    ET.register_namespace("", SITEMAP[1:-1])
    ET.register_namespace("xhtml", XHTML[1:-1])
    combined = ET.Element(SITEMAP + "urlset")
    for pages in (en, ja):
        for relative_path, entry in pages.items():
            for lang, base_url in (("en", EN_URL), ("ja", JA_URL)):
                ET.SubElement(
                    entry,
                    XHTML + "link",
                    rel="alternate",
                    hreflang=lang,
                    href=base_url + relative_path,
                )
            combined.append(entry)
    ET.indent(combined)
    xml = ET.tostring(combined, encoding="utf-8", xml_declaration=True)
    sitemap.write_bytes(xml)
    sitemap.with_suffix(".xml.gz").write_bytes(gzip.compress(xml, mtime=0))
    print(f"Built {len(en)} English and {len(ja)} Japanese pages.")


def main() -> int:
    # The English clean build owns site/, so it must precede the Japanese build.
    for lang in ("en", "ja"):
        result = subprocess.run(
            [
                sys.executable, "-m", "zensical", "build", "--clean",
                "-f", f"zensical.{lang}.yml",
            ],
            cwd=ROOT,
            check=False,
        )
        if result.returncode:
            return result.returncode
    try:
        combine_sitemaps()
    except (OSError, ET.ParseError, ValueError) as error:
        print(f"Could not combine documentation sitemaps: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
