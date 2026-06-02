#!/usr/bin/env bash
set -euo pipefail

artifact_dir="${1:-dist}"
output_file="${2:-$artifact_dir/SHA256SUMS}"

checksum_command=()
if command -v sha256sum >/dev/null 2>&1; then
  checksum_command=(sha256sum)
elif command -v shasum >/dev/null 2>&1; then
  checksum_command=(shasum -a 256)
else
  echo "sha256sum or shasum is required before generating release checksums." >&2
  exit 127
fi

if [[ ! -d "$artifact_dir" ]]; then
  echo "Artifact directory does not exist: $artifact_dir" >&2
  exit 78
fi

artifacts=()
while IFS= read -r artifact; do
  artifacts+=("$artifact")
done < <(
  find "$artifact_dir" -type f \
    ! -name "$(basename "$output_file")" \
    ! -path '*/.DS_Store' \
    | sort
)

if [[ "${#artifacts[@]}" -eq 0 ]]; then
  echo "No release artifacts found under $artifact_dir" >&2
  exit 78
fi

"${checksum_command[@]}" "${artifacts[@]}" > "$output_file"
echo "Created $output_file"
