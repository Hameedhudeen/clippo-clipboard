# Reproducible Builds

Reproducible builds are a research goal, not a v1 blocker.

## Current Position

The Rust workspace uses `Cargo.lock` and CI checks to make dependency resolution stable. Native app packaging will still depend on platform toolchains, signing systems, and installer formats. Release checksums should be generated with `scripts/generate-checksums.sh` so every published artifact has a reproducible SHA-256 manifest.

## Research Tasks

- Pin native platform toolchain versions in release docs.
- Record build OS images for release workflows.
- Generate checksums for all artifacts with `scripts/generate-checksums.sh`.
- Consider SBOM generation before v1.
- Compare locally built hashes against CI artifacts once packaging exists.
