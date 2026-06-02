# Clippo macOS Shell

This folder is the native macOS shell for Clippo.

## Target

- SwiftUI/AppKit.
- Menu bar app.
- core compact history popup.
- Accessibility permission flow for automatic paste.
- Rust shared core through a narrow FFI boundary.

## Current Status

The shell has a SwiftUI `App`, menu bar entry, history window, search-focused popup scaffold, basic row selection, Enter-to-select, numbered item shortcuts, pinned shortcut labels, keyboard commands, NSPasteboard polling, copy/paste actions, compact footer actions for the common core commands, menu actions for pause and ignore-next-copy, Carbon global shortcut registration for `Shift-Command-C`, `Command-Comma` preferences access, `SMAppService` launch-at-login integration, active-Space/current-display popup placement, point-aligned AppKit sizing for Retina and non-Retina displays, native `UserNotifications`, and a preferences scene. Universal Clipboard validation and signed bundle validation still require macOS and Xcode.
