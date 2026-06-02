#!/usr/bin/env bash
set -euo pipefail

artifact_dir="${1:-dist}"
output_file="${2:-$artifact_dir/SHA256SUMS}"

if ! command -v sha256sum >/dev/null 2>&1; then
  echo "sha256sum is not installed. Install GNU coreutils before generating release checksums." >&2
  exit 127
fi

if [[ ! -d "$artifact_dir" ]]; then
  echo "Artifact directory does not exist: $artifact_dir" >&2
  exit 78
fi

mapfile -t artifacts < <(
  find "$artifact_dir" -type f \
    ! -name "$(basename "$output_file")" \
    ! -path '*/.DS_Store' \
    | sort
)

if [[ "${#artifacts[@]}" -eq 0 ]]; then
  echo "No release artifacts found under $artifact_dir" >&2
  exit 78
fi

sha256sum "${artifacts[@]}" > "$output_file"
echo "Created $output_file"
