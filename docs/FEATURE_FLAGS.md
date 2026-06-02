# Feature Flags

Clippo uses compile-time features to keep platform-specific behavior explicit.

## Core

- `diagnostics`: enables redacted diagnostic snapshot support. Enabled by default.

## Platform

- `macos`: enables macOS-specific platform integration code when it exists.
- `windows`: enables Windows-specific platform integration code when it exists.
- `linux`: enables Linux-specific platform integration code when it exists.

## Persistence

- `json`: enables the initial JSON file store. Enabled by default.

Platform shells should only enable the feature flags they need.
