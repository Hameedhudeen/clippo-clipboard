#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
import re
from pathlib import Path

validation = Path("docs/EXTERNAL_VALIDATION.md").read_text()

if Path("tasks.md").exists():
    source = "tasks.md"
    unchecked = [
        match.group(1).strip()
        for match in re.finditer(r"^- \[ \] (.+)$", Path("tasks.md").read_text(), re.MULTILINE)
    ]
else:
    source = "docs/EXTERNAL_VALIDATION.md"
    section = validation.split("## Exact Unchecked Task Coverage", 1)[1].split("## README And Portfolio Assets", 1)[0]
    unchecked = [
        match.group(1).strip()
        for match in re.finditer(r"^- (.+)$", section, re.MULTILINE)
    ]

missing = [task for task in unchecked if task not in validation]

if missing:
    print("docs/EXTERNAL_VALIDATION.md is missing exact coverage for unchecked tasks:")
    for task in missing:
        print(f"- {task}")
    raise SystemExit(1)

print(f"External validation coverage found for {len(unchecked)} gate(s) from {source}")
PY
