# macOS User Guide

Clippo for macOS has a SwiftUI/AppKit shell scaffold in source. This guide describes the intended user workflow and should be revalidated on a macOS host before the shell is presented as beta-ready.

## Launch

Clippo should run as a menu bar app. Opening the app a second time should focus or open the existing Clippo popup.

## Background And Startup

Closing the history popup should not stop clipboard capture; use Quit Clippo from the menu bar item to exit. On first launch, Clippo asks whether it should start automatically when you sign in. Preferences expose the same Launch at Login control through macOS `SMAppService`.

## Open Clipboard History

Use `Shift+Command+C` or the menu bar icon to open the compact history popup. Search should be focused immediately.

## Select And Paste

- Press Enter or click an item to select it.
- Use `Option+Enter`, option-click, or numbered shortcuts to paste the selected item where Accessibility permission allows it.
- Use paste-without-formatting to paste plain text from rich content.

## Manage History

- Pin or unpin frequent items.
- Delete one item.
- Clear unpinned history.
- Clear all history including pinned items.
- Pause capture.
- Ignore only the next copy.

## Permissions

Automatic paste requires macOS Accessibility permission. Clippo should explain why before opening System Settings.

## Troubleshooting

See `docs/TROUBLESHOOTING.md` and `docs/ACCESSIBILITY.md`.

## Validation Status

The source includes the menu bar entry, compact popup, search focus, pinned/history sections, row shortcut hints, native help text for full previews, actions, preferences, local history persistence, first-run Launch at Login prompt, launch-at-login control, sleep/wake handling, and clipboard monitoring. Target-host screenshots, scaling checks, VoiceOver checks, Universal Clipboard behavior, and signed app-bundle validation are still required before this guide can be considered release evidence.
