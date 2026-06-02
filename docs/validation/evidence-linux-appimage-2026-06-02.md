# Validation Evidence: Package as AppImage for broad testing.

## Summary

- Task or milestone: Package as AppImage for broad testing.
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
- Package type, if applicable: AppImage

## Steps

1. Ran `scripts/package-preflight.sh`.
2. Ran `APPIMAGE_EXTRACT_AND_RUN=1 scripts/package-linux-appimage.sh`.
3. Ran `APPIMAGE_EXTRACT_AND_RUN=1 dist/linux-release/Clippo-0.1.0-x86_64.AppImage`.
4. Generated `dist/linux-release/SHA256SUMS`.
5. Published the artifact in GitHub Release `v0.1.0`.

## Evidence

- Screenshot:
- Screen recording:
- Terminal log: AppImage package script completed successfully; AppImage launch printed Clippo Linux shell status without crashing.
- Package artifact: https://github.com/Hameedhudeen/clippo-clipboard/releases/download/v0.1.0/Clippo-0.1.0-x86_64.AppImage
- Checksum: `047401adeb5d418373f0497acd74dc6b2f6ad9e820f18818a2c7994a7adb8cda  dist/linux-release/Clippo-0.1.0-x86_64.AppImage`
- GitHub issue or release: https://github.com/Hameedhudeen/clippo-clipboard/releases/tag/v0.1.0

## Observations

- Expected behavior: Build a Linux AppImage, launch it locally, and publish it with a checksum.
- Actual behavior: The AppImage was built, launched, checksummed, and attached to the `v0.1.0` pre-release.
- Known limitations: AppStream metadata is not included yet; this is still a pre-alpha tester artifact.
- Follow-up issues: Add AppStream metadata before stable Linux distribution.

## Privacy Review

- [x] Evidence uses synthetic clipboard content or redacted real content.
- [x] Screenshots do not expose private file paths, customer data, credentials, or sensitive URLs.
- [x] Logs do not include clipboard contents unless intentionally synthetic test data.
