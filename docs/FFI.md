# FFI Boundary

Clippo uses Rust for shared core behavior and native shells for OS-specific UI.

## Boundary Rule

Native shells should call a narrow command/event API rather than reaching into internal history, search, privacy, or persistence structures directly.

## Core Owns

- Clipboard item model.
- History operations.
- Search.
- Settings validation.
- Privacy and ignore decisions.
- Persistence-facing data structures.
- Command decisions.

## Native Shells Own

- Clipboard API integration.
- Paste simulation.
- Global shortcuts.
- Tray, menu bar, or desktop shell integration.
- Permission prompts.
- Native windows and controls.
- Installer and package integration.

## Data Shape

The FFI layer should expose stable commands, events, settings, and view models. It should not expose Rust collection internals or require native shells to duplicate core rules.

## Initial Implementation

The current Rust crates define the command and platform traits first. Concrete FFI bindings should be added when the first native shell is built.
