#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import sys

ROOT = Path.cwd()

CHECKS = [
    (
        "macOS active-Space behavior",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "window.collectionBehavior.insert(.moveToActiveSpace)",
    ),
    (
        "macOS transient popup behavior",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "window.collectionBehavior.insert(.transient)",
    ),
    (
        "macOS current screen lookup",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "NSScreen.screens.first",
    ),
    (
        "macOS visible frame placement",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "screen?.visibleFrame",
    ),
    (
        "macOS menu-bar-like top-right offset",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "visibleFrame.maxY - pointAlignedHeight - 16",
    ),
    (
        "macOS fallback centering",
        ROOT / "apps/macos/Sources/ClippoMac/ClippoMacApp.swift",
        "window.center()",
    ),
    (
        "Windows cursor screen lookup",
        ROOT / "apps/windows/Program.cs",
        "Screen.FromPoint(Cursor.Position)",
    ),
    (
        "Windows taskbar-aware working area",
        ROOT / "apps/windows/Program.cs",
        "screen.WorkingArea",
    ),
    (
        "Windows right-edge clamp",
        ROOT / "apps/windows/Program.cs",
        "area.Right - windowSize.Width",
    ),
    (
        "Windows bottom-edge clamp",
        ROOT / "apps/windows/Program.cs",
        "area.Bottom - windowSize.Height",
    ),
    (
        "Windows left/top clamp",
        ROOT / "apps/windows/Program.cs",
        "new Point(Math.Max(area.Left, x), Math.Max(area.Top, y))",
    ),
    (
        "Linux popup placement helper",
        ROOT / "apps/linux/src/main.rs",
        "struct PopupPlacement",
    ),
    (
        "Linux work-area clamp",
        ROOT / "apps/linux/src/main.rs",
        "anchor.x.clamp(work_area.x, work_area.right() - width)",
    ),
    (
        "Linux bottom clamp",
        ROOT / "apps/linux/src/main.rs",
        "anchor.y.clamp(work_area.y, work_area.bottom() - height)",
    ),
    (
        "Linux scale-aligned dimensions",
        ROOT / "apps/linux/src/main.rs",
        "scale.align(preferred_size.width).min(work_area.width)",
    ),
    (
        "Linux placement test inside work area",
        ROOT / "apps/linux/src/main.rs",
        "fn popup_placement_stays_inside_monitor_work_area()",
    ),
    (
        "Linux small-monitor clamp test",
        ROOT / "apps/linux/src/main.rs",
        "fn popup_placement_clamps_to_small_monitors()",
    ),
    (
        "Linux fractional scale test",
        ROOT / "apps/linux/src/main.rs",
        "fn fractional_scale_alignment_keeps_logical_size_stable()",
    ),
    (
        "UI docs state native placement policy",
        ROOT / "docs/UI_UX.md",
        "Popup placement follows native screen/work-area rules",
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
    print("Popup positioning source checks failed:")
    for item in missing:
        print(f"- {item}")
    sys.exit(1)

print("Popup positioning source check passed: platform placement and work-area clamp hooks are present")
PY
