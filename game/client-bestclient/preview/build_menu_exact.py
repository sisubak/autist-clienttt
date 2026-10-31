#!/usr/bin/env python3
import base64
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
TEMPLATE = HERE / "menu_exact.template.html"
OUTPUT = HERE / "menu_exact.html"
ACTAY = pathlib.Path("/home/user/drive/so.loud/data/peekgg/ActayWide-Bold.otf")
ICONS = HERE / "peekgg-icons.otf"


def b64(path):
    return base64.b64encode(path.read_bytes()).decode("ascii")


def main():
    if not ACTAY.exists():
        sys.exit(f"нет файла шрифта: {ACTAY}")
    html = TEMPLATE.read_text(encoding="utf-8")
    html = html.replace("__ACTAY__", b64(ACTAY))
    html = html.replace("__ICONS__", b64(ICONS))
    OUTPUT.write_text(html, encoding="utf-8")
    print(f"{OUTPUT} — {OUTPUT.stat().st_size} байт")


if __name__ == "__main__":
    main()
