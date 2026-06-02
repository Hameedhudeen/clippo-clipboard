#!/usr/bin/env bash
set -euo pipefail

output_dir="${1:-dist/validation-issues}"

python3 - "$output_dir" <<'PY'
import re
import sys
from pathlib import Path

output_dir = Path(sys.argv[1])
validation = Path("docs/EXTERNAL_VALIDATION.md").read_text()

if Path("tasks.md").exists():
    unchecked = [
        match.group(1).strip()
        for match in re.finditer(r"^- \[ \] (.+)$", Path("tasks.md").read_text(), re.MULTILINE)
    ]
else:
    section = validation.split("## Exact Unchecked Task Coverage", 1)[1].split("## README And Portfolio Assets", 1)[0]
    unchecked = [
        match.group(1).strip()
        for match in re.finditer(r"^- (.+)$", section, re.MULTILINE)
    ]

if not unchecked:
    raise SystemExit("No unchecked tasks found.")

output_dir.mkdir(parents=True, exist_ok=True)

index_lines = ["# Validation Issue Drafts", ""]

for index, task in enumerate(unchecked, start=1):
    if task not in validation:
        raise SystemExit(f"Missing external validation coverage for task: {task}")

    slug = re.sub(r"[^a-z0-9]+", "-", task.lower()).strip("-")[:72]
    path = output_dir / f"{index:02d}-{slug}.md"
    body = f"""# Validation: {task}

## Validation Area

- Task or milestone: {task}
- Platform:
- OS/version:
- Desktop environment/window manager, if Linux:
- X11 or Wayland, if Linux:
- Clippo commit:

## Evidence

Attach or link the relevant screenshot, screen recording, terminal log, package artifact, checksum file, or release URL.

## Steps Run

1.
2.
3.

## Result

- [ ] Passed
- [ ] Failed
- [ ] Needs retest

## Notes

Do not paste sensitive clipboard contents into this issue. Redact sample clip text, file paths, URLs, and app names when needed.
"""
    path.write_text(body)
    index_lines.append(f"- [{task}]({path.name})")

(output_dir / "README.md").write_text("\n".join(index_lines) + "\n")
print(f"Exported {len(unchecked)} validation issue drafts to {output_dir}")
PY
