# GitHub Milestones And Project Board

This file defines the initial GitHub milestone and project-board structure for Clippo. It is the local source for creating remote GitHub milestones or a GitHub Project after the repository is published.

## Milestones

| Milestone | Goal | Exit Criteria |
| --- | --- | --- |
| `v0.1` | Repository scaffold | Workspace builds locally, docs exist, Maccy attribution is present. |
| `v0.2` | Shared core | History, search, settings, persistence, privacy, lifecycle, onboarding, and platform traits are tested. |
| `v0.3` | First usable shell | One native shell can open, search, copy, paste, pause, ignore next copy, and open preferences. |
| `v0.4` | Cross-platform shell baseline | macOS, Windows, and Linux shells can open, search, copy, and paste items. |
| `v0.5` | Maccy workflow parity | Core Maccy 2.6.1 workflows are implemented or documented as OS-specific differences. |
| `v0.6` | Privacy and settings hardening | Ignore rules, permission prompts, diagnostics redaction, and settings UX are complete. |
| `v0.7` | Packaging | macOS, Windows, and Linux packages exist with checksums and install/uninstall docs. |
| `v0.8` | Landing page and docs | README, screenshots, parity matrix, user docs, and troubleshooting are release-ready. |
| `v0.9` | Beta validation | External users test supported OSes and reported blockers are triaged. |
| `v1.0` | Stable release | Signed or checksummed artifacts, known limitations, privacy review, and no unchecked v1 parity tasks. |
| `post-v1` | Future research | Optional enhancements such as encryption, sync, OCR, tags, scripting, and plugins. |

## Project Board Columns

| Column | Meaning |
| --- | --- |
| Triage | New issues needing scope, labels, milestone, and reproducibility details. |
| Ready | Well-scoped work that can be picked up without major product decisions. |
| In Progress | Actively being implemented. |
| Needs OS Testing | Code exists but requires platform validation on macOS, Windows, Linux X11, or Linux Wayland. |
| Needs Review | PRs or docs waiting for maintainer review. |
| Done | Merged work with checklist and docs updated. |

## Required Labels

- `platform:macos`
- `platform:windows`
- `platform:linux`
- `platform:wayland`
- `type:bug`
- `type:feature`
- `type:task`
- `type:docs`
- `type:release`
- `parity:maccy`
- `privacy`
- `security`
- `performance`
- `accessibility`
- `good first issue`
- `help wanted`
- `needs-design`
- `post-v1`

## Triage Rules

- Every issue must have a milestone or explicit `post-v1` label.
- Maccy parity issues must link to `docs/PARITY.md`.
- Platform issues must include OS version, desktop environment where relevant, and permission state.
- Performance issues must include item count, content type, OS, build type, and benchmark or measurement method.
- Security and privacy issues should avoid pasting clipboard contents into GitHub.
