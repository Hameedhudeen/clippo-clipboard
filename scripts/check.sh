#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is not installed. Install Rust before running Clippo checks." >&2
  exit 127
fi

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

scripts/check-doc-links.sh
scripts/check-readme.sh
scripts/check-i18n.sh
scripts/check-ui-motion.sh
scripts/check-native-ui-policy.sh
scripts/check-theme-policy.sh
scripts/check-high-contrast-source.sh
scripts/check-accessibility-source.sh
scripts/check-screen-reader-source.sh
scripts/check-full-preview-source.sh
scripts/check-interaction-source.sh
scripts/check-global-shortcut-source.sh
scripts/check-layout-scaling-source.sh
scripts/check-maccy-ui-parity-source.sh
scripts/check-popup-positioning-source.sh
scripts/check-external-validation.sh
scripts/check-validation-evidence.sh
scripts/check-validation-runbook.sh
