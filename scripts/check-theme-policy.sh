#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import re
import sys

ROOT = Path.cwd()
SOURCE_ROOTS = [
    ROOT / "apps" / "macos",
    ROOT / "apps" / "windows",
    ROOT / "apps" / "linux",
]
EXTENSIONS = {".swift", ".cs", ".rs", ".xaml", ".ui", ".css"}

PATTERNS = [
    ("hardcoded hex color", re.compile(r"#[0-9a-fA-F]{3,8}\b")),
    ("hardcoded rgb color", re.compile(r"\brgba?\s*\(", re.IGNORECASE)),
    ("forced SwiftUI color scheme", re.compile(r"\b(preferredColorScheme|colorScheme)\s*\(", re.IGNORECASE)),
    ("forced AppKit appearance", re.compile(r"\bNSAppearance\b|\.appearance\s*=", re.IGNORECASE)),
    ("hardcoded WinForms color", re.compile(r"\b(BackColor|ForeColor)\s*=|\bColor\.(Black|White|Red|Blue|Green|Yellow|Orange|Purple|Brown|Gray|LightGray|DarkGray)\b", re.IGNORECASE)),
    ("CSS color override", re.compile(r"\b(background|color)\s*:\s*(#[0-9a-fA-F]{3,8}|rgba?\(|black|white|red|blue|green|yellow|orange|purple|brown|gray)\b", re.IGNORECASE)),
]

ALLOW_MARKER = "clippo-theme-reviewed"


def candidate_files():
    for root in SOURCE_ROOTS:
        if not root.exists():
            continue
        for path in root.rglob("*"):
            if path.is_file() and path.suffix in EXTENSIONS:
                yield path


findings = []
for path in candidate_files():
    text = path.read_text(encoding="utf-8", errors="replace")
    for index, line in enumerate(text.splitlines(), start=1):
        if ALLOW_MARKER in line:
            continue
        for label, pattern in PATTERNS:
            if pattern.search(line):
                findings.append((path.relative_to(ROOT), index, label, line.strip()))

if findings:
    print("Hardcoded or forced theme styling found in platform shell source.")
    print("Clippo should inherit OS light/dark colors through native controls and semantic colors.")
    print(f"If a usage is required, document it in docs/UI_UX.md and add '{ALLOW_MARKER}' on that source line.")
    print()
    for path, line, label, source in findings:
        print(f"{path}:{line}: {label}: {source}")
    sys.exit(1)

print("Theme policy check passed: platform shells do not hardcode light/dark colors")
PY
