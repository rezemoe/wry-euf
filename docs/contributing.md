# Contributing

WRY Unity is a fork of Tauri WRY. Keep the fork close to upstream where possible,
and keep Unity-specific behavior in the C ABI, Unity package, or clearly marked
platform adapter code.

## Repository layout

```text
src/                    WRY core and platform backends
crates/wry-unity/       Unity C ABI and native handle adapters
unity/                  Unity Package Manager package
android/                Unity-compatible Android library source
docs/                   Documentation site source
```

## Before opening a change

Run:

```sh
cargo fmt --all
cargo check -p wry-unity
cargo test -p wry-unity --no-run
```

If your change affects Android, iOS, or Windows, run the corresponding build on
the platform SDK. A Linux host cannot fully validate those links.

For documentation changes:

```sh
cd docs
pnpm install
pnpm run build
```

## Platform changes

Document the thread that owns each native object. WebView crashes are often caused
by creating or destroying a view on the wrong thread, not by the page itself.

Keep lifecycle behavior explicit. A Unity scene can disappear while an Activity,
window, or view is being recreated, so native cleanup must not depend on a C#
object receiving a finalizer at the right time.

## Documentation style

Write for someone integrating the library for the first time. Prefer a short
explanation and a useful example over a list of internal types. Put platform
requirements on the platform page where a reader will look for them.
