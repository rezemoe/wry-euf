# JavaScript and IPC

WRY Unity has two communication directions:

- JavaScript sends a string through `window.ipc.postMessage`.
- C# evaluates JavaScript in the page.

The native bridge keeps the transport small. Decide on a message format for your
application, usually JSON with a short event name and a payload.

## JavaScript to C#

From a page loaded in WRY Unity:

```js
window.ipc.postMessage(JSON.stringify({
  event: "login-complete",
  userId: "player-123"
}));
```

Receive the message in Unity:

```csharp
webView.IpcMessage += message =>
{
    Debug.Log("Page URL: " + message.Url);
    Debug.Log("Message: " + message.Body);
};
```

`WryIpcMessage` contains the native WebView ID, the page URL that sent the
message, and the message body. The body is not parsed automatically. Keeping
parsing in the application lets a game choose its preferred JSON library and
versioning strategy.

## C# to JavaScript

Send an application message into the page:

```csharp
webView.SendMessageToPage("{\"event\":\"open-profile\"}");
```

The helper dispatches a `wry-unity-message` browser event. A page can listen for
it like this:

```js
window.addEventListener("wry-unity-message", function (event) {
  const message = JSON.parse(event.detail);
  if (message.event === "open-profile") {
    openProfile();
  }
});
```

For code that already has a page API, use `EvaluateScript` directly:

```csharp
webView.EvaluateScript("window.gameAccount.refresh()" );
```

## Script results

`EvaluateScript` returns a request ID. Subscribe to `ScriptResult` when the page
needs to return a value:

```csharp
webView.ScriptResult += (requestId, result) =>
{
    Debug.Log("Result " + requestId + ": " + result);
};

webView.EvaluateScript("JSON.stringify(window.gameAccount.status())");
```

The result follows the platform WebView's serialized JavaScript result format.
Treat it as a string and parse it only after checking for a page-side error.

## Page loading and navigation

```csharp
webView.PageLoad += (url, finished) =>
{
    if (finished) Debug.Log("Ready: " + url);
};

webView.NavigationRequested += url =>
{
    return url.StartsWith("https://accounts.example.com/");
};
```

Returning `false` from `NavigationRequested` cancels navigation. Use this for
allowlists, not as a substitute for server-side authorization.

## Message design

Keep messages explicit and small:

```json
{
  "event": "captcha-complete",
  "requestId": "login-42",
  "payload": {
    "token": "..."
  }
}
```

Do not put secrets, long-lived access tokens, or account passwords in messages
that may be logged. A page can send a completion event while the actual token is
exchanged through your authenticated backend.
