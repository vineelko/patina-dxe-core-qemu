# QEMU DXE Core Binaries

## Overview

The main purpose of this repository is to integrate the Rust components and dependencies necessary to build a Rust
DXE Core binary that will be used in QEMU firmware.

Currently, two QEMU platforms are supported. The build commands for each are given below.

Set the `RUSTC_BOOTSTRAP` environment variable to `1` in the terminal used for the build.

- Linux: `export RUSTC_BOOTSTRAP=1`
- Windows (cmd): `set RUSTC_BOOTSTRAP=1`
- Windows (powershell): `$env:RUSTC_BOOTSTRAP=1`

---

- **QEMU Q35**: `cargo make q35`
  - Release build: `cargo make q35-release`
- **QEMU SBSA**: `cargo make sbsa`
  - Release build: `cargo make sbsa-release`

The binaries are produced in the `target` directory.

- **QEMU Q35**: `target/x86_64-unknown-uefi`
- **QEMU SBSA**: `target/aarch64-unknown-uefi`

## Working with Local Dependencies

If working with local dependencies outside of this repository, such as making changes in [Patina](https://github.com/OpenDevicePartnership/patina)
that you wish to compile into one of the qemu binaries in this repository, then simply add the path to the local
repository to the command line, and the build tools will automatically patch in all crates in that repository for that
build.

``` cmd
> cargo make q35 C:\\src\\patina\\
> cargo make sbsa C:/src/patina C:/src/patina-paging
```

**IMPORTANT**: This tool temporarily adds the patches to the Cargo.toml, so you must meet Cargo.toml expectations
with the path that you define. That is to say, if you are providing windows pathing, you must use double slashes
(`\\`).

## NuGet Publishing Instructions

The NuGet package is currently published to the public [Patina QEMU DXE Core](https://dev.azure.com/patina-fw/artifacts/_artifacts/feed/qemu-dxe-core)
feed where it is consumed in the [Patina QEMU](https://github.com/OpenDevicePartnership/patina-qemu) repository.

The NuGet is built and published using a GitHub workflow in [Patina QEMU DXE Core Actions](https://github.com/OpenDevicePartnership/patina-dxe-core-qemu/actions).
