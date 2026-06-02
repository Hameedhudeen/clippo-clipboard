#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import sys

ROOT = Path.cwd()

CHECKS = [
    ("macOS keyboard copy", ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift", ".keyboardShortcut(.return, modifiers: [])"),
    ("macOS keyboard paste", ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift", ".keyboardShortcut(.return, modifiers: [.option])"),
    ("macOS keyboard plain paste", ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift", ".keyboardShortcut(.return, modifiers: [.option, .shift])"),
    ("macOS pointer row action", ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift", ".onTapGesture"),
    ("macOS pointer action menu", ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift", 'Menu("Actions")'),
    ("Windows keyboard handler", ROOT / "apps/windows/Program.cs", "private void OnKeyDown"),
    ("Windows keyboard paste", ROOT / "apps/windows/Program.cs", "eventArgs.Alt && eventArgs.KeyCode == Keys.Enter"),
    ("Windows keyboard plain paste", ROOT / "apps/windows/Program.cs", "eventArgs.Alt && eventArgs.Shift && eventArgs.KeyCode == Keys.Enter"),
    ("Windows pointer handler", ROOT / "apps/windows/Program.cs", "private void OnListMouseClick"),
    ("Windows footer buttons", ROOT / "apps/windows/Program.cs", 'footer.Controls.Add(Button("Copy"'),
    ("Windows footer paste", ROOT / "apps/windows/Program.cs", 'footer.Controls.Add(Button("Paste"'),
    ("Windows footer preferences", ROOT / "apps/windows/Program.cs", 'footer.Controls.Add(Button("Prefs"'),
    ("Linux fallback action dialog", ROOT / "apps/linux/src/main.rs", "fn zenity_history_action_command"),
    ("Linux fallback copy action", ROOT / "apps/linux/src/main.rs", 'const COPY_LABEL: &\'static str = "Copy";'),
    ("Linux fallback paste action", ROOT / "apps/linux/src/main.rs", 'const PASTE_LABEL: &\'static str = "Paste";'),
    ("Linux fallback plain paste action", ROOT / "apps/linux/src/main.rs", 'const PASTE_PLAIN_LABEL: &\'static str = "Paste Without Formatting";'),
    ("Linux fallback pin action", ROOT / "apps/linux/src/main.rs", 'const PIN_LABEL: &\'static str = "Pin";'),
    ("Linux fallback delete action", ROOT / "apps/linux/src/main.rs", 'const DELETE_LABEL: &\'static str = "Delete";'),
    ("Linux keyboard shortcut copy", ROOT / "apps/linux/src/main.rs", "--copy-shortcut="),
    ("Linux keyboard shortcut paste", ROOT / "apps/linux/src/main.rs", "--paste-shortcut="),
    ("Linux keyboard shortcut plain paste", ROOT / "apps/linux/src/main.rs", "--paste-plain-shortcut="),
    ("Linux keyboard shortcut pin", ROOT / "apps/linux/src/main.rs", "--toggle-pin-shortcut="),
    ("Linux keyboard shortcut delete", ROOT / "apps/linux/src/main.rs", "--delete-shortcut="),
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
    print("Interaction source checks failed:")
    for item in missing:
        print(f"- {item}")
    sys.exit(1)

print("Interaction source check passed: pointer and keyboard action paths are present")
PY
