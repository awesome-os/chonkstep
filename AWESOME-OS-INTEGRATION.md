# Awesome-OS Integration — `chonkstep`

This repository is the **Awesome-OS** upstream mirror and integration package of [`chriskearney/chonkstep`](https://github.com/chriskearney/chonkstep), a performance-focused floating Wayland compositor and window manager written in Rust, built as a drop-in replacement for Hyprland on Omarchy.

All upstream Rust crates, vendored Smithay sources, Omarchy plugins, themes, documentation, and the original **GNU General Public License v3.0** (`LICENSE`) are preserved intact.

## Awesome-OS Ecosystem Links

| Repository | Role |
| :--- | :--- |
| [`awesome-os/t2desktop-omarchy`](https://github.com/awesome-os/t2desktop-omarchy) | Omarchy desktop environment and session integration layer for Awesome-OS |
| [`awesome-os/noctalia`](https://github.com/awesome-os/noctalia) | Wayland desktop shell, bar, launcher, and notification ecosystem |
| [`awesome-os/explorer`](https://github.com/awesome-os/explorer) | Awesome-OS desktop file manager and workspace explorer |

## B8G ABI v3 Zero-Copy Bridge

- [`src/b8g_abi.rs`](src/b8g_abi.rs) provides the Rust `#[repr(C, align(64))]` `B8G_ABI_MAGIC_V3` (`0x33564742u32`), `B8gArenaHeaderV1`, and `LibsectiumDisplaySurfaceV1` (`libsectium_display` surface descriptor) bridge with unit tests verifying 64-byte alignment and magic invariants.

## Integration Notes

- **Compositor Role:** Drop-in floating Wayland compositor implementing Hyprland IPC request/event sockets and `~/.config/hypr/` settings for Omarchy sessions.
- **License:** GPL-3.0-only (see [`LICENSE`](LICENSE)).
- **Upstream Source:** <https://github.com/chriskearney/chonkstep>
- **CI Validation:** [`.github/workflows/awesome-os-ci.yml`](.github/workflows/awesome-os-ci.yml) runs `rustc --test src/b8g_abi.rs && ./b8g_abi` and validates `Cargo.toml` workspace metadata and repository layout on `ubuntu-latest`.
