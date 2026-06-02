#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
import re
from collections import OrderedDict
from pathlib import Path

validation = Path("docs/EXTERNAL_VALIDATION.md").read_text()

if Path("tasks.md").exists():
    tasks = Path("tasks.md").read_text()
    checked = re.findall(r"^- \[x\] (.+)$", tasks, re.MULTILINE)
    unchecked = re.findall(r"^- \[ \] (.+)$", tasks, re.MULTILINE)
else:
    checked = []
    section = validation.split("## Exact Unchecked Task Coverage", 1)[1].split("## README And Portfolio Assets", 1)[0]
    unchecked = [
        match.group(1).strip()
        for match in re.finditer(r"^- (.+)$", section, re.MULTILINE)
    ]
covered = [task for task in unchecked if task in validation]
missing = [task for task in unchecked if task not in validation]

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

for task in unchecked:
    lower = task.lower()
    if lower.startswith("v0.") or lower.startswith("v1."):
        groups["Release milestones and beta validation"].append(task)
    elif "macos" in lower or "signed app bundle" in lower:
        groups["macOS runtime and packaging validation"].append(task)
    elif "windows" in lower or "msix" in lower or "msi" in lower:
        groups["Windows runtime and packaging validation"].append(task)
    elif "linux" in lower or "wayland" in lower or "appimage" in lower or "flatpak" in lower:
        groups["Linux runtime, Wayland, and packaging validation"].append(task)
    elif any(
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
        groups["README, screenshots, badges, and portfolio assets"].append(task)
    else:
        groups["Cross-platform UI, accessibility, and workflow validation"].append(task)

print("# Clippo Task Status")
print()
if not Path("tasks.md").exists():
    print("`tasks.md` is not present in this public checkout; reporting public validation gates from `docs/EXTERNAL_VALIDATION.md`.")
    print()
print(f"- Checked tasks: {len(checked)}")
print(f"- Unchecked tasks: {len(unchecked)}")
print(f"- Unchecked tasks covered by external validation matrix: {len(covered)}")
print(f"- Unchecked tasks missing validation coverage: {len(missing)}")
print()

for title, items in groups.items():
    print(f"## {title}")
    if items:
        for item in items:
            print(f"- {item}")
    else:
        print("- None")
    print()

if missing:
    print("## Missing Validation Coverage")
    for item in missing:
        print(f"- {item}")
    raise SystemExit(1)
PY
