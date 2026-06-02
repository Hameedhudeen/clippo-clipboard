# Validation Evidence: Add badges only after CI, releases, and license metadata are real.

## Summary

- Task or milestone: Add badges only after CI, releases, and license metadata are real.
- Result: Passed
- Tester: Codex on Linux validation host
- Date: 2026-06-02
- Clippo commit: 0477f9c7f777b500c0fee84ddde1b7ca843ffae9

## Environment

- Platform: GitHub repository
- OS/version: GitHub Actions, GitHub Releases, and MIT license metadata on 2026-06-02
- Desktop environment/window manager, if Linux:
- X11 or Wayland, if Linux:
- Display scaling:
- Assistive technology used, if applicable:
- Package type, if applicable: README badges

## Steps

1. Verified `.github/workflows/ci.yml` exists.
2. Published GitHub Release `v0.1.0`.
3. Verified `LICENSE` is MIT.
4. Added CI, release, and MIT license badges to `README.md`.
5. Ran `scripts/check-readme.sh` through `scripts/check.sh`.

## Evidence

- Screenshot:
- Screen recording:
- Terminal log: `scripts/check.sh` passed after badge updates.
- Package artifact:
- Checksum:
- GitHub issue or release: https://github.com/Hameedhudeen/clippo-clipboard/releases/tag/v0.1.0

## Observations

- Expected behavior: README badges point to real CI, release, and license targets.
- Actual behavior: The README now links to the active CI workflow, GitHub Releases, and MIT license file.
- Known limitations: The CI badge reflects the current workflow status; release artifacts are still pre-alpha.
- Follow-up issues: Add platform-specific release badges only after stable cross-platform artifacts exist.

## Privacy Review

- [x] Evidence uses synthetic clipboard content or redacted real content.
- [x] Screenshots do not expose private file paths, customer data, credentials, or sensitive URLs.
- [x] Logs do not include clipboard contents unless intentionally synthetic test data.
