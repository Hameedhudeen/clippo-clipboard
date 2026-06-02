# Contributing to Clippo

Clippo is currently in pre-alpha. The shared Rust core and native shell scaffolds are in place, and contributions are welcome when they stay aligned with `tasks.md` and the platform parity plan.

## Current Priorities

1. Keep the shared Rust core small, tested, and platform-neutral.
2. Preserve native-feeling behavior per OS instead of forcing one UI model everywhere.
3. Treat privacy, low memory usage, and clear platform limitations as release blockers.
4. Keep `tasks.md` aligned with real implementation status.

## Development Expectations

- Open an issue before large architectural changes.
- Keep changes focused and easy to review.
- Add tests for shared core behavior.
- Add focused tests for platform shell helpers when runtime UI testing is not available.
- Do not log clipboard contents.
- Document OS-specific limitations instead of hiding them.

See `docs/CONTRIBUTOR_SETUP.md` for local setup and check commands.

## Reference Project

Clippo is a separate community open-source project. Keep contributions focused on the documented workflow, privacy, performance, and native-platform goals.
