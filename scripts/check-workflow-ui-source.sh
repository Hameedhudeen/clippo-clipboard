#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import sys

ROOT = Path.cwd()

CHECKS = [
    (
        "UI docs declare Clippo workflow reference",
        ROOT / "docs/UI_UX.md",
        "Clippo uses its own compact, keyboard-first clipboard workflow as the UI/UX reference.",
    ),
    (
        "UI docs preserve compact searchable list",
        ROOT / "docs/UI_UX.md",
        "- Compact searchable list.",
    ),
    (
        "UI docs preserve search-first flow",
        ROOT / "docs/UI_UX.md",
        "- Search-first flow.",
    ),
    (
        "UI docs preserve pinned section",
        ROOT / "docs/UI_UX.md",
        "- Pinned section above regular history.",
    ),
    (
        "Parity docs record Clippo workflow target",
        ROOT / "docs/PARITY.md",
        "Clippo targets equivalent clipboard workflows on macOS, Windows, and Linux.",
    ),
    (
        "macOS popup search field",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        'TextField("Search clipboard history"',
    ),
    (
        "macOS popup focuses search",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "searchFocused = true",
    ),
    (
        "macOS pinned section",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        'Section("Pinned")',
    ),
    (
        "macOS history section",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        'Section("History")',
    ),
    (
        "macOS visible shortcut label",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "model.visibleShortcut(for: item)",
    ),
    (
        "macOS native full preview help",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        ".help(item.text)",
    ),
    (
        "macOS footer actions",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        'Menu("Actions")',
    ),
    (
        "Windows popup search field",
        ROOT / "apps/windows/Program.cs",
        'search.PlaceholderText = "Search clipboard history"',
    ),
    (
        "Windows popup focuses search",
        ROOT / "apps/windows/Program.cs",
        "historyWindow.FocusSearch();",
    ),
    (
        "Windows compact history list",
        ROOT / "apps/windows/Program.cs",
        "list.View = View.Details",
    ),
    (
        "Windows visible shortcut column",
        ROOT / "apps/windows/Program.cs",
        'list.Columns.Add("Shortcut", 80)',
    ),
    (
        "Windows full preview tooltip",
        ROOT / "apps/windows/Program.cs",
        "ToolTipText = item.Text",
    ),
    (
        "Windows footer copy action",
        ROOT / "apps/windows/Program.cs",
        'footer.Controls.Add(Button("Copy"',
    ),
    (
        "Windows footer paste action",
        ROOT / "apps/windows/Program.cs",
        'footer.Controls.Add(Button("Paste"',
    ),
    (
        "Windows footer pause action",
        ROOT / "apps/windows/Program.cs",
        'footer.Controls.Add(Button("Pause"',
    ),
    (
        "Linux fallback search-first flow",
        ROOT / "apps/linux/src/main.rs",
        "let query = match ZenityDialog::search_query()",
    ),
    (
        "Linux fallback compact list",
        ROOT / "apps/linux/src/main.rs",
        "fn zenity_history_command(history: &[LinuxHistoryItem])",
    ),
    (
        "Linux visible shortcut column",
        ROOT / "apps/linux/src/main.rs",
        '"Shortcut".to_string()',
    ),
    (
        "Linux pinned column",
        ROOT / "apps/linux/src/main.rs",
        '"Pin".to_string()',
    ),
    (
        "Linux full-text native action",
        ROOT / "apps/linux/src/main.rs",
        'const SHOW_FULL_TEXT_LABEL: &\'static str = "Show Full Text";',
    ),
    (
        "Linux action dialog includes paste without formatting",
        ROOT / "apps/linux/src/main.rs",
        'const PASTE_PLAIN_LABEL: &\'static str = "Paste Without Formatting";',
    ),
    (
        "Linux pause command",
        ROOT / "apps/linux/src/main.rs",
        "--pause-capture",
    ),
    (
        "Linux ignore-next command",
        ROOT / "apps/linux/src/main.rs",
        "--ignore-next-copy",
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
    print("Clippo workflow UI source checks failed:")
    for item in missing:
        print(f"- {item}")
    sys.exit(1)

print("Clippo workflow UI source check passed: search-first, compact list, pinned rows, shortcuts, preview, and actions are represented")
PY
