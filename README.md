# pkgctrl

Package controller. For when you want reconciling what packages you currently have installed on your system. Currenly only supporting pacman, though plans to expand to go, cargo & krew package managers in the future

## Usecase

I have multiple machines, where I randomly install & uninstall packages. Sometimes I'd like to sync state across machines,
or when one dies restore from a backup.

## Quick start

```bash
yay -S pkgctrl-bin
```

Optionally you can install via cargo:

```bash
cargo install pkgctrl
```

* Create example config.yml:

```yaml
want:
  - bat
ignore:
ignore_groups:
  - gnome
  - kde-applications
  - kde-graphics
  - kde-multimedia
  - kde-network
  - kde-system
  - kde-utilities
  - kf5
  - linux510-extramodules
  - linux54-extramodules
  - manjaro-tools
  - plasma
  - qt
  - qt5
  - xorg
  - xorg-apps
  - xorg-drivers
  - base-devel
```

sync your currently installed packages to the file:
```bash
pkgctrl sync-config --config config.yml
```

Inspect the file, edit as appropriate.

...

some time passes

...

Reconcile the state after backup/on different machine

```bash
pkgctrl reconcile --config config.yaml
```
