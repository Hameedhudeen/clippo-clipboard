# Platform Smoke Test Strategy

Use [EXTERNAL_VALIDATION.md](EXTERNAL_VALIDATION.md) to decide when a smoke-test result is strong enough to close remaining OS-specific validation gates.

## macOS

- Launch Clippo from Finder and from login.
- Open history with global shortcut.
- Open history from menu bar icon.
- Copy text, rich text, image, file, and URL content.
- Search immediately after opening the popup.
- Paste normally and without formatting.
- Verify Accessibility permission flow.
- Pin, unpin, delete, clear unpinned, and clear all.
- Verify multi-monitor and Spaces popup placement.
- Verify light mode, dark mode, keyboard focus, and VoiceOver labels.

## Windows

- Launch Clippo from Start and startup at login.
- Open history with global shortcut.
- Open history from tray icon.
- Copy text, rich text, image, file, and URL content.
- Search immediately after opening the popup.
- Paste normally and without formatting.
- Verify behavior with elevated target apps.
- Pin, unpin, delete, clear unpinned, and clear all.
- Verify multi-monitor placement and DPI scaling at 100%, 125%, 150%, and 200%.
- Verify light mode, dark mode, keyboard focus, and screen reader labels.

## Linux

- Test GNOME Wayland, GNOME X11, and KDE.
- Open history with global shortcut where supported.
- Open history from tray/status notifier where supported.
- Copy text, rich text, image, file, and URL content.
- Search immediately after opening the popup.
- Paste normally and without formatting where supported.
- Verify portal fallback messaging on Wayland.
- Pin, unpin, delete, clear unpinned, and clear all.
- Verify multi-monitor placement and fractional scaling.
- Verify light mode, dark mode, keyboard focus, and screen reader labels.
