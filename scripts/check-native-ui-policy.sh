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
    ("decorative gradient", re.compile(r"\b(LinearGradient|RadialGradient|AngularGradient|LinearGradientBrush|PathGradientBrush)\b|gradient\s*\(", re.IGNORECASE)),
    ("decorative blur/acrylic", re.compile(r"\b(VisualEffectBlur|AcrylicBrush|BackdropMaterial|BlurEffect)\b|\.blur\s*\(|backdrop-filter\s*:", re.IGNORECASE)),
    ("decorative shadow", re.compile(r"\b(DropShadow|ShadowEffect)\b|\.shadow\s*\(|box-shadow\s*:", re.IGNORECASE)),
    ("custom decorative drawing", re.compile(r"\b(Canvas|GraphicsPath|DrawingArea|gtk_drawing_area|set_draw_func|drawRect|OnPaint)\b", re.IGNORECASE)),
    ("pill-shaped decorative chrome", re.compile(r"border-radius\s*:\s*(999|50%)", re.IGNORECASE)),
]

ALLOW_MARKER = "clippo-native-ui-reviewed"


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
    lines = text.splitlines()
    for index, line in enumerate(lines, start=1):
        if ALLOW_MARKER in line:
            continue
        for label, pattern in PATTERNS:
            if pattern.search(line):
                findings.append((path.relative_to(ROOT), index, label, line.strip()))

if findings:
    print("Decorative or custom-drawn UI patterns found in native shell source.")
    print("Clippo should stay close to Maccy's compact utility UI while using native OS controls.")
    print(f"If a usage is intentional, document it in docs/UI_UX.md and add '{ALLOW_MARKER}' on that source line.")
    print()
    for path, line, label, source in findings:
        print(f"{path}:{line}: {label}: {source}")
    sys.exit(1)

print("Native UI policy check passed: no decorative or heavy custom UI patterns found")
PY
