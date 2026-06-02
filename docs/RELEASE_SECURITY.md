# Release Security Checklist

Before every public release:

- Run `scripts/check.sh`.
- Run `scripts/check-doc-links.sh`.
- Run `scripts/check-i18n.sh`.
- Run `scripts/generate-checksums.sh <artifact-directory>` after package artifacts are built.
- Run `cargo audit`.
- Review `deny.toml` policy results in CI.
- Confirm release artifacts are produced from the expected commit.
- Confirm artifacts have checksums.
- Confirm the release workflow uploaded `SHA256SUMS` beside each staged artifact set.
- Confirm signing/notarization status is documented.
- Confirm logs and diagnostics do not include clipboard contents.
- Confirm privacy docs match current behavior.
- Confirm known platform limitations are listed in release notes.
- Confirm the previous stable release remains available for rollback.

## Secure Deletion Note

Clippo can clear its own stored history, but true secure deletion is not guaranteed on modern filesystems, SSDs, snapshots, or backups. User-facing docs must describe history clearing as best-effort removal from Clippo storage.
