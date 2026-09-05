# Development Notes

This repository is a fork of Tauri WRY with a Unity integration layer.

- WRY core is at the repository root.
- The Unity C ABI is `crates/wry-unity`.
- The Unity Package Manager package is `unity/`.
- Android Kotlin/AAR sources are under `android/`.
- Platform integration notes are in `docs/`.

Run `cargo fmt --all` and `cargo check -p wry-unity` for the native Linux slice.
Desktop WebView calls must run on Unity's main thread. Android and iOS builds
require their platform SDKs and cannot be fully linked on a Linux host.
