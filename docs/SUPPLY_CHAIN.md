# Supply Chain Review

Current dependency review was performed on 2026-06-01.

## Advisory Review

`cargo audit` completed successfully against `Cargo.lock` after loading the RustSec advisory database. No vulnerable dependencies were reported.

## License Review

Current dependency licenses are compatible with the project policy in `deny.toml`:

- MIT
- Apache-2.0
- Unlicense
- Unicode-3.0
- Zlib

## CI Gates

GitHub Actions includes:

- RustSec advisory audit.
- cargo-deny license/source policy check.

Release builds should repeat this review before publishing artifacts.
