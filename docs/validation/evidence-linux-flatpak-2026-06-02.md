# Validation Evidence: Package as Flatpak if portal integration is viable.

## Summary

- Task or milestone: Package as Flatpak if portal integration is viable.
- Result: Passed
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
- Package type, if applicable: Flatpak local repo archive

## Steps

1. Ran `scripts/package-linux-flatpak.sh`.
2. Exported a local repo with `flatpak-builder --force-clean --repo=dist/linux/flatpak-repo dist/linux/flatpak-build packaging/flatpak/app.clippo.Clippo.yml`.
3. Updated the local user install with `flatpak --user update -y app.clippo.Clippo`.
4. Launched the installed app with `flatpak --user run app.clippo.Clippo`.
5. Archived `dist/linux/flatpak-repo` and generated `dist/linux-release/SHA256SUMS`.
6. Published the Flatpak local repo archive in GitHub Release `v0.1.0`.

## Evidence

- Screenshot:
- Screen recording:
- Terminal log: Flatpak build, local repo export, user update, and installed app launch completed successfully.
- Package artifact: https://github.com/Hameedhudeen/clippo-clipboard/releases/download/v0.1.0/clippo-0.1.0-flatpak-repo.tar.gz
- Checksum: `2eeb9cd9ab3ba511f8a3d594416ccabdbdb99e7762e69544bd7ade566e3e4ff9  dist/linux-release/clippo-0.1.0-flatpak-repo.tar.gz`
- GitHub issue or release: https://github.com/Hameedhudeen/clippo-clipboard/releases/tag/v0.1.0

## Observations

- Expected behavior: Build a Flatpak directory, export an installable repo, launch the installed app, and publish an archive with checksum.
- Actual behavior: The Flatpak local repo installed and launched successfully from the user installation.
- Known limitations: The validation host did not expose `org.freedesktop.portal.GlobalShortcuts`, so Wayland shortcut activation remains a separate retest gate.
- Follow-up issues: Add AppStream metadata and repeat portal behavior testing on GNOME/KDE hosts that expose GlobalShortcuts.

## Privacy Review

- [x] Evidence uses synthetic clipboard content or redacted real content.
- [x] Screenshots do not expose private file paths, customer data, credentials, or sensitive URLs.
- [x] Logs do not include clipboard contents unless intentionally synthetic test data.
