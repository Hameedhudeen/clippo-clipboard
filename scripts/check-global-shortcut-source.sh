#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import sys

ROOT = Path.cwd()

CHECKS = [
    (
        "macOS Carbon hotkey registration",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "RegisterEventHotKey",
    ),
    (
        "macOS global shortcut opens history",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "self?.showHistoryWindow()",
    ),
    (
        "macOS default global shortcut",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "cmdKey | shiftKey",
    ),
    (
        "Windows hotkey registration",
        ROOT / "apps/windows/Program.cs",
        "NativeMethods.RegisterHotKey",
    ),
    (
        "Windows global shortcut opens history",
        ROOT / "apps/windows/Program.cs",
        "app.ShowHistory();",
    ),
    (
        "Windows default global shortcut",
        ROOT / "apps/windows/Program.cs",
        "NativeMethods.ModWin | NativeMethods.ModShift",
    ),
    (
        "Linux show-history command",
        ROOT / "apps/linux/src/main.rs",
        '--show-history',
    ),
    (
        "Linux X11 managed shortcut block",
        ROOT / "apps/linux/src/main.rs",
        "fn xbindkeys_block",
    ),
    (
        "Linux X11 default global shortcut",
        ROOT / "apps/linux/src/main.rs",
        "Mod4+Shift + c",
    ),
    (
        "Linux Wayland portal shortcut binding",
        ROOT / "apps/linux/src/main.rs",
        "org.freedesktop.portal.GlobalShortcuts.BindShortcuts",
    ),
    (
        "Linux Wayland activation command mapping",
        ROOT / "apps/linux/src/main.rs",
        'command: "clippo-linux --show-history"',
    ),
    (
        "Linux restricted-shortcut fallback guidance",
        ROOT / "docs/WAYLAND.md",
        "clippo-linux --show-history",
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
    print("Global shortcut source checks failed:")
    for item in missing:
        print(f"- {item}")
    sys.exit(1)

print("Global shortcut source check passed: open-history shortcut paths are present")
PY
