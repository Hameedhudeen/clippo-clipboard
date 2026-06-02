# Localization

Clippo stores translatable strings in JSON files under `i18n/`.

The shared core exposes the required localization key list in `clippo-core::LOCALIZATION_KEYS`. `scripts/check-i18n.sh` validates that every locale file contains those keys, so native shells can render shared view models without hard-coded English labels.

## v1 Language Scope

- Required for v1: English (`en-US`).
- Community-supported after v1: additional languages through pull requests.

Clippo v1 keeps the language scope intentionally small. The target is to keep all user-facing strings externalized, validate pseudo-localized layouts, and make translation pull requests straightforward after the core cross-platform workflow is stable.

## File Format

Each locale file is a flat JSON object:

```json
{
  "app.name": "Clippo"
}
```

Keys should be stable and descriptive. Values should be plain user-facing strings.

Shared Rust view models should expose localization keys for labels, section titles, settings titles/descriptions, permission/fallback messages, and content-kind labels. Clipboard contents and user-provided file names or URLs remain raw data and should not be translated.

## Pseudo-Localization

`i18n/qps-ploc.json` is reserved for pseudo-localized strings. Native shells should use it during layout checks to catch clipped or overlapping text.

## RTL Review

Right-to-left layout should be reviewed before adding RTL languages. Until then, RTL languages are not v1 release blockers.

## Date And Time Formatting

The shared core exposes locale-aware timestamp formatting for clipboard metadata. Native shells should pass the user's locale and UTC offset when rendering item created/last-used timestamps. The current formatter supports US English, day-first English locales, Japanese numeric ordering, and an ISO-like fallback for other locales until full platform-native date formatting is wired in.

## Contributor Workflow

1. Copy `i18n/en-US.json`.
2. Rename it to the target BCP 47 locale.
3. Translate values only.
4. Run `scripts/check-i18n.sh`.
