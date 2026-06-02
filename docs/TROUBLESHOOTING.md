# Troubleshooting

This page will grow as platform shells are implemented.

## Global Shortcut Does Not Open Clippo

- Check whether another app already owns the shortcut.
- Try changing the shortcut in preferences once the preferences UI exists.
- On Linux Wayland, check compositor and portal support.

## Clippo Does Not Paste Automatically

- On macOS, confirm Accessibility permission.
- On Windows, check whether the target app is elevated.
- On Linux Wayland, confirm whether the compositor allows paste automation.

## Clipboard Items Are Missing

- Check pause capture.
- Check ignored clipboard types, applications, and content patterns.
- Password managers and transient clipboard types may be ignored by default.

## Stored History Needs To Be Cleared

Use the clear-all or emergency clear-history-and-quit action once platform shells expose it. During development, delete the local JSON store used by the test shell.
