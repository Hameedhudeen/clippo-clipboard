# Validation Evidence: Add badges only after CI and releases exist.

## Summary

- Task or milestone: Add badges only after CI and releases exist.
- Result: Passed
- Tester: Codex on Linux validation host
- Date: 2026-06-02
- Clippo commit: 0477f9c7f777b500c0fee84ddde1b7ca843ffae9

## Environment

- Platform: GitHub repository
- OS/version: GitHub Actions and GitHub Releases on 2026-06-02
- Desktop environment/window manager, if Linux:
- X11 or Wayland, if Linux:
- Display scaling:
- Assistive technology used, if applicable:
- Package type, if applicable: README badges

## Steps

1. Verified `.github/workflows/ci.yml` exists.
2. Published GitHub Release `v0.1.0`.
3. Added README badges for CI, release, and license.
4. Ran `scripts/check-readme.sh` through `scripts/check.sh`.

## Evidence

- Screenshot:
- Screen recording:
- Terminal log: `scripts/check.sh` passed after badge updates.
- Package artifact:
- Checksum:
- GitHub issue or release: https://github.com/Hameedhudeen/clippo-clipboard/releases/tag/v0.1.0

## Observations

- Expected behavior: README badge area is backed by real CI and release targets.
- Actual behavior: CI and release badges link to real GitHub resources.
- Known limitations: The release badge currently reflects a pre-release.
- Follow-up issues: Revisit badges after the first stable cross-platform release.

## Privacy Review

- [x] Evidence uses synthetic clipboard content or redacted real content.
- [x] Screenshots do not expose private file paths, customer data, credentials, or sensitive URLs.
- [x] Logs do not include clipboard contents unless intentionally synthetic test data.
