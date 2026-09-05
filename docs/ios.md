# iOS

iOS uses WKWebView as a subview of Unity's GL view. The Rust static library is
linked into the generated Unity Xcode project.

## Build integration

Add the device and simulator static libraries under the Unity iOS plugin folder.
The package's post-build script adds `WebKit.framework` to the Xcode project.
If your Unity build pipeline disables package post-build scripts, add the
framework manually in Xcode or in your own post-build step.

## View placement

The bridge uses Unity's exported `UnityGetGLView` function to find the parent view.
The WKWebView is inserted as a subview and is therefore above the Unity rendering
surface.

Full-screen child views resize with the parent. Explicit child bounds use the
WKWebView frame and can be changed through `SetBounds`.

## Main thread

UIKit and WebKit objects are main-thread objects. Create, resize, navigate, and
destroy the WebView from Unity's main thread. Do not call the C ABI from a worker
thread or from a background callback.

## WebKit differences

WKWebView has its own cookie store, navigation policy, popup behavior, and data
store rules. Test your actual login provider on iOS versions supported by the
game. A page that works in desktop Chromium may still need changes for mobile
Safari behavior.

## Shipping checklist

- Build both device and simulator variants needed by your Unity version.
- Link `WebKit.framework`.
- Test safe areas, rotation, and split-screen or multitasking where applicable.
- Test the software keyboard with login forms.
- Test WebView removal when changing scenes or closing the overlay.
