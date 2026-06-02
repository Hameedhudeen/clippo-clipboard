# Validation Evidence: Verify all README links, images, badges, and anchors render correctly on GitHub.

## Summary

- Task or milestone: Verify all README links, images, badges, and anchors render correctly on GitHub.
- Result: Passed
- Tester: Codex on Linux validation host
- Date: 2026-06-02
- Clippo commit: bbdd81cee7ebef2215237fb81e68ef18a9c182c6

## Environment

- Platform: GitHub repository
- OS/version: GitHub README renderer on 2026-06-02
- Desktop environment/window manager, if Linux:
- X11 or Wayland, if Linux:
- Display scaling:
- Assistive technology used, if applicable:
- Package type, if applicable: GitHub-rendered README

## Steps

1. Fetched the remote `README.md` from `Hameedhudeen/clippo-clipboard` through the GitHub API.
2. Rendered it through the GitHub Markdown API in `gfm` mode with repository context.
3. Verified rendered HTML contains badge images, release download table content, and a table tag.
4. Verified all four `v0.1.0` release download URLs are reachable.
5. Ran `scripts/check-readme.sh` through `scripts/check.sh`.

## Evidence

- Screenshot:
- Screen recording:
- Terminal log: GitHub-rendered README contains badges, release table, and 4 reachable release download URLs.
- Package artifact:
- Checksum:
- GitHub issue or release: https://github.com/Hameedhudeen/clippo-clipboard/blob/main/README.md

## Observations

- Expected behavior: README renders on GitHub with working images, badges, anchors, tables, and release links.
- Actual behavior: GitHub-rendered README HTML includes badges and release table content, and the release download URLs respond successfully.
- Known limitations: This is an API-rendered verification rather than a visual screenshot review in a browser.
- Follow-up issues: Repeat visual README review when screenshots or GIFs are added.

## Privacy Review

- [x] Evidence uses synthetic clipboard content or redacted real content.
- [x] Screenshots do not expose private file paths, customer data, credentials, or sensitive URLs.
- [x] Logs do not include clipboard contents unless intentionally synthetic test data.
