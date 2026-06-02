#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is not installed. Install Rust before formatting Clippo." >&2
  exit 127
fi

cargo fmt --all
