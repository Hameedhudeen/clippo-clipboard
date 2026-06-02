#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import sys

ROOT = Path.cwd()

CHECKS = [
    (
        "macOS rows expose native help text for full item content",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        ".help(item.text)",
    ),
    (
        "Windows rows expose native tooltip text for full item content",
        ROOT / "apps/windows/Program.cs",
        "ToolTipText = item.Text",
    ),
    (
        "Linux fallback exposes full text action",
        ROOT / "apps/linux/src/main.rs",
        'const SHOW_FULL_TEXT_LABEL: &\'static str = "Show Full Text";',
    ),
    (
        "Linux fallback full text action opens native info dialog",
        ROOT / "apps/linux/src/main.rs",
        'ZenityDialog::info("Clippo Item", &item.text)',
    ),
]

missing = []
for label, path, needle in CHECKS:
    if not path.exists():
        missing.append(f"{label}: missing file {path.relative_to(ROOT)}")
        continue
    text = path.read_text(encoding="utf-8", errors="replace")
    if needle not in text:
        missing.append(f"{label}: missing `{needle}` in {path.relative_to(ROOT)}")

if missing:
    print("Full-preview source checks failed:")
    for item in missing:
        print(f"- {item}")
    sys.exit(1)

print("Full-preview source check passed: each shell exposes full item text")
PY
