# Post-v1 Research Decisions

These topics are intentionally outside v1 unless the roadmap changes.

## Fuzzy Search

Decision: post-v1.

v1 uses fast normalized substring search with ranking. Fuzzy matching can be reconsidered after the core workflow is stable.

## Encrypted History Storage

Decision: post-v1.

Encrypted storage needs platform keychain integration and recovery/error UX. v1 should first ship a clear local-only storage model, ignored sensitive types, and fast clear-history controls.

## Search And Settings Crate Split

Decision: keep search and settings inside `clippo-core` for now.

The code is still small enough that separate crates would add workspace overhead without improving boundaries. Split later if either module grows enough to need independent ownership or dependencies.

## Cloud Sync

Decision: post-v1, opt-in only if ever implemented.

Cloud sync would change Clippo's privacy model from local-first to network-capable. It must not be introduced until local desktop behavior is stable and the encryption, account, deletion, and threat model are designed.

## Per-App History Profiles

Decision: post-v1.

Per-app profiles may be useful, but v1 should focus on global history, ignored applications, and clear privacy controls.

## Temporary Or Private Clipboard Mode

Decision: post-v1.

v1 already includes pause capture and ignore-next-copy. A richer private mode can be evaluated after users validate those simpler controls.

## OCR For Images

Decision: post-v1.

OCR adds CPU cost, platform dependencies, and privacy concerns. It should be opt-in if added.

## Tags Or Folders For Pinned Clips

Decision: post-v1.

Pinned clips should remain simple in v1. Tagging can be evaluated after the core pinning workflow is stable.

## Scripting Or Automation Hooks

Decision: post-v1.

Automation hooks can expose clipboard contents to scripts. They require a separate permission and threat model.

## Plugin Architecture

Decision: post-v1.

Plugins are not needed for v1 parity and would increase maintenance and security scope.

## Mobile Companion

Decision: post-v1.

Clippo is a desktop project for v1. Mobile support would require a separate product and sync model.

## Browser Extension Integration

Decision: post-v1.

Browser integration should wait until the desktop privacy model and permission story are stable.

## Team Or Shared Clipboards

Decision: post-v1, only if clearly opt-in.

Shared clipboards are outside the local-private default. They would require accounts, sharing permissions, encryption, audit controls, and revocation.
