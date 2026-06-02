#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import sys

ROOT = Path.cwd()

CHECKS = [
    (
        "macOS history window minimum size",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "window.contentMinSize = NSSize(width: 360, height: 420)",
    ),
    (
        "macOS point-aligned popup size",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "let pointAlignedWidth = round(420 * scale) / scale",
    ),
    (
        "macOS row text line limit",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        ".lineLimit(2)",
    ),
    (
        "macOS shortcut column stable width",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        ".frame(width: 20, alignment: .leading)",
    ),
    (
        "Windows per-monitor DPI mode",
        ROOT / "apps/windows/Program.cs",
        "Application.SetHighDpiMode(HighDpiMode.PerMonitorV2)",
    ),
    (
        "Windows history window DPI autoscale",
        ROOT / "apps/windows/Program.cs",
        "AutoScaleMode = AutoScaleMode.Dpi",
    ),
    (
        "Windows list shortcut column width",
        ROOT / "apps/windows/Program.cs",
        'list.Columns.Add("Shortcut", 80)',
    ),
    (
        "Windows list clip column width",
        ROOT / "apps/windows/Program.cs",
        'list.Columns.Add("Clip", 340)',
    ),
    (
        "Windows footer wraps controls",
        ROOT / "apps/windows/Program.cs",
        "WrapContents = true",
    ),
    (
        "Windows long preference text wraps",
        ROOT / "apps/windows/Program.cs",
        "MaximumSize = new Size(360, 0)",
    ),
    (
        "Linux fractional scale alignment helper",
        ROOT / "apps/linux/src/main.rs",
        "struct ScaleFactor(f64)",
    ),
    (
        "Linux popup placement uses scale alignment",
        ROOT / "apps/linux/src/main.rs",
        "let width = scale.align(preferred_size.width).min(work_area.width)",
    ),
    (
        "Linux search dialog width",
        ROOT / "apps/linux/src/main.rs",
        '"420".to_string()',
    ),
    (
        "Linux history dialog width",
        ROOT / "apps/linux/src/main.rs",
        '"520".to_string()',
    ),
    (
        "Linux history dialog height",
        ROOT / "apps/linux/src/main.rs",
        '"560".to_string()',
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
    print("Layout and scaling source checks failed:")
    for item in missing:
        print(f"- {item}")
    sys.exit(1)

print("Layout and scaling source check passed: stable sizing, wrapping, and scale-aware layout hooks are present")
PY
