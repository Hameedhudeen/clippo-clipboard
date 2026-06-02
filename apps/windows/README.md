# Clippo Windows Shell

This folder is the native Windows shell for Clippo.

## Target

- WinForms native Windows UI for the first shell scaffold, with room to migrate to WinUI 3 later.
- System tray integration.
- core compact history popup adapted to Windows conventions.
- Windows clipboard APIs and input simulation.
- Rust shared core through a narrow FFI boundary.

## Current Status

The shell has a WinForms app target, system tray icon, global hotkey registration, Windows clipboard listener, searchable history popup, keyboard and pointer selection, numbered selection shortcuts, pinned-item shortcut labels, wrapped footer buttons and tray menu items for the common core commands, copy/paste actions through Windows APIs, Windows Clipboard History coexistence safeguards, UAC-aware fallback when the foreground target is elevated, pause/ignore actions, `Ctrl+Comma` preferences access, launch-at-login registration, DPI-aware forms, and tray notifications.

Runtime validation still requires Windows and the .NET desktop workload. Installer packaging and final Windows Clipboard History coexistence testing need Windows-specific validation before release.
