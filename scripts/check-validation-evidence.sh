#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import re
import sys

ROOT = Path.cwd()
VALIDATION_DIR = ROOT / "docs" / "validation"
SKIP = {"README.md", "TEMPLATE.md"}
ALLOWED_RESULTS = {"Passed", "Failed", "Needs retest"}
VALIDATION_TEXT = (ROOT / "docs" / "EXTERNAL_VALIDATION.md").read_text(encoding="utf-8")
TASKS_PATH = ROOT / "tasks.md"
if TASKS_PATH.exists():
    TASKS_TEXT = TASKS_PATH.read_text(encoding="utf-8")
    KNOWN_TASKS = set(re.findall(r"^- \[[ x]\] (.+)$", TASKS_TEXT, re.MULTILINE))
    UNCHECKED_TASKS = set(re.findall(r"^- \[ \] (.+)$", TASKS_TEXT, re.MULTILINE))
else:
    section = VALIDATION_TEXT.split("## Exact Unchecked Task Coverage", 1)[1].split("## README And Portfolio Assets", 1)[0]
    exact_unchecked = set(
        match.group(1).strip()
        for match in re.finditer(r"^- (.+)$", section, re.MULTILINE)
    )
    if "## Completed Evidence Coverage" in VALIDATION_TEXT:
        completed_section = VALIDATION_TEXT.split("## Completed Evidence Coverage", 1)[1].split("## README And Portfolio Assets", 1)[0]
        completed_tasks = set(
            match.group(1).strip()
            for match in re.finditer(r"^- (.+)$", completed_section, re.MULTILINE)
        )
    else:
        completed_tasks = set()
    table_tasks = set(
        match.group(1).strip()
        for match in re.finditer(r"^\| ([^|]+?) \| [^|]+ \| [^|]+ \|$", VALIDATION_TEXT, re.MULTILINE)
        if match.group(1).strip() not in {"Task", "Milestone", "---"}
    )
    KNOWN_TASKS = exact_unchecked | completed_tasks | table_tasks
    UNCHECKED_TASKS = exact_unchecked
MILESTONE_PREFIXES = {"v0.3", "v0.4", "v0.5", "v0.6", "v0.7", "v0.8", "v0.9", "v1.0"}
REQUIRED_HEADINGS = [
    "## Summary",
    "## Environment",
    "## Steps",
    "## Evidence",
    "## Observations",
    "## Privacy Review",
]


def field_value(text: str, label: str) -> str:
    match = re.search(rf"^- {re.escape(label)}:\s*(.*)$", text, re.MULTILINE)
    return match.group(1).strip() if match else ""


def checked(text: str, label: str) -> bool:
    return f"- [x] {label}" in text or f"- [X] {label}" in text


def evidence_values(text: str) -> list[str]:
    labels = [
        "Screenshot",
        "Screen recording",
        "Terminal log",
        "Package artifact",
        "Checksum",
        "GitHub issue or release",
    ]
    return [field_value(text, label) for label in labels]


issues = []
files = sorted(path for path in VALIDATION_DIR.glob("*.md") if path.name not in SKIP)

for path in files:
    text = path.read_text(encoding="utf-8")
    relative = path.relative_to(ROOT)

    for heading in REQUIRED_HEADINGS:
        if heading not in text:
            issues.append(f"{relative}: missing heading `{heading}`")

    task = field_value(text, "Task or milestone")
    result = field_value(text, "Result")
    date = field_value(text, "Date")
    commit = field_value(text, "Clippo commit")
    platform = field_value(text, "Platform")
    os_version = field_value(text, "OS/version")

    if not task:
        issues.append(f"{relative}: missing task or milestone")
    elif task not in KNOWN_TASKS:
        issues.append(f"{relative}: task or milestone is not present in the validation source")
    elif task not in VALIDATION_TEXT:
        issues.append(f"{relative}: task is not covered by docs/EXTERNAL_VALIDATION.md")
    if result not in ALLOWED_RESULTS:
        issues.append(f"{relative}: result must be one of {sorted(ALLOWED_RESULTS)}")
    if not re.fullmatch(r"\d{4}-\d{2}-\d{2}", date):
        issues.append(f"{relative}: date must be YYYY-MM-DD")
    if not commit or commit == "unknown":
        issues.append(f"{relative}: Clippo commit must be recorded")
    if not platform:
        issues.append(f"{relative}: platform must be recorded")
    if not os_version:
        issues.append(f"{relative}: OS/version must be recorded")

    if result == "Passed":
        if not any(value for value in evidence_values(text)):
            issues.append(f"{relative}: passed evidence needs an artifact, issue, release, screenshot, recording, log, or checksum link")
        privacy_labels = [
            "Evidence uses synthetic clipboard content or redacted real content.",
            "Screenshots do not expose private file paths, customer data, credentials, or sensitive URLs.",
            "Logs do not include clipboard contents unless intentionally synthetic test data.",
        ]
        for label in privacy_labels:
            if not checked(text, label):
                issues.append(f"{relative}: passed evidence must check privacy item `{label}`")

if issues:
    print("Validation evidence check failed:")
    for issue in issues:
        print(f"- {issue}")
    sys.exit(1)

print(f"Validation evidence check passed for {len(files)} committed evidence log(s)")
PY
