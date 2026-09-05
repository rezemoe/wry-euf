# Windows

The Windows implementation uses Microsoft Edge WebView2 in a native child HWND.
This makes it independent of Unity's DirectX graphics API and allows the browser
to receive normal mouse and keyboard input.

## Runtime requirement

The target system needs the WebView2 Runtime. A game installer should:

1. Check whether the runtime is available.
2. Offer the Evergreen Runtime bootstrapper or a fixed-version runtime when it is
   missing.
3. Show a useful error instead of silently leaving an empty overlay.

The Rust library does not install the runtime for the application.

## Window behavior

The native bridge creates a child WebView below Unity's top-level player window.
The parent HWND is found from the current process when possible. The C ABI also
accepts an explicit native parent handle for hosts that manage several windows.

Bounds are manually controlled by `wry_unity_set_bounds`. The coordinates are
native client-area coordinates after the C# wrapper converts Unity's origin.

## DPI

Test at 100%, 125%, 150%, and 200% display scaling. Also test moving a player
between monitors with different DPI settings. A WebView that appears offset or
the wrong size is usually a mismatch between Unity's screen pixels, the HWND
client area, and the process DPI-awareness mode.

## Data profiles

Pass a profile directory when login data must be isolated. The directory should
be stable for the intended account or installation and should not be inside a
temporary directory.

Do not share the same WebView2 data directory between processes unless the WebView2
environment is designed for that use. Concurrent profile access can fail or lock
the directory.

## Shipping checklist

- Test with the WebView2 Runtime absent.
- Test keyboard focus after opening and closing the overlay.
- Test window resize, monitor changes, and DPI changes.
- Test borderless and exclusive fullscreen separately.
- Include the native DLL with the correct Unity architecture settings.
- Keep the profile directory writable by the game process.
