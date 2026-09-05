using UnityEngine;

namespace Wry.Unity.Samples
{
    public sealed class LoginOverlayExample : MonoBehaviour
    {
        [SerializeField] private WryWebView webView;
        [SerializeField] private string loginUrl = "https://example.invalid/login";

        private void Start()
        {
            webView.IpcMessage += message => Debug.Log("Web IPC: " + message.Body);
            webView.PageLoad += (url, finished) => Debug.Log((finished ? "Loaded " : "Loading ") + url);
            webView.Create(loginUrl, new RectInt(80, 120, 900, 620));
        }
    }
}
