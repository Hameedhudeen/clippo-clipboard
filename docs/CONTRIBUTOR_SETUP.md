# Contributor Setup

## Prerequisites

- Git.
- Rust stable with `rustfmt` and `clippy`.

Install Rust with rustup:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
rustup component add rustfmt clippy
```

## Local Checks

Run the combined local check suite:

```sh
scripts/check.sh
```

This runs Rust formatting, clippy, tests, documentation link checks, README image and anchor checks, i18n validation, UI motion checks, and external validation coverage.

Format Rust code:

```sh
scripts/format.sh
```

Check task progress:

```sh
scripts/list-tasks.sh
scripts/task-status-report.sh
```

Check packaging host readiness:

```sh
scripts/package-preflight.sh
```

## Project Rules

- Do not log clipboard contents.
- Keep platform-specific behavior behind platform traits or native shells.
- Update the roadmap, documentation, or validation matrix when work is actually completed.
- Use the documented Clippo workflow scope as the v1 parity baseline unless the baseline is intentionally updated.
