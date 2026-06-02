# Packaging And Distribution

Clippo is still pre-alpha, but Linux packaging paths now exist for tester builds. The `v0.1.1` pre-release publishes Linux AppImage, `.deb`, Flatpak local repo, and checksum artifacts through GitHub Releases. This document records the release packaging decisions and the validation still required before stable distribution.

## Package Identity

- App name: Clippo.
- CLI/helper executable prefix: `clippo`.
- macOS bundle identifier: `app.clippo.Clippo`.
- Windows package identity: `Clippo.Clippo`.
- Linux app id: `app.clippo.Clippo`.

## Distribution Targets

- macOS: signed and notarized app bundle, then Homebrew tap or formula.
- Windows: MSIX or MSI, then winget manifest.
- Linux: AppImage for broad testing, Flatpak if portal integration is viable, `.deb` for Debian/Ubuntu users.
- Snap: not a v1 target unless AppImage, Flatpak, and `.deb` do not cover the audience.

Run `scripts/package-preflight.sh` on a packaging host before building artifacts. It reports which local tools are present for Linux, macOS, Windows, checksum, and release packaging paths.

## macOS Plan

- Build a release app bundle on macOS.
- Local app bundle scaffold: run `scripts/package-macos.sh` on macOS to build `dist/macos/Clippo.app` from the Swift package and `apps/macos/Resources/Info.plist`.
- Sign with an Apple Developer ID certificate.
- Notarize with Apple notary tooling.
- Publish a `.dmg` or zipped app bundle through GitHub Releases.
- Add a Homebrew formula or tap after the first stable artifact URL is available.

## Windows Plan

- Build the Windows shell with Windows App SDK tooling.
- Prefer MSIX if it fits tray/startup behavior; otherwise use MSI.
- MSIX scaffold: run `scripts/package-windows-msix.ps1` from Windows Developer PowerShell.
- Windows package PNG assets live under `packaging/windows/Assets/` and can be regenerated with `node tools/generate-windows-assets.mjs`.
- The initial manifest lives in `packaging/windows/AppxManifest.xml` and uses full-trust desktop execution so tray, clipboard, startup, and paste automation behavior can be tested before choosing a final installer path.
- Sign installer artifacts before v1 where practical.
- Add a winget manifest after stable download URLs and checksums exist.

## Linux Plan

- AppImage: broad early testing artifact. Run `scripts/package-linux-appimage.sh` to prepare an AppDir; it emits an AppImage when `appimagetool` is installed. The AppRun wrapper starts the resident background monitor by default and accepts `--show-history` or `--status` for visible workflow and diagnostic smoke checks.
- Flatpak: preferred if portal integration works cleanly. The manifest lives in `packaging/flatpak/app.clippo.Clippo.yml`; run `scripts/package-linux-flatpak.sh` on a host with `flatpak-builder` to build it. For local install testing, export a repo with `flatpak-builder --repo=dist/linux/flatpak-repo` and install it with `flatpak --user install`.
- `.deb`: useful for Debian/Ubuntu users and direct portfolio demos. Local package scaffold: run `scripts/package-linux-deb.sh` to build `dist/linux/clippo_<version>_amd64.deb`. Desktop launch starts background capture; use the Open History desktop action or `clippo-linux --show-history` for the visible popup and `clippo-linux --status` for terminal diagnostics.
- Snap: deferred unless user demand appears.

## Updates

The initial release uses GitHub Releases for downloads. Auto-update is a post-alpha decision because it affects signing, security, and user trust.

## Signing And Checksums

v1 artifacts should be signed where practical and always published with checksums. Run `scripts/generate-checksums.sh dist` after packaging to write `dist/SHA256SUMS`, or run it against a release-only staging folder to avoid hashing unpacked build directories. The GitHub release workflow stages Rust artifacts under `dist/release` and uploads a `SHA256SUMS` file with each OS artifact bundle.

## SBOM

Generate an SBOM for release artifacts before v1 if the packaging toolchain supports it without major maintenance cost.

## Rollback

Keep the previous stable release available on GitHub Releases. If a release is broken, mark it as such in release notes and publish a patched release instead of mutating artifacts silently.

## GitHub Releases

Release artifacts should be published through GitHub Releases with:

- Versioned tag.
- Release notes.
- Checksums.
- Known limitations.
- Platform support matrix.
- Links to install/uninstall docs.
