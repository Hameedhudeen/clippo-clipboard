#!/usr/bin/env bash
set -euo pipefail

failed=0

while IFS= read -r match; do
  file=${match%%:*}
  link=${match#*:}
  target=${link%%#*}

  [[ -z "$target" ]] && continue
  [[ "$target" =~ ^https?:// ]] && continue
  [[ "$target" =~ ^mailto: ]] && continue

  path=$(dirname "$file")/"$target"
  if [[ ! -e "$path" ]]; then
    echo "Broken markdown link in $file: $target" >&2
    failed=1
  fi
done < <(perl -ne 'while (/\[[^\]]+\]\(([^)]+)\)/g) { print "$ARGV:$1\n" }' $(find . -name '*.md' -not -path './target/*' -not -path './dist/*'))

exit "$failed"
