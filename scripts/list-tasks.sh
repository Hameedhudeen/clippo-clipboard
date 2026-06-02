#!/usr/bin/env bash
set -euo pipefail

if [[ ! -f tasks.md ]]; then
  echo "tasks.md is not present in this public checkout."
  echo "Use docs/EXTERNAL_VALIDATION.md for remaining public validation gates."
  exit 0
fi

checked=$(grep -c '^- \[x\]' tasks.md || true)
unchecked=$(grep -c '^- \[ \]' tasks.md || true)

printf 'checked: %s\n' "$checked"
printf 'unchecked: %s\n' "$unchecked"
