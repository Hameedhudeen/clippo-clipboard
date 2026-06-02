#!/usr/bin/env bash
set -euo pipefail

output_path="${1:-dist/validation-runbook.md}"

python3 - "$output_path" <<'PY'
from collections import OrderedDict
from pathlib import Path
import re
import shlex
import sys

output_path = Path(sys.argv[1])
validation_text = Path("docs/EXTERNAL_VALIDATION.md").read_text(encoding="utf-8")

if Path("tasks.md").exists():
    source = "`tasks.md` and `docs/EXTERNAL_VALIDATION.md`"
    unchecked = [
        match.group(1).strip()
        for match in re.finditer(r"^- \[ \] (.+)$", Path("tasks.md").read_text(encoding="utf-8"), re.MULTILINE)
    ]
else:
    source = "`docs/EXTERNAL_VALIDATION.md`"
    section = validation_text.split("## Exact Unchecked Task Coverage", 1)[1].split("## README And Portfolio Assets", 1)[0]
    unchecked = [
        match.group(1).strip()
        for match in re.finditer(r"^- (.+)$", section, re.MULTILINE)
    ]

if not unchecked:
    raise SystemExit("No unchecked tasks found.")

groups = OrderedDict(
    [
        ("README, screenshots, badges, and portfolio assets", []),
        ("macOS runtime and packaging validation", []),
        ("Windows runtime and packaging validation", []),
        ("Linux runtime, Wayland, and packaging validation", []),
        ("Cross-platform UI, accessibility, and workflow validation", []),
        ("Release milestones and beta validation", []),
    ]
)


def group_for(task: str) -> str:
    lower = task.lower()
    if lower.startswith("v0.") or lower.startswith("v1."):
        return "Release milestones and beta validation"
    if "macos" in lower or "signed app bundle" in lower:
        return "macOS runtime and packaging validation"
    if "windows" in lower or "msix" in lower or "msi" in lower:
        return "Windows runtime and packaging validation"
    if any(keyword in lower for keyword in ["linux", "wayland", "appimage", "flatpak"]):
        return "Linux runtime, Wayland, and packaging validation"
    if any(
        keyword in lower
        for keyword in [
            "screenshot",
            "gif",
            "badge",
            "readme",
            "release downloads",
            "demo video",
            "portfolio",
        ]
    ):
        return "README, screenshots, badges, and portfolio assets"
    return "Cross-platform UI, accessibility, and workflow validation"


def slug(task: str) -> str:
    value = re.sub(r"[^a-z0-9]+", "-", task.lower()).strip("-")
    return value[:72] or "validation"


def evidence_hint(task: str) -> str:
    lower = task.lower()
    for line in validation_text.splitlines():
        if line.startswith("|") and task in line:
            cells = [cell.strip() for cell in line.strip("|").split("|")]
            if len(cells) >= 3:
                return f"Required environment: {cells[1]}; evidence: {cells[2]}"

    aliases = [
        ("readme links", "Verify README renders on GitHub"),
        ("images, badges, and anchors", "Verify README renders on GitHub"),
        ("side-by-side ui review", "Compare popup against Maccy reference"),
        ("popup against maccy", "Compare popup against Maccy reference"),
        ("maccy ui/ux reference", "Compare popup against Maccy reference"),
        ("native system chrome", "Verify native system chrome and conventions"),
        ("screenshot", "Add screenshots or animated GIFs"),
        ("animated gif", "Add screenshots or animated GIFs"),
        ("high-quality readme screenshots", "Add screenshots or animated GIFs"),
        ("release downloads", "Add release downloads section"),
        ("badge", "Add badges"),
        ("readme", "Verify README renders on GitHub"),
        ("demo video", "Add demo video or animated walkthrough"),
        ("portfolio case study", "Add portfolio case study"),
        ("universal clipboard", "Verify Universal Clipboard behavior"),
        ("signed app bundle", "Package signed app bundle"),
        ("msix", "Package as MSIX or MSI"),
        ("msi", "Package as MSIX or MSI"),
        ("wayland", "Wayland global shortcuts through portals"),
        ("appimage", "Package as AppImage"),
        ("flatpak", "Package as Flatpak"),
        ("popup lightweight", "Keep popup lightweight and fast to open"),
        ("screen reader", "Screen reader basics"),
        ("high contrast", "High contrast mode"),
        ("text overlaps", "Text overlap and scaling"),
        ("popup positioning", "Popup positioning"),
        ("global shortcut", "Open history with global shortcut"),
        ("modifier plus enter", "Select and paste with modifier plus Enter"),
        ("modifier plus click", "Select and paste with modifier plus click"),
    ]
    for needle, row_label in aliases:
        if needle in lower:
            for line in validation_text.splitlines():
                if line.startswith("|") and row_label in line:
                    cells = [cell.strip() for cell in line.strip("|").split("|")]
                    if len(cells) >= 3:
                        return f"Required environment: {cells[1]}; evidence: {cells[2]}"

    if lower.startswith("v0.") or lower.startswith("v1."):
        milestone = task.split(" ", 1)[0]
        for line in validation_text.splitlines():
            if line.startswith(f"| {milestone} "):
                cells = [cell.strip() for cell in line.strip("|").split("|")]
                if len(cells) >= 2:
                    return f"Milestone evidence: {cells[1]}"
    return "See docs/EXTERNAL_VALIDATION.md for required evidence."


missing = [task for task in unchecked if task not in validation_text]
if missing:
    for task in missing:
        print(f"Missing external validation coverage for task: {task}", file=sys.stderr)
    raise SystemExit(1)

for task in unchecked:
    groups[group_for(task)].append(task)

lines = [
    "# Clippo Validation Runbook",
    "",
    f"Generated from {source}.",
    "",
    "Use this runbook on target hosts or release infrastructure. Do not check off tasks until evidence exists in `docs/validation/`, a linked GitHub issue, or a release artifact.",
    "",
    "After adding evidence, run:",
    "",
    "```sh",
    "scripts/check-validation-evidence.sh",
    "scripts/check-external-validation.sh",
    "```",
    "",
]

for title, items in groups.items():
    lines.append(f"## {title}")
    lines.append("")
    if not items:
        lines.append("- No remaining gates.")
        lines.append("")
        continue
    for index, task in enumerate(items, start=1):
        output = f"docs/validation/{slug(task)}-YYYY-MM-DD.md"
        command = (
            "scripts/collect-validation-evidence.sh "
            f"--task {shlex.quote(task)} "
            f"--output {shlex.quote(output)}"
        )
        lines.extend(
            [
                f"### {index}. {task}",
                "",
                evidence_hint(task),
                "",
                "Evidence command:",
                "",
                "```sh",
                command,
                "```",
                "",
            ]
        )

output_path.parent.mkdir(parents=True, exist_ok=True)
output_path.write_text("\n".join(lines), encoding="utf-8")
print(f"Exported validation runbook for {len(unchecked)} gate(s) to {output_path}")
PY
