# Troubleshooting

## The WebView is blank

Check these in order:

1. Confirm the native library is present and imported for the active architecture.
2. Confirm `wry_unity_init` succeeded and inspect `wry_unity_last_error`.
3. Confirm the platform WebView runtime is installed.
4. Try a local HTML page to separate networking from embedding.
5. Check that the rectangle has a positive width and height.
6. Check that the WebView is visible and above the Unity view.

## The WebView appears behind Unity

Use a windowed or borderless player. Exclusive fullscreen often prevents native
child windows from appearing above the rendering surface. On Android and iOS,
check that the native view was added to the Activity content or Unity GL view,
not to a temporary scene object.

## Input does not work

Click the WebView and check whether the operating system gives it focus. On
desktop, check whether another child window is covering it. On mobile, check that
Unity is not intercepting the touch before it reaches the native view.

## Linux freezes or does not update

Call `wry_unity_pump` every frame from the GTK initialization thread. Confirm that
the player is using X11 or XWayland and that GTK can open the same display.

## Android initialization fails

Confirm all of the following:

- the AAR is merged into the final Gradle project
- the native `.so` uses the name `wry_unity`
- the package is `com.wryunity.webview`
- `WryUnity.initialize(currentActivity)` runs before creation
- the Activity is still alive when the WebView is created

Inspect Android logcat for JNI class lookup and WebView errors.

## iOS build fails to link

Make sure the static library matches the build architecture and that
`WebKit.framework` is linked. If `UnityGetGLView` is missing, check that the Unity
native interface is included in the generated Xcode project and that the plugin
is being linked into the Unity framework target.

## JavaScript callbacks never arrive

Make sure the page calls `window.ipc.postMessage` with a string. Then verify the
native callback is registered before navigation begins. For a simple test:

```js
window.ipc.postMessage("bridge-ready");
```

If page-load events arrive but IPC does not, inspect the page's console for a
blocked or overwritten `window.ipc` object.

## The page works in a browser but not in the game

Compare the user agent, cookies, viewport size, certificate chain, and browser
engine version. WebView2, WebKitGTK, Android WebView, and WKWebView are different
browser environments. Use a minimal page and add the failing feature back one
piece at a time.
