#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
import re
from pathlib import Path

readme = Path("README.md")
text = readme.read_text()

def slugify(heading: str) -> str:
    heading = heading.strip().lower()
    heading = re.sub(r"`([^`]*)`", r"\1", heading)
    heading = re.sub(r"[^a-z0-9\s-]", "", heading)
    heading = re.sub(r"\s+", "-", heading)
    return heading.strip("-")

anchors = {
    slugify(match.group(1))
    for match in re.finditer(r"^#+\s+(.+)$", text, re.MULTILINE)
}

for target in re.findall(r"!\[[^\]]*\]\(([^)]+)\)", text):
    if target.startswith(("http://", "https://")):
        continue
    if not Path(target).exists():
        raise SystemExit(f"Missing README image: {target}")

for target in re.findall(r"(?<!!)\[[^\]]+\]\(([^)]+)\)", text):
    if target.startswith(("http://", "https://", "mailto:")):
        continue
    if "#" in target:
        path, anchor = target.split("#", 1)
    else:
        path, anchor = target, ""
    if path and not Path(path).exists():
        raise SystemExit(f"Missing README link target: {target}")
    if not path and anchor and anchor not in anchors:
        raise SystemExit(f"Missing README anchor: {anchor}")

print("README local images and anchors validated")
PY
