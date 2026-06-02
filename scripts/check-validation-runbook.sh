#!/usr/bin/env bash
set -euo pipefail

runbook_path="${TMPDIR:-/tmp}/clippo-validation-runbook-check.md"
scripts/export-validation-runbook.sh "$runbook_path" >/dev/null

python3 - "$runbook_path" <<'PY'
from pathlib import Path
import re
import shlex
import sys

runbook = Path(sys.argv[1]).read_text(encoding="utf-8")
validation = Path("docs/EXTERNAL_VALIDATION.md").read_text(encoding="utf-8")

if Path("tasks.md").exists():
    source_label = "tasks.md"
    unchecked = [
        match.group(1).strip()
        for match in re.finditer(r"^- \[ \] (.+)$", Path("tasks.md").read_text(encoding="utf-8"), re.MULTILINE)
    ]
else:
    source_label = "docs/EXTERNAL_VALIDATION.md"
    section = validation.split("## Exact Unchecked Task Coverage", 1)[1].split("## README And Portfolio Assets", 1)[0]
    unchecked = [
        match.group(1).strip()
        for match in re.finditer(r"^- (.+)$", section, re.MULTILINE)
    ]

sections = re.findall(r"^### \d+\. (.+)$", runbook, re.MULTILINE)

issues = []
if len(sections) != len(unchecked):
    issues.append(
        f"runbook has {len(sections)} task sections but {source_label} has {len(unchecked)} validation gates"
    )

for task in unchecked:
    if task not in sections:
        issues.append(f"missing runbook section for task: {task}")
    quoted = shlex.quote(task)
    if f"--task {quoted}" not in runbook:
        issues.append(f"missing evidence collection command for task: {task}")

required_groups = [
    "README, screenshots, badges, and portfolio assets",
    "macOS runtime and packaging validation",
    "Windows runtime and packaging validation",
    "Linux runtime, Wayland, and packaging validation",
    "Cross-platform UI, accessibility, and workflow validation",
    "Release milestones and beta validation",
]
for group in required_groups:
    if f"## {group}" not in runbook:
        issues.append(f"missing runbook group: {group}")

if issues:
    print("Validation runbook check failed:")
    for issue in issues:
        print(f"- {issue}")
    sys.exit(1)

print(f"Validation runbook check passed for {len(unchecked)} unchecked gate(s)")
PY
