#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env" 2>/dev/null || true

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is not installed. Install Rust before building Clippo." >&2
  exit 127
fi

cargo build --workspace

case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*)
    if ! command -v dotnet >/dev/null 2>&1; then
      echo "dotnet is not installed. Install the .NET desktop workload before building Clippo for Windows." >&2
      exit 127
    fi
    dotnet build apps/windows/Clippo.Windows.csproj
    ;;
  *) echo "Windows native shell packaging requires Windows tooling and is not available on this host." >&2 ;;
esac
