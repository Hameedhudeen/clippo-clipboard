#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env" 2>/dev/null || true

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is not installed. Install Rust before building Clippo." >&2
  exit 127
fi

cargo build --workspace

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "macOS native shell packaging requires macOS/Xcode and is not available on this host." >&2
  exit 0
fi

swift build --package-path apps/macos --configuration debug
