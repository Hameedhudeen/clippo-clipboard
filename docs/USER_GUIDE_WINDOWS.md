# Windows User Guide

Clippo for Windows has a WinForms tray shell scaffold in source. This guide describes the intended user workflow and should be revalidated on a Windows host before the shell is presented as beta-ready.

## Launch

Clippo should run with a system tray icon. Opening the app a second time should focus or open the existing Clippo popup.

## Background And Startup

Closing or minimizing the history popup hides it while the tray process keeps clipboard capture active. Use Exit from the tray menu to stop Clippo. On first launch, Clippo asks whether it should start automatically when you sign in. Preferences include the same Launch at login control, which writes a current-user startup entry.

## Open Clipboard History

Use `Win+Shift+C`, double-click the tray icon, or use the tray menu to open the compact history popup. Search should be focused immediately.

## Select And Paste

- Press Enter or click an item to select it.
- Use `Alt+Enter`, modifier-click, double-click, or numbered shortcuts to paste where Windows input simulation allows it.
- Use paste-without-formatting to paste plain text from rich content.

## Manage History

- Pin or unpin frequent items.
- Delete one item.
- Clear unpinned history.
- Clear all history including pinned items.
- Pause capture.
- Ignore only the next copy.

## Limitations

Paste automation into elevated applications may be restricted by Windows security boundaries.

## Troubleshooting

See `docs/TROUBLESHOOTING.md` and `docs/LIMITATIONS.md`.

## Validation Status

The source includes the tray entry, compact popup, search focus, shortcut column, tooltip previews, footer actions, keyboard shortcuts, Windows Clipboard History coexistence handling, preferences, local history persistence, first-run Launch at login prompt, launch-at-login control, elevated-app paste fallback, hide-instead-of-close popup behavior, and clipboard monitoring. Target-host screenshots, DPI scaling checks, Windows screen reader checks, Windows Clipboard History coexistence testing, and installer validation are still required before this guide can be considered release evidence.
