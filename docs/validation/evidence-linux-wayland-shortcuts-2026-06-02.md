# Validation Evidence: Implement global shortcut support for Wayland through available portals where possible.

## Summary

- Task or milestone: Implement global shortcut support for Wayland through available portals where possible.
- Result: Needs retest
- Tester: Codex on Linux validation host
- Date: 2026-06-02
- Clippo commit: 0477f9c7f777b500c0fee84ddde1b7ca843ffae9

## Environment

- Platform: Linux
- OS/version: Zorin OS 18, Linux 6.14.0-37-generic x86_64
- Desktop environment/window manager, if Linux: GNOME
- X11 or Wayland, if Linux: Wayland
- Display scaling: not measured
- Assistive technology used, if applicable: none
- Package type, if applicable: source and installed Flatpak smoke test

## Steps

1. Added `clippo-linux --wayland-shortcuts-daemon`.
2. Ran `scripts/check.sh`, including Linux unit tests for portal command construction, unique portal tokens, and activation filtering.
3. Ran `target/release/clippo-linux --wayland-shortcuts-status`.
4. Ran `target/release/clippo-linux --wayland-shortcuts-daemon`.
5. Launched the installed Flatpak, which reported Wayland shortcut portal status.

## Evidence

- Screenshot:
- Screen recording:
- Terminal log: Source tests passed; local host reported `Wayland GlobalShortcuts portal was not found` and daemon fell back to desktop shortcut guidance.
- Package artifact:
- Checksum:
- GitHub issue or release:

## Observations

- Expected behavior: On a compositor exposing `org.freedesktop.portal.GlobalShortcuts`, the daemon creates a session, binds `open-history`, receives `Activated`, and opens Clippo history.
- Actual behavior: The daemon implementation and tests are present, but this host does not expose the portal, so activation could not be proven.
- Known limitations: GNOME/KDE Wayland target-host evidence is still required before this task can be marked complete.
- Follow-up issues: Retest on a current GNOME or KDE Wayland environment with GlobalShortcuts portal support.

## Privacy Review

- [x] Evidence uses synthetic clipboard content or redacted real content.
- [x] Screenshots do not expose private file paths, customer data, credentials, or sensitive URLs.
- [x] Logs do not include clipboard contents unless intentionally synthetic test data.
