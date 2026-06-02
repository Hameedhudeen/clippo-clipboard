#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env" 2>/dev/null || true

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is not installed. Install Rust before packaging Clippo for Linux." >&2
  exit 127
fi

if ! command -v dpkg-deb >/dev/null 2>&1; then
  echo "dpkg-deb is not installed. Install Debian packaging tools before building a .deb." >&2
  exit 127
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="$(grep -m1 '^version = ' "$repo_root/apps/linux/Cargo.toml" | sed -E 's/version = "([^"]+)"/\1/')"
package_root="$repo_root/dist/linux/deb/clippo_${version}_amd64"
binary_path="$repo_root/target/release/clippo-linux"
deb_path="$repo_root/dist/linux/clippo_${version}_amd64.deb"

cargo build --package clippo-linux --release

rm -rf "$package_root"
mkdir -p \
  "$package_root/DEBIAN" \
  "$package_root/usr/bin" \
  "$package_root/usr/share/applications" \
  "$package_root/usr/share/icons/hicolor/scalable/apps" \
  "$package_root/usr/share/doc/clippo"

install -m 0755 "$binary_path" "$package_root/usr/bin/clippo-linux"
install -m 0644 "$repo_root/assets/icon.svg" "$package_root/usr/share/icons/hicolor/scalable/apps/app.clippo.Clippo.svg"
install -m 0644 "$repo_root/README.md" "$package_root/usr/share/doc/clippo/README.md"
install -m 0644 "$repo_root/LICENSE" "$package_root/usr/share/doc/clippo/LICENSE"

printf '%s\n' \
  '[Desktop Entry]' \
  'Type=Application' \
  'Name=Clippo' \
  'Comment=Native clipboard manager' \
  'Exec=clippo-linux' \
  'Icon=app.clippo.Clippo' \
  'Terminal=false' \
  'Categories=Utility;' \
  'Actions=OpenHistory;PauseCapture;IgnoreNextCopy;ClearUnpinned;ClearAll;Preferences;' \
  '' \
  '[Desktop Action OpenHistory]' \
  'Name=Open History' \
  'Exec=clippo-linux --show-history' \
  '' \
  '[Desktop Action PauseCapture]' \
  'Name=Pause Capture' \
  'Exec=clippo-linux --pause-capture' \
  '' \
  '[Desktop Action IgnoreNextCopy]' \
  'Name=Ignore Next Copy' \
  'Exec=clippo-linux --ignore-next-copy' \
  '' \
  '[Desktop Action ClearUnpinned]' \
  'Name=Clear Unpinned' \
  'Exec=clippo-linux --clear-unpinned' \
  '' \
  '[Desktop Action ClearAll]' \
  'Name=Clear All' \
  'Exec=clippo-linux --clear-all' \
  '' \
  '[Desktop Action Preferences]' \
  'Name=Preferences' \
  'Exec=clippo-linux --preferences' \
  > "$package_root/usr/share/applications/app.clippo.Clippo.desktop"

installed_size="$(du -sk "$package_root/usr" | cut -f1)"
printf '%s\n' \
  'Package: clippo' \
  "Version: $version" \
  'Section: utils' \
  'Priority: optional' \
  'Architecture: amd64' \
  'Maintainer: Clippo Contributors <maintainers@example.invalid>' \
  "Installed-Size: $installed_size" \
  'Depends: libc6' \
  'Description: Lightweight native clipboard manager' \
  ' Clippo is a cross-platform clipboard manager inspired by Maccy.' \
  > "$package_root/DEBIAN/control"

dpkg-deb --build "$package_root" "$deb_path"
echo "Created $deb_path"
