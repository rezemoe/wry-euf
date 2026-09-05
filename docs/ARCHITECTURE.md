# Architecture

WRY Unity is a native WebView overlay for Unity. It does not copy browser pixels
into a Unity texture. Each platform creates its normal native WebView and places
it above the Unity player window or view.

That choice matters for real application UI. Text input, accessibility, browser
cookies, CAPTCHA controls, video playback, focus, and platform authentication
flows remain the responsibility of the operating system WebView instead of being
reimplemented in Unity.

## The layers

The project has four layers:

1. **WRY core** is the forked WebView library at the repository root.
2. **`wry-unity`** is a small Rust library with a C-compatible API.
3. **Platform adapters** connect the Rust library to native windows, views, and
   Android lifecycle code.
4. **The Unity package** provides a C# component and turns native callbacks into
   Unity events.

The C API is deliberately smaller than the full WRY API. Unity needs a stable
boundary for loading pages, moving the overlay, running JavaScript, and receiving
events. Platform-specific WRY features remain available inside the Rust layer.

## Platform embedding

### Windows

The bridge creates a WebView2 child window owned by Unity's player HWND. The
WebView2 runtime handles browser rendering and input. WRY resizes the child window
when Unity calls `wry_unity_set_bounds`.

### Linux

The bridge initializes GTK and creates a WebKitGTK child window on the Unity X11
window. Unity runs its own event loop, so the bridge exposes `wry_unity_pump` and
the Unity manager calls it once per frame.

Linux support is intended for Unity's X11 player, including XWayland sessions.
It is not a native Wayland embedding implementation.

### Android

The Android adapter receives Unity's current `Activity`, initializes WRY's JNI
bridge, and adds a `WebView` to `android.R.id.content`. This places it above the
Unity player without replacing Unity's content view.

Android WebView work is scheduled through the Android main looper. Bounds are
implemented with `FrameLayout.LayoutParams`; visibility uses the normal Android
view visibility states.

### iOS

The iOS bridge obtains Unity's GL view and attaches a WKWebView as a subview. The
WKWebView uses autoresizing for full-screen overlays and explicit frames for child
overlays.

## Threads

The public native API is main-thread oriented:

- Windows calls belong on Unity's main thread and its normal Win32 message pump.
- Linux calls belong on the thread that initialized GTK.
- Android calls belong on the Android UI thread.
- iOS calls belong on the iOS main thread.

The C# package queues WebView callbacks before raising managed events. This keeps
event handling inside Unity's normal `Update` flow and avoids calling Unity APIs
from a native callback.

## Data ownership

WebView instances are owned by the Rust bridge. The C# object stores only the
numeric native ID. Destroying the Unity component destroys the native WebView and
removes its callback handlers.

On Windows and Linux, an optional profile directory is passed to WRY's
`WebContext`. It keeps cookies and local storage separate from other WebViews.
Android uses the system WebView profile in this version.
