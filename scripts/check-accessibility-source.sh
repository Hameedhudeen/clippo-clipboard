#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import sys

ROOT = Path.cwd()

CHECKS = [
    (
        "macOS search field has accessibility label",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        'accessibilityLabel("Search clipboard history")',
    ),
    (
        "macOS history list has accessibility label",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        'accessibilityLabel("Clipboard history")',
    ),
    (
        "macOS popup focuses search on appear",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "searchFocused = true",
    ),
    (
        "macOS rows expose item accessibility labels",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        ".accessibilityLabel(item.text)",
    ),
    (
        "Windows search field has accessible name",
        ROOT / "apps/windows/Program.cs",
        'search.AccessibleName = "Search clipboard history"',
    ),
    (
        "Windows history list has accessible name",
        ROOT / "apps/windows/Program.cs",
        'list.AccessibleName = "Clipboard history"',
    ),
    (
        "Windows popup focuses search on open",
        ROOT / "apps/windows/Program.cs",
        "search.Focus();",
    ),
    (
        "Windows preferences window has accessible name",
        ROOT / "apps/windows/Program.cs",
        'AccessibleName = "Clippo Preferences"',
    ),
    (
        "Linux fallback search dialog has visible label",
        ROOT / "apps/linux/src/main.rs",
        '"Search clipboard history".to_string()',
    ),
    (
        "Linux fallback opens search before history selection",
        ROOT / "apps/linux/src/main.rs",
        "let query = match ZenityDialog::search_query()",
    ),
    (
        "Linux fallback history dialog exposes shortcut column",
        ROOT / "apps/linux/src/main.rs",
        '"Shortcut".to_string()',
    ),
    (
        "Linux fallback history dialog exposes clip column",
        ROOT / "apps/linux/src/main.rs",
        '"Clip".to_string()',
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
    print("Accessibility source checks failed:")
    for item in missing:
        print(f"- {item}")
    sys.exit(1)

print("Accessibility source check passed: labels and initial focus hooks are present")
PY
