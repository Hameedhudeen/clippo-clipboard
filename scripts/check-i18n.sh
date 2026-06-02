#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
import json
import re
from pathlib import Path

base = json.loads(Path("i18n/en-US.json").read_text())
base_keys = set(base)
source = Path("crates/clippo-core/src/localization.rs").read_text()
required_keys = set(re.findall(r'"([a-z0-9_.]+)"', source))

missing_required = required_keys - base_keys
if missing_required:
    raise SystemExit(f"i18n/en-US.json missing required keys: {sorted(missing_required)}")

for path in sorted(Path("i18n").glob("*.json")):
    data = json.loads(path.read_text())
    keys = set(data)
    missing = base_keys - keys
    extra = keys - base_keys
    if missing or extra:
        raise SystemExit(f"{path}: missing={sorted(missing)} extra={sorted(extra)}")

print(f"validated {len(list(Path('i18n').glob('*.json')))} locale files")
PY
