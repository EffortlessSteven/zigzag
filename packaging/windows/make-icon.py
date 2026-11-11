#!/usr/bin/env python3
"""Regenerate packaging/windows/AppIcon.ico from AppIcon.png.

Needs Pillow:  pip install --upgrade pillow
"""
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent.parent
SRC = ROOT / "packaging" / "macos" / "AppIcon.png"
DST = ROOT / "packaging" / "windows" / "AppIcon.ico"

# Windows pulls the size it needs for Explorer, the Start menu, the taskbar
# and the title bar from the .ico's PNG-encoded frames.
SIZES = [(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]


def main() -> None:
    Image.open(SRC).convert("RGBA").save(DST, format="ICO", sizes=SIZES)
    print(f"Wrote {DST.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
