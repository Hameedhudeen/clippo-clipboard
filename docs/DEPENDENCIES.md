# Dependency Policy

Clippo dependencies should support the project's low-memory, privacy-first goals.

## Rules

- Prefer small, actively maintained libraries.
- Prefer permissive licenses compatible with MIT.
- Avoid libraries that add network behavior unless the feature explicitly needs it.
- Avoid logging or telemetry dependencies by default.
- Keep platform-specific dependencies isolated behind platform crates or shells.
- Review licenses and security advisories before public releases.
