# Install And Uninstall

Clippo does not publish installable artifacts yet. This document records intended v1 install and uninstall behavior.

## macOS

Install:

- Download the signed and notarized app bundle from GitHub Releases.
- Move Clippo to Applications.
- Grant Accessibility permission if automatic paste is enabled.

Uninstall:

- Quit Clippo.
- Remove Clippo from Applications.
- Remove stored Clippo data from the Application Support directory if desired.

## Windows

Install:

- Download the MSIX or MSI from GitHub Releases.
- Run the installer.
- Configure startup at login if desired.

Uninstall:

- Quit Clippo.
- Use Windows Apps settings or the installer uninstaller.
- Remove stored Clippo data from AppData if desired.

## Linux

Install:

- Use AppImage for broad testing, Flatpak where portal integration is viable, or `.deb` for Debian/Ubuntu systems.

Uninstall:

- Quit Clippo.
- Remove the installed package or AppImage.
- Remove stored Clippo data from the XDG data/config directories if desired.
