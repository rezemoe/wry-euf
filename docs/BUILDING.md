# Building

This repository contains source code, not prebuilt Unity plugin binaries. Build
the native library for each target and place the result in Unity's plugin folders.

## Requirements

You need:

- Rust 1.85 or newer
- Unity 2021.3 or newer
- A platform SDK for every target you build
- The native WebView development files for desktop builds

The Linux development machine used by this project needs GTK3 and WebKitGTK 4.1.
On Arch Linux:

```sh
sudo pacman -S webkit2gtk-4.1
```

On Debian or Ubuntu:

```sh
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev
```

## Build the Rust bridge

From the repository root:

```sh
cargo fmt --all
cargo check -p wry-unity
cargo build -p wry-unity --release
```

The output names are:

| Platform | Library |
| --- | --- |
| Windows | `wry_unity.dll` |
| Linux | `libwry_unity.so` |
| Android | `libwry_unity.so` for each ABI |
| iOS | `libwry_unity.a` |

## Windows

Build for the Windows target using the Windows SDK and a WebView2-compatible Rust
toolchain. The target machine must have the WebView2 Runtime installed. Windows
10 and Windows 11 generally include it, but production installers should verify
the runtime and provide a repair or bootstrap path.

Place the DLL under the Unity package's Windows plugin directory. Use a windowed
or borderless player. Exclusive fullscreen modes can prevent a child WebView from
appearing above the game.

## Linux

Build the native plugin on a system with WebKitGTK development files. Place the
result in the Linux plugin directory. The Unity player should use X11. When Unity
runs through XWayland, the X11 path normally remains available.

GTK events must be pumped every frame. The included C# manager does this for the
normal package setup.

## Android

The Android build has two parts:

1. Build the Rust shared library for every ABI used by the Unity player.
2. Build the Android library in `android/` and merge its AAR into the Unity app.

The adapter currently uses package name `com.wryunity.webview` and library name
`wry_unity`. Its manifest declares the `INTERNET` permission. The minimum Android
API configured by the adapter is 23.

The AAR must be initialized with Unity's current activity before creating a
WebView. `WryWebViewManager` does this automatically.

## iOS

Build device and simulator static libraries for the architectures used by the
Unity version. Add the static library to `unity/Plugins/iOS` and link
`WebKit.framework`. The included Unity post-build script adds that framework to
the generated Xcode project.

The native bridge uses Unity's exported `UnityGetGLView` symbol. Calls must be
made on the iOS main thread.

## Packaging files in Unity

The source package layout is:

```text
unity/
  Runtime/       C# API and callbacks
  Editor/        Unity build hooks
  Plugins/       native libraries supplied by your build
  Samples~/      optional examples
```

Unity generates `.meta` files when the package is imported. Do not put generated
build output in the Rust source directories.

## GitHub Actions

The `Build native Unity libraries` workflow in `.github/workflows/build-native.yml`
builds the four platform targets and publishes these artifacts:

- `native-windows-x64`
- `native-linux-x64`
- `native-android`
- `native-ios`
- `native-unity-bundle`, containing the Unity plugin layout and C header

It runs for pull requests, the main development branches, version tags, and
manual workflow dispatches. The workflow does not commit binaries to the
repository.
