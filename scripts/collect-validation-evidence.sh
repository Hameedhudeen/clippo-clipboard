#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/collect-validation-evidence.sh --task "Task text" [--result Passed|Failed|Needs-retest] [--platform name] [--output path]

Creates a commit-friendly validation evidence log with target-host environment
details and optional Clippo command output. Use synthetic clipboard contents only.
EOF
}

task=""
result="Needs retest"
platform=""
output=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --task)
      task="${2:-}"
      shift 2
      ;;
    --result)
      result="${2:-}"
      shift 2
      ;;
    --platform)
      platform="${2:-}"
      shift 2
      ;;
    --output)
      output="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$task" ]]; then
  echo "--task is required" >&2
  usage >&2
  exit 2
fi

if [[ -z "$platform" ]]; then
  platform="$(uname -s 2>/dev/null || echo unknown)"
fi

date_utc="$(date -u +%Y-%m-%d)"
slug="$(printf '%s' "$task" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9]+/-/g; s/^-//; s/-$//' | cut -c1-72)"
if [[ -z "$output" ]]; then
  output="docs/validation/${slug}-${date_utc}.md"
fi

mkdir -p "$(dirname "$output")"

clippo_commit="$(git rev-parse --short HEAD 2>/dev/null || echo unknown)"
os_name="$(uname -a 2>/dev/null || echo unknown)"
desktop="${XDG_CURRENT_DESKTOP:-${DESKTOP_SESSION:-}}"
session_type="${XDG_SESSION_TYPE:-}"
display_scaling="${GDK_SCALE:-${QT_SCALE_FACTOR:-unknown}}"

mapfile -t validation_hint < <(python3 - "$task" <<'PY'
from pathlib import Path
import sys

task = sys.argv[1]
validation_text = Path("docs/EXTERNAL_VALIDATION.md").read_text(encoding="utf-8")


def row_hint(row_label: str):
    for line in validation_text.splitlines():
        if line.startswith("|") and row_label in line:
            cells = [cell.strip() for cell in line.strip("|").split("|")]
            if len(cells) >= 3:
                return cells[1], cells[2]
    return None


def evidence_hint(task: str):
    lower = task.lower()
    exact = row_hint(task)
    if exact:
        return exact

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
            hint = row_hint(row_label)
            if hint:
                return hint

    if lower.startswith("v0.") or lower.startswith("v1."):
        milestone = task.split(" ", 1)[0]
        for line in validation_text.splitlines():
            if line.startswith(f"| {milestone} "):
                cells = [cell.strip() for cell in line.strip("|").split("|")]
                if len(cells) >= 2:
                    return "Release milestone evidence", cells[1]

    return "See docs/EXTERNAL_VALIDATION.md", "Record evidence that proves the exact unchecked task."


required_environment, required_evidence = evidence_hint(task)
print(required_environment)
print(required_evidence)
PY
)
required_environment="${validation_hint[0]:-See docs/EXTERNAL_VALIDATION.md}"
required_evidence="${validation_hint[1]:-Record evidence that proves the exact unchecked task.}"

command_log=""
if command -v clippo-linux >/dev/null 2>&1; then
  command_log+="clippo-linux status:\n"
  command_log+="$(clippo-linux 2>&1 || true)\n\n"
  command_log+="clippo-linux --wayland-shortcuts-status:\n"
  command_log+="$(clippo-linux --wayland-shortcuts-status 2>&1 || true)\n"
elif [[ -x target/debug/clippo-linux ]]; then
  command_log+="target/debug/clippo-linux status:\n"
  command_log+="$(target/debug/clippo-linux 2>&1 || true)\n\n"
  command_log+="target/debug/clippo-linux --wayland-shortcuts-status:\n"
  command_log+="$(target/debug/clippo-linux --wayland-shortcuts-status 2>&1 || true)\n"
else
  command_log="Clippo executable was not found on PATH or at target/debug/clippo-linux."
fi

cat >"$output" <<EOF
# Validation Evidence: ${task}

## Summary

- Task or milestone: ${task}
- Result: ${result}
- Tester:
- Date: ${date_utc}
- Clippo commit: ${clippo_commit}

## Environment

- Platform: ${platform}
- OS/version: ${os_name}
- Desktop environment/window manager, if Linux: ${desktop:-unknown}
- X11 or Wayland, if Linux: ${session_type:-unknown}
- Display scaling: ${display_scaling}
- Assistive technology used, if applicable:
- Package type, if applicable:

## Task-Specific Requirements

- Required environment: ${required_environment}
- Required evidence: ${required_evidence}

## Steps

1. Build or install Clippo for the target host.
2. Run the task-specific smoke steps from docs/EXTERNAL_VALIDATION.md for the required environment above.
3. Attach or link screenshots, screen recordings, logs, artifacts, checksums, or release URLs.

## Command Output

\`\`\`text
$(printf '%b' "$command_log")
\`\`\`

## Evidence

- Screenshot:
- Screen recording:
- Terminal log:
- Package artifact:
- Checksum:
- GitHub issue or release:

## Observations

- Expected behavior:
- Actual behavior:
- Known limitations:
- Follow-up issues:

## Privacy Review

- [ ] Evidence uses synthetic clipboard content or redacted real content.
- [ ] Screenshots do not expose private file paths, customer data, credentials, or sensitive URLs.
- [ ] Logs do not include clipboard contents unless intentionally synthetic test data.
EOF

echo "Wrote validation evidence log to $output"
