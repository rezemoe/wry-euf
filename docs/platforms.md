# Platform Overview

WRY Unity uses the WebView technology that each operating system provides. The
browser engine, runtime installation, permissions, and window rules therefore
remain platform-specific even though the C# API is shared.

| Platform | Engine | Native surface | Main requirement |
| --- | --- | --- | --- |
| Windows | WebView2 | Child HWND | WebView2 Runtime |
| Linux | WebKitGTK | Child X11 window | GTK3 and WebKitGTK 4.1 |
| Android | Android WebView | Activity content view | Unity-compatible AAR |
| iOS | WKWebView | Subview of Unity's GL view | WebKit framework |

## What is shared

Every platform supports the same core flow:

- create a WebView with a URL or HTML
- set bounds and visibility
- navigate and reload
- evaluate JavaScript
- receive IPC and page-load callbacks
- destroy the WebView

## What is not shared

Browser behavior is not identical across engines. Plan for differences in:

- cookie and storage location
- user-agent strings
- autoplay and media permissions
- file selection and camera access
- browser version and supported web APIs
- keyboard and focus behavior
- WebView profile isolation

If a login or CAPTCHA flow matters to revenue or account access, test it on every
target device family rather than relying on desktop browser testing.

## Coordinate systems

The public Unity API accepts `RectInt` values in Unity screen coordinates:

- origin at the bottom-left
- width and height in screen pixels

The bridge converts these values to native coordinates. On high-DPI displays,
verify the result on a real player because Unity's DPI awareness settings and the
platform's native units can differ.

## Fullscreen and focus

Native overlays work best in windowed or borderless-window players. Exclusive
fullscreen can prevent a child window from appearing above the game.

When a user clicks a WebView, it can receive native keyboard focus. Decide whether
the game should pause while the overlay is open and set Unity's background-running
behavior accordingly.

Open the platform-specific page before shipping a target:

- [Windows](./windows)
- [Linux](./linux)
- [Android](./android)
- [iOS](./ios)
