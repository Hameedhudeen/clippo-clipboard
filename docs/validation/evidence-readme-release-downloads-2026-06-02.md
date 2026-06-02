# Validation Evidence: Add release downloads section linking GitHub Releases when artifacts exist.

## Summary

- Task or milestone: Add release downloads section linking GitHub Releases when artifacts exist.
- Result: Passed
- Tester: Codex on Linux validation host
- Date: 2026-06-02
- Clippo commit: 0477f9c7f777b500c0fee84ddde1b7ca843ffae9

## Environment

- Platform: GitHub repository
- OS/version: GitHub Releases and README on 2026-06-02
- Desktop environment/window manager, if Linux:
- X11 or Wayland, if Linux:
- Display scaling:
- Assistive technology used, if applicable:
- Package type, if applicable: GitHub Release

## Steps

1. Published GitHub Release `v0.1.0` as a pre-release.
2. Attached Linux AppImage, Debian package, Flatpak local repo archive, and `SHA256SUMS`.
3. Updated `README.md` with a downloads table linking to the release artifacts.
4. Ran `scripts/check-readme.sh` through `scripts/check.sh`.

## Evidence

- Screenshot:
- Screen recording:
- Terminal log: `scripts/check.sh` passed after README updates.
- Package artifact: https://github.com/Hameedhudeen/clippo-clipboard/releases/tag/v0.1.0
- Checksum: https://github.com/Hameedhudeen/clippo-clipboard/releases/download/v0.1.0/SHA256SUMS
- GitHub issue or release: https://github.com/Hameedhudeen/clippo-clipboard/releases/tag/v0.1.0

## Observations

- Expected behavior: README links to real GitHub Release artifacts and checksums.
- Actual behavior: README links point to the `v0.1.0` pre-release artifact URLs.
- Known limitations: Downloads are Linux-only pre-alpha artifacts; macOS and Windows downloads remain pending.
- Follow-up issues: Update downloads after macOS and Windows artifacts are published.

## Privacy Review

- [x] Evidence uses synthetic clipboard content or redacted real content.
- [x] Screenshots do not expose private file paths, customer data, credentials, or sensitive URLs.
- [x] Logs do not include clipboard contents unless intentionally synthetic test data.
