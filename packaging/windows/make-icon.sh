#!/bin/bash
# Regenerate packaging/windows/AppIcon.ico from AppIcon.png.
# Needs Python with Pillow: pip install --upgrade pillow
set -eo pipefail
cd "$(dirname "$0")/../.."
python3 packaging/windows/make-icon.py
