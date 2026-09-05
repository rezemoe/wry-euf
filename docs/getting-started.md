# Getting Started

This guide takes you from a clean checkout to a WebView component in a Unity
player. It assumes you are using the source package in this repository. The
native libraries still need to be built for your target platform.

## 1. Build the native bridge

Install the desktop dependencies described in [Building](./BUILDING), then run:

```sh
cargo build -p wry-unity --release
```

For desktop development, the important files are:

```text
target/release/wry_unity.dll       Windows
target/release/libwry_unity.so    Linux
```

The Android and iOS outputs are produced by their platform-specific toolchains.

## 2. Add the Unity package

Open Unity's Package Manager, select **Add package from disk**, and choose
`unity/package.json`. For a project that tracks packages in source control, copy
the package into `Packages/com.wry.unity` instead.

Unity will create metadata files when it imports the package. Keep the package
folder in the project while testing; do not use a symlink on platforms where
Unity's native plugin importer cannot follow it.

## 3. Add the native library

Place the correct native library under the package's plugin folders. The exact
folder can vary between projects, but the platform and architecture must match
Unity's importer settings.

Use these names:

| Target | File |
| --- | --- |
| Windows x64 | `wry_unity.dll` |
| Linux x64 | `libwry_unity.so` |
| Android | `libwry_unity.so` inside the AAR for each ABI |
| iOS | `libwry_unity.a` |

Do not rename the library. The C# package uses that name when it calls native
functions.

## 4. Create a component

Add `WryWebView` to a GameObject. Set an initial URL and rectangle in the
Inspector, or create it from code:

```csharp
using UnityEngine;
using Wry.Unity;

public sealed class AnnouncementScreen : MonoBehaviour
{
    [SerializeField] private WryWebView webView;

    private void Start()
    {
        webView.PageLoad += (url, finished) =>
            Debug.Log((finished ? "Finished: " : "Started: ") + url);

        webView.Create(
            "https://example.com/announcements",
            new RectInt(40, 40, Screen.width - 80, Screen.height - 80));
    }
}
```

The rectangle is measured from the bottom-left in Unity coordinates. The native
bridge converts it to the top-left coordinate system used by the platform view.

## 5. Test the overlay

Test these behaviors before building a full login flow:

1. A page loads over the Unity player.
2. A text field accepts keyboard input.
3. `SetVisible(false)` hides the overlay and `SetVisible(true)` shows it again.
4. `SetBounds` moves and resizes the native view.
5. A JavaScript message reaches the `IpcMessage` event.
6. Closing or destroying the component removes the native view.

Start with a page you control. This makes network, certificate, content-security,
and JavaScript errors easier to separate from Unity integration problems.

## Common first-run issues

- A blank page usually means the native library was not imported or the platform
  WebView runtime is missing.
- A page behind the game usually means the player is using exclusive fullscreen.
- No network response usually means Unity's Internet Access setting or the
  Android manifest is missing the `INTERNET` permission.
- No Android WebView usually means `WryUnity.initialize` did not run before
  `Create`.

See [Troubleshooting](./troubleshooting) before changing the native code.
