#!/usr/bin/env bash
set -euo pipefail

status=0

check_command() {
  local name="$1"
  local command="$2"
  local install_note="$3"

  if command -v "$command" >/dev/null 2>&1; then
    printf '[ok] %s: %s\n' "$name" "$(command -v "$command")"
  else
    printf '[missing] %s: %s\n' "$name" "$install_note"
    status=1
  fi
}

printf 'Clippo packaging preflight\n'
printf 'Host: %s\n\n' "$(uname -s)"

check_command "Rust cargo" "cargo" "Install Rust stable."
check_command "SHA-256 checksums" "sha256sum" "Install GNU coreutils."

printf '\nLinux packaging\n'
check_command "Debian package builder" "dpkg-deb" "Install Debian packaging tools."
check_command "AppImage builder" "appimagetool" "Install appimagetool to create AppImage artifacts."
check_command "Flatpak builder" "flatpak-builder" "Install flatpak-builder to validate and build Flatpak artifacts."

printf '\nmacOS packaging\n'
if [[ "$(uname -s)" == "Darwin" ]]; then
  check_command "Swift/Xcode toolchain" "swift" "Install Xcode or Swift."
  check_command "codesign" "codesign" "Install Xcode command line tools."
  check_command "notarytool" "notarytool" "Install Xcode with notarytool."
else
  printf '[skip] macOS packaging requires macOS with Xcode.\n'
fi

printf '\nWindows packaging\n'
if [[ "$(uname -s)" == MINGW* || "$(uname -s)" == MSYS* || "$(uname -s)" == CYGWIN* ]]; then
  check_command ".NET SDK" "dotnet" "Install the .NET desktop workload."
  check_command "Windows SDK makeappx" "makeappx.exe" "Install the Windows SDK and use Developer PowerShell."
else
  printf '[skip] Windows MSIX packaging requires Windows with .NET and Windows SDK.\n'
fi

printf '\n'
if [[ "$status" -eq 0 ]]; then
  printf 'All packaging tools for this host are available.\n'
else
  printf 'Some optional packaging tools are missing on this host.\n'
fi

exit "$status"
