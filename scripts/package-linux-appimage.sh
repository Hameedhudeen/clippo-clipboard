#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env" 2>/dev/null || true

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is not installed. Install Rust before preparing a Linux AppImage." >&2
  exit 127
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="$(grep -m1 '^version = ' "$repo_root/apps/linux/Cargo.toml" | sed -E 's/version = "([^"]+)"/\1/')"
appdir="$repo_root/dist/linux/appimage/Clippo.AppDir"

cargo build --package clippo-linux --release

rm -rf "$appdir"
mkdir -p \
  "$appdir/usr/bin" \
  "$appdir/usr/share/applications" \
  "$appdir/usr/share/icons/hicolor/scalable/apps"

install -m 0755 "$repo_root/target/release/clippo-linux" "$appdir/usr/bin/clippo-linux"
install -m 0644 "$repo_root/assets/icon.svg" "$appdir/usr/share/icons/hicolor/scalable/apps/app.clippo.Clippo.svg"

printf '%s\n' \
  '[Desktop Entry]' \
  'Type=Application' \
  'Name=Clippo' \
  'Comment=Native clipboard manager' \
  'Exec=clippo-linux --show-history' \
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
  > "$appdir/app.clippo.Clippo.desktop"

cp "$appdir/app.clippo.Clippo.desktop" "$appdir/usr/share/applications/app.clippo.Clippo.desktop"
cp "$repo_root/assets/icon.svg" "$appdir/app.clippo.Clippo.svg"

printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  'if [[ "$#" -eq 0 ]]; then' \
  '  exec "$APPDIR/usr/bin/clippo-linux" --show-history' \
  'fi' \
  'exec "$APPDIR/usr/bin/clippo-linux" "$@"' \
  > "$appdir/AppRun"
chmod +x "$appdir/AppRun"

if ! command -v appimagetool >/dev/null 2>&1; then
  echo "Prepared AppDir at $appdir"
  echo "Install appimagetool and rerun this script to produce dist/linux/Clippo-${version}-x86_64.AppImage." >&2
  exit 78
fi

appimagetool "$appdir" "$repo_root/dist/linux/Clippo-${version}-x86_64.AppImage"
