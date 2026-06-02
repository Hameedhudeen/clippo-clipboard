#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env" 2>/dev/null || true

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "macOS app bundle packaging requires macOS." >&2
  exit 78
fi

if ! command -v swift >/dev/null 2>&1; then
  echo "swift is not installed. Install Xcode or Swift before packaging Clippo for macOS." >&2
  exit 127
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
app_root="$repo_root/dist/macos/Clippo.app"
contents="$app_root/Contents"
macos_dir="$contents/MacOS"
resources_dir="$contents/Resources"

swift build \
  --package-path "$repo_root/apps/macos" \
  --configuration release

binary_path="$repo_root/apps/macos/.build/release/ClippoMac"
if [[ ! -x "$binary_path" ]]; then
  echo "expected macOS executable not found at $binary_path" >&2
  exit 1
fi

rm -rf "$app_root"
mkdir -p "$macos_dir" "$resources_dir"

cp "$binary_path" "$macos_dir/ClippoMac"
cp "$repo_root/apps/macos/Resources/Info.plist" "$contents/Info.plist"
cp "$repo_root/assets/icon.svg" "$resources_dir/icon.svg"

echo "Created $app_root"
echo "Signing and notarization are intentionally handled by the release workflow."
