#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env" 2>/dev/null || true

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is not installed. Install Rust before preparing a Flatpak build." >&2
  exit 127
fi

if ! command -v flatpak-builder >/dev/null 2>&1; then
  echo "flatpak-builder is not installed. Install Flatpak build tooling before building Clippo as a Flatpak." >&2
  exit 78
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
build_dir="$repo_root/dist/linux/flatpak-build"
manifest="$repo_root/packaging/flatpak/app.clippo.Clippo.yml"

cargo build --package clippo-linux --release

flatpak-builder \
  --force-clean \
  "$build_dir" \
  "$manifest"

echo "Built Flatpak directory at $build_dir"
