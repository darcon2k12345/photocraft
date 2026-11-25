---
title: Install Patronus Photo
description: Download Patronus Photo for Windows 10 or 11, macOS or Linux from getartcraft.com or GitHub, or install it with ArtCraft Launcher.
order: 10
verified: 0.8.2
---

The easiest way is the [Patronus Photo page on getartcraft.com](https://getartcraft.com/apps/photocraft). You can also install it with [ArtCraft Launcher](https://getartcraft.com/docs/craft-launcher/guides/install-and-update-apps), which lists Patronus Photo. For a specific build or file format, take it from the [latest release](https://github.com/storytold/photocraft/releases/latest) on GitHub. In the file names below, `<version>` stands for the version number, for example `0.8.2`.

## Windows

Patronus Photo needs Windows 10 or later. On Windows 7 and 8 the installer stops with "Patronus Photo needs Windows 10 or later."

1. Open the [latest release](https://github.com/storytold/photocraft/releases/latest).
2. Download `patronusphoto-<version>-windows-x64.msi`. It fits most PCs (64-bit Intel and AMD).
3. Run the file and follow the installer.

To run without installing, download `patronusphoto-<version>-windows-x64-portable.zip` instead and unzip it.

| Your PC                            | Installer                                | Portable                                          |
|------------------------------------|------------------------------------------|---------------------------------------------------|
| 64-bit Intel or AMD                | `patronusphoto-<version>-windows-x64.msi`   | `patronusphoto-<version>-windows-x64-portable.zip`   |
| Arm (Snapdragon and other Arm PCs) | `patronusphoto-<version>-windows-arm64.msi` | `patronusphoto-<version>-windows-arm64-portable.zip` |
| 32-bit                             | `patronusphoto-<version>-windows-x86.msi`   | `patronusphoto-<version>-windows-x86-portable.zip`   |

The installers and executables are code-signed.

## macOS

1. Download `patronusphoto-<version>-macos-universal.dmg`. It works on Apple silicon and Intel Macs, and is signed and notarized by Apple.
2. Open it and drag Patronus Photo to Applications.

## Linux

Pick one file from the release. Use `x86_64` for Intel and AMD, or `aarch64` for Arm.

| Format              | File                                         | Notes                                                                              |
|---------------------|----------------------------------------------|------------------------------------------------------------------------------------|
| AppImage            | `patronusphoto-<version>-linux-x86_64.AppImage` | Runs anywhere and needs no install.                                                |
| Debian and Ubuntu   | `patronusphoto-<version>-linux-x86_64.deb`      |                                                                                    |
| Fedora and openSUSE | `patronusphoto-<version>-linux-x86_64.rpm`      |                                                                                    |
| Flatpak             | `patronusphoto-<version>-linux-x86_64.flatpak`  | Run `flatpak install --user <file>`, then `flatpak run com.patronus.patronusphoto`. |
| Tarball             | `patronusphoto-<version>-linux-x86_64.tar.gz`   | Unpack it anywhere.                                                                |

The first time you run the AppImage, it adds its icon and menu entry under `~/.local/share`. Set `PATRONUSPHOTO_NO_DESKTOP_INTEGRATION=1` to skip that. For RISC-V, take the `riscv64` file: `patronusphoto-<version>-linux-riscv64.tar.gz`, `.deb` or `.rpm`. The RISC-V tarball needs glibc 2.39 or newer.

On a Wayland session, files dropped on the Patronus Photo window do not open yet. Use **File › Open…** instead, or copy the image in your file manager and paste it with Ctrl+V. To drag and drop, start Patronus Photo under XWayland with `WAYLAND_DISPLAY= patronusphoto`. Patronus Photo shows the right command for your install in a notice until you dismiss it.

## FreeBSD and the web

FreeBSD 14 (x86_64) has a tarball, `patronusphoto-<version>-freebsd-x86_64.tar.gz`, laid out like `/usr/local`. The web version comes as `patronusphoto-web-<version>.zip`, a static site you host on any web server. It runs in a modern browser.

## Check your download

`SHA256SUMS.txt` on the release lists a checksum for every file.

## Next step

Open an image: [Open and save files](../guides/open-and-save-files.md).
