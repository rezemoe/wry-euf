# Security

A WebView is a browser embedded inside the game. Treat it as an untrusted input
surface even when it normally loads your own site.

## Navigation policy

Use `NavigationRequested` to allow only the origins your flow needs:

```csharp
webView.NavigationRequested += url =>
    url.StartsWith("https://accounts.example.com/");
```

Prefer exact origin checks over substring checks. A check for
`example.com` also accepts `example.com.attacker.test`; parse the URL and compare
scheme, host, and port.

## IPC messages

Validate every message from JavaScript. The page may be compromised by an XSS
bug, a redirect, an injected third-party script, or a malicious server response.
Use a versioned message envelope and reject unknown event names.

Do not treat an IPC message as proof that an account operation succeeded. The
server must validate authentication, CAPTCHA, payment, and account-linking
results independently.

## JavaScript evaluation

Avoid concatenating user-controlled values into JavaScript source. If a value must
cross into the page, serialize it as JSON and pass it through a controlled page
API. This prevents quotes, markup, and script delimiters from changing the code
that is executed.

## Credentials and tokens

- Never log passwords, session cookies, CAPTCHA tokens, or authorization headers.
- Keep access tokens in native or server-managed storage where possible.
- Do not expose a native secret through JavaScript or IPC.
- Use HTTPS and reject invalid certificates through the platform's normal policy.
- Clear a profile when the user explicitly signs out if the product requires it.

## Remote content

Remote pages change independently of the game release. Pin the allowed origins,
keep the browser page's Content Security Policy strict, and avoid loading arbitrary
third-party frames unless the flow requires them.

## Native permissions

Only request camera, microphone, location, storage, or file access for a feature
that needs it. A WebView page requesting a permission is not enough reason to
grant it. Explain the feature to the player using the platform's normal permission
flow.
