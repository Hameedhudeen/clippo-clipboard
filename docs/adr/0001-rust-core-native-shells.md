# ADR 0001: Rust Core With Native Platform Shells

## Status

Accepted.

## Context

Clippo targets a low-memory, native-feeling clipboard manager for macOS, Windows, and Linux. The app should reuse core behavior across platforms while allowing each OS to expose familiar UI, permissions, shortcuts, and tray/menu patterns.

## Decision

Use a Rust shared core for clipboard history, search, settings, persistence, privacy rules, and platform-neutral command handling. Build thin native shells around that core:

- macOS: SwiftUI/AppKit.
- Windows: WinUI 3 or equivalent native Windows UI.
- Linux: GTK4/libadwaita unless platform research rejects it.

## Consequences

- Shared logic can be tested without desktop UI dependencies.
- Platform shells can feel native and respect OS-specific constraints.
- FFI and packaging complexity are accepted as the cost of low memory usage and native behavior.
