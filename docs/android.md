# Android

Android uses the system `android.webkit.WebView` and adds it above Unity's player
view inside the current Activity. The adapter is in `android/wry-unity`.

## How initialization works

The Unity manager obtains:

```text
com.unity3d.player.UnityPlayer.currentActivity
```

It calls `com.wryunity.webview.WryUnity.initialize(activity)`. The Kotlin helper
loads the Rust library and registers the Activity with WRY. Only after that should
the C# component create a WebView.

Applications using the C ABI directly must perform the same initialization step.

## AAR contents

The AAR contains the Kotlin classes that WRY expects:

- `RustWebView`
- `RustWebViewClient`
- `RustWebChromeClient`
- `Ipc`
- `Rust`
- `WryUnity`

The package name is fixed to `com.wryunity.webview` in this version. Keep the
package name and native library name aligned when building the AAR and Rust
library.

## Overlay layout

The WebView is added to the Activity content frame with a `FrameLayout` layout
parameter. It does not call `setContentView`, so Unity's player remains in the
view hierarchy.

The native bridge accepts pixel dimensions and margins. Android WebView bounds
are updated when the C# component calls `SetBounds`.

## WebView behavior

JavaScript, DOM storage, database storage, and media playback are enabled by the
adapter. Initialization scripts are run when a page starts loading. The adapter
does not yet provide per-WebView Android data-directory isolation; Android's
normal WebView profile is used.

## Permissions

The adapter declares `INTERNET`. Camera, microphone, geolocation, and file-picker
flows need application-specific Android permissions and should be tested in the
actual Unity Activity. Do not grant permissions solely because a page asks for
them; make the decision in the native application and expose only the capabilities
the feature needs.

## Rotation and lifecycle

Unity projects commonly configure the Activity to handle orientation changes
without recreation. If your application recreates the Activity, test WebView
creation, destruction, and login state across that transition. The Rust bridge
keeps native ownership separate from the C# component, so lifecycle order matters.

## Shipping checklist

- Build the Rust library for every Android ABI you ship.
- Merge the AAR into the Unity Gradle build.
- Verify `INTERNET` is present in the merged manifest.
- Test Android back navigation while the WebView is visible.
- Test text input, CAPTCHA controls, and keyboard dismissal.
- Test Activity recreation and process death.
