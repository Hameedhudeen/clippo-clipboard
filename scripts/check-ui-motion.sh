#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
import re
from pathlib import Path

patterns = [
    r"\.animation\s*\(",
    r"\bwithAnimation\s*\(",
    r"\bAnyTransition\b",
    r"\bUIView\.animate\b",
    r"\bDoubleAnimation\b",
    r"\bStoryboard\b",
    r"\bBeginAnimation\b",
    r"\bTransitionManager\b",
    r"\bgtk_widget_add_tick_callback\b",
]

paths = [
    Path("apps/macos"),
    Path("apps/windows"),
    Path("apps/linux"),
]

offenders = []
for root in paths:
    for path in root.rglob("*"):
        if path.is_dir() or path.suffix.lower() not in {".swift", ".cs", ".rs"}:
            continue
        text = path.read_text(errors="ignore")
        for pattern in patterns:
            if re.search(pattern, text):
                offenders.append((path, pattern))

if offenders:
    print("Custom UI animation APIs found. Add reduced-motion handling before using them:")
    for path, pattern in offenders:
        print(f"- {path}: {pattern}")
    raise SystemExit(1)

print("No custom UI animation APIs found in platform shells")
PY
