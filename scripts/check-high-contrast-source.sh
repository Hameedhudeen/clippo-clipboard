#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import re
import sys

ROOT = Path.cwd()

docs = ROOT / "docs" / "UI_UX.md"
macos = ROOT / "apps" / "macos" / "Sources" / "ClippoMac" / "ClippoMacApp.swift"
windows = ROOT / "apps" / "windows" / "Program.cs"
linux = ROOT / "apps" / "linux" / "src" / "main.rs"

required = [
    (docs, "High Contrast Mode", "UI/UX docs define the high-contrast policy"),
    (macos, "TextField(\"Search clipboard history\"", "macOS shell uses native SwiftUI search control"),
    (macos, "List(selection:", "macOS shell uses native SwiftUI list control"),
    (macos, "Form {", "macOS shell uses native SwiftUI preferences form"),
    (windows, "private readonly TextBox search = new();", "Windows shell uses native WinForms search control"),
    (windows, "private readonly ListView list = new();", "Windows shell uses native WinForms list control"),
    (windows, "new FlowLayoutPanel", "Windows shell uses native WinForms layout controls"),
    (linux, "ZenityDialog::search_query", "Linux fallback uses desktop-themed zenity search dialog"),
    (linux, "ZenityDialog::select_history_item", "Linux fallback uses desktop-themed zenity list dialog"),
]

blocked_patterns = [
    (
        macos,
        re.compile(r"\.accessibilityIgnoresInvertColors\s*\(\s*true\s*\)|\.colorInvert\s*\(", re.IGNORECASE),
        "macOS source opts out of system contrast or invert-color behavior",
    ),
    (
        macos,
        re.compile(r"\bNSColor\.(black|white|red|blue|green|yellow|orange|purple|brown|gray)\b", re.IGNORECASE),
        "macOS source uses hardcoded AppKit colors instead of semantic system colors",
    ),
    (
        windows,
        re.compile(r"\bOwnerDraw\s*=\s*true\b|\bDrawItem\s*\+=", re.IGNORECASE),
        "Windows source uses owner-drawn controls without a documented contrast implementation",
    ),
    (
        windows,
        re.compile(r"\bApplication\.RenderWithVisualStyles\s*=\s*false\b", re.IGNORECASE),
        "Windows source disables visual styles that carry system contrast behavior",
    ),
    (
        linux,
        re.compile(r"\bGTK_THEME\s*=|\bAdwStyleManager\b.*force|--class\s+.*dark", re.IGNORECASE),
        "Linux source forces a theme instead of inheriting desktop contrast settings",
    ),
]

issues = []

for path, snippet, description in required:
    if not path.exists():
        issues.append(f"{path.relative_to(ROOT)}: missing file for {description}")
        continue
    text = path.read_text(encoding="utf-8", errors="replace")
    if snippet not in text:
        issues.append(f"{path.relative_to(ROOT)}: missing source evidence: {description}")

for path, pattern, description in blocked_patterns:
    if not path.exists():
        continue
    text = path.read_text(encoding="utf-8", errors="replace")
    for index, line in enumerate(text.splitlines(), start=1):
        if "clippo-high-contrast-reviewed" in line:
            continue
        if pattern.search(line):
            issues.append(f"{path.relative_to(ROOT)}:{index}: {description}: {line.strip()}")

if issues:
    print("High contrast source policy check failed.")
    print("Clippo shells should inherit OS high-contrast behavior through native controls and system colors.")
    print("If a platform-specific exception is required, document it in docs/UI_UX.md and mark the source line with clippo-high-contrast-reviewed.")
    print()
    for issue in issues:
        print(issue)
    sys.exit(1)

print("High contrast source check passed: shell source inherits native contrast-capable controls")
PY
