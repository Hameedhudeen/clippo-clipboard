#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import sys

ROOT = Path.cwd()

CHECKS = [
    (
        "macOS search field label",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        'accessibilityLabel("Search clipboard history")',
    ),
    (
        "macOS history list label",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        'accessibilityLabel("Clipboard history")',
    ),
    (
        "macOS row text label",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        ".accessibilityLabel(item.text)",
    ),
    (
        "macOS pinned row hint",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        'accessibilityHint(item.pinned ? "Pinned clipboard history item" : "Clipboard history item")',
    ),
    (
        "macOS preferences toggle label",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        'accessibilityLabel("Launch Clippo at login")',
    ),
    (
        "Windows search accessible name",
        ROOT / "apps/windows/Program.cs",
        'search.AccessibleName = "Search clipboard history"',
    ),
    (
        "Windows search accessible description",
        ROOT / "apps/windows/Program.cs",
        'search.AccessibleDescription = "Filters the visible clipboard history items."',
    ),
    (
        "Windows history accessible name",
        ROOT / "apps/windows/Program.cs",
        'list.AccessibleName = "Clipboard history"',
    ),
    (
        "Windows history accessible description",
        ROOT / "apps/windows/Program.cs",
        'list.AccessibleDescription = "Clipboard history items. Use arrow keys, Enter, numbers, or action buttons."',
    ),
    (
        "Windows footer button role",
        ROOT / "apps/windows/Program.cs",
        "AccessibleRole = AccessibleRole.PushButton",
    ),
    (
        "Windows preferences checkbox role",
        ROOT / "apps/windows/Program.cs",
        "AccessibleRole = AccessibleRole.CheckButton",
    ),
    (
        "Linux search dialog label",
        ROOT / "apps/linux/src/main.rs",
        '"Search clipboard history".to_string()',
    ),
    (
        "Linux history shortcut column",
        ROOT / "apps/linux/src/main.rs",
        '"Shortcut".to_string()',
    ),
    (
        "Linux history clip column",
        ROOT / "apps/linux/src/main.rs",
        '"Clip".to_string()',
    ),
    (
        "Linux preferences native dialog title",
        ROOT / "apps/linux/src/main.rs",
        'ZenityDialog::info("Clippo Preferences", &body)',
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
    print("Screen reader source checks failed:")
    for item in missing:
        print(f"- {item}")
    sys.exit(1)

print("Screen reader source check passed: history and preferences expose baseline accessible names, roles, and labels")
PY
