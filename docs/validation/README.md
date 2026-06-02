# Validation Evidence Logs

Use this folder for small, commit-friendly validation logs. Large screenshots, videos, package artifacts, and environment-specific evidence can stay in GitHub issues created from the `Validation evidence` issue template.

## Rules

- Do not include real clipboard contents, private file paths, credentials, customer data, or sensitive URLs.
- Redact copied text in screenshots and logs unless the content is synthetic test data.
- Link large artifacts or GitHub Releases instead of committing binaries here.
- Update the roadmap or validation matrix only after the evidence proves the exact gate being checked.

## Evidence Collector

On a target host, run:

```sh
scripts/collect-validation-evidence.sh --task "Open history with global shortcut"
```

The script writes a timestamped markdown log under `docs/validation/` with OS, desktop/session details, task-specific requirements from `docs/EXTERNAL_VALIDATION.md`, Clippo command output, evidence placeholders, and the privacy review checklist. Use the exact validation gate text from `docs/EXTERNAL_VALIDATION.md` so the evidence can be traced back to the matrix.

To generate a grouped runbook for every remaining external gate, run:

```sh
scripts/export-validation-runbook.sh
```

The default output is `dist/validation-runbook.md`. It contains exact evidence collection commands and required evidence hints for each unchecked task.

Run `scripts/check-validation-runbook.sh` to confirm the generated runbook still contains every unchecked task and an evidence collection command for each one. The standard `scripts/check.sh` command includes this check.

## Evidence Validation

Run:

```sh
scripts/check-validation-evidence.sh
```

The check ignores this README and `TEMPLATE.md`. For committed evidence logs, it verifies required sections, task/result/date/commit/environment fields, and stricter proof requirements for logs marked `Passed`. Evidence logs must reference a current gate or release milestone covered by `docs/EXTERNAL_VALIDATION.md`. A passed log must include at least one evidence link or artifact field and all privacy review checkboxes must be checked.

## Suggested File Names

- `macos-universal-clipboard-YYYY-MM-DD.md`
- `windows-msix-install-YYYY-MM-DD.md`
- `linux-wayland-shortcuts-YYYY-MM-DD.md`
- `ui-parity-review-YYYY-MM-DD.md`
- `accessibility-review-YYYY-MM-DD.md`
- `release-vX.Y.Z-YYYY-MM-DD.md`
