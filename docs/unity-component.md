# Unity Integration

The Unity package is in `unity/`. Add it as a local package through Package
Manager, or copy it into a project's `Packages/` directory.

The package targets Unity 2021.3 and newer. It uses ordinary `MonoBehaviour`,
P/Invoke, and `AndroidJavaObject` APIs so it works with both Mono and IL2CPP.

## Add a WebView

Create a GameObject, add the `WryWebView` component, and set its URL and rectangle.
The rectangle uses Unity screen coordinates: `(0, 0)` is the bottom-left corner.

```csharp
using UnityEngine;
using Wry.Unity;

public sealed class LoginOverlay : MonoBehaviour
{
    [SerializeField] private WryWebView webView;

    private void Start()
    {
        webView.IpcMessage += message => Debug.Log(message.Body);
        webView.Create(
            "https://example.com/login",
            new RectInt(80, 120, 900, 620));
    }
}
```

The native overlay is positioned in the platform window. It is not constrained by
the Unity Canvas hierarchy, camera, or render pipeline.

## Inspector setup

The component exposes these useful fields:

- **Initial URL**: loaded during `Awake` when non-empty.
- **Bounds**: the initial screen rectangle.
- **Initially Visible**: whether the native view appears immediately.
- **Transparent**: requests a transparent native background where supported.
- **Profile Directory**: an optional Windows/Linux browser profile directory.

For dynamic screens, leave the initial URL empty and call `Create` yourself.

## Loading content

```csharp
webView.LoadUrl("https://example.com/announcements");
webView.LoadHtml("<h1>Maintenance</h1><p>Back soon.</p>");
webView.SetVisible(false);
webView.SetBounds(new RectInt(0, 0, Screen.width, Screen.height));
```

`WryWebView` also provides `GoBack`, `GoForward`, `CanGoBack`, `Reload`,
`SetBackgroundColor`, and `EvaluateScript`.

## Lifetime

Destroying the component destroys the native WebView. A WebView should not be
created from a background thread, and it should not outlive the Unity activity or
player window that owns it.

Keep the component on a persistent object if the browser session must survive a
scene change.

## Runtime expectations

- Set Player Settings **Internet Access** to `Require` for network content.
- Keep the player windowed or borderless when using desktop child windows.
- Test keyboard focus and controller input on the actual target platform.
- Use a profile directory when cookies and login state need clear ownership.
