#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env" 2>/dev/null || true

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is not installed. Install Rust before running Clippo benchmarks." >&2
  exit 127
fi

cargo run --release -p clippo-bench
