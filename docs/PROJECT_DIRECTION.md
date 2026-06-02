# Project Direction

Clippo is a separate open-source project inspired by Maccy. It is not an official Maccy port and does not target upstream Maccy PRs for the cross-platform rewrite.

## v1 Goal

v1 should deliver Maccy-like clipboard manager workflows on macOS, Windows, and Linux using native platform conventions where exact behavior is not possible.

## UI/UX Target

Clippo should feel familiar to Maccy users:

- Compact popup for clipboard history.
- Search-first interaction when the popup opens.
- Keyboard-first navigation and selection.
- Pinned clips visually and behaviorally above regular history.
- Shortcut hints visible for fast actions.
- Menu bar, tray, or desktop shell actions equivalent to Maccy's menu actions.
- Tooltip or native equivalent for delayed full-content preview.

Clippo should not become a generic clipboard dashboard, notes app, or sync product in v1.

## Native-Feeling Criteria

- macOS should use SwiftUI/AppKit conventions, menu bar behavior, macOS keyboard labels, and system permission prompts.
- Windows should use native Windows windowing, tray behavior, WinUI-style controls, Windows shortcut labels, and DPI scaling behavior.
- Linux should use GTK4/libadwaita conventions for GNOME-first behavior while documenting KDE and Wayland differences.
- UI should be compact, fast to open, accessible by keyboard, and visually restrained.
- Platform differences must be documented instead of hidden.

## Supported OS Targets

- macOS Sonoma 14 or newer for v1 unless implementation research proves a lower version is safe.
- Windows 11 for v1, with Windows 10 support treated as best-effort until tested.
- Ubuntu 24.04 GNOME Wayland and X11 for Linux v1 validation.
- KDE Plasma on a current Linux distribution for secondary Linux validation.

## v1 Non-Goals

- Cloud sync.
- Mobile apps.
- Browser extensions.
- Team or shared clipboards.
- Plugin system.
- OCR.
- AI features.
- Pixel-perfect Maccy cloning across non-macOS platforms.
- Telemetry by default.

## Maintainer Expectations

- Keep the public roadmap and validation matrix aligned with actual implementation status.
- Treat privacy, low memory usage, and documented limitations as release gates.
- Triage platform issues by OS and desktop environment.
- Prefer small, reviewable PRs.
- Re-audit Maccy before v1 so parity claims remain current.
