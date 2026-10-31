#!/usr/bin/env python3
import base64
import os
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
TARGETS = [
    (HERE / "menu_exact.template.html", HERE / "menu_exact.html"),
    (HERE / "menu_redesign.template.html", HERE / "menu_redesign.html"),
]
ACTAY_LOCAL = HERE / "ActayWide-Bold.otf"
ACTAY_DRIVE = pathlib.Path(os.environ.get("ACTAY_OTF", "/home/user/drive/so.loud/data/peekgg/ActayWide-Bold.otf"))
ICONS = HERE / "peekgg-icons.otf"


def b64(path):
    return base64.b64encode(path.read_bytes()).decode("ascii")


def main():
    actay = ACTAY_LOCAL if ACTAY_LOCAL.exists() else ACTAY_DRIVE
    if not actay.exists():
        sys.exit(f"нет файла шрифта: {ACTAY_LOCAL} / {ACTAY_DRIVE}")
    for template, output in TARGETS:
        if not template.exists():
            continue
        html = template.read_text(encoding="utf-8")
        html = html.replace("__ACTAY__", b64(actay))
        html = html.replace("__ICONS__", b64(ICONS))
        output.write_text(html, encoding="utf-8")
        print(f"{output.name} — {output.stat().st_size} байт")


if __name__ == "__main__":
    main()
