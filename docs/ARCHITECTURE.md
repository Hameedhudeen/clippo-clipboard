# Clippo Architecture

Clippo is organized around a shared Rust core and native platform shells.

## Shared Core

The shared core owns behavior that should be consistent across operating systems:

- Clipboard item model.
- History ordering and limits.
- Pinning and deletion.
- Search.
- Settings defaults and validation.
- Privacy and ignore decisions.

## Platform Layer

The platform layer exposes OS capabilities through traits:

- Clipboard access.
- Paste simulation.
- Global shortcuts.
- Tray/menu integration.
- Notifications.
- Autostart.
- Permission checks.

## Native Shells

Native shells should stay thin. Their job is to render OS-appropriate UI, request permissions, translate native events into core commands, and display core state.

## Reference Baseline

Clippo workflow parity is tracked in `docs/PARITY.md`; private local references are not part of the public repository.
