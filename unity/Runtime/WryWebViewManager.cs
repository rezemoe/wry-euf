using System;
using UnityEngine;

namespace Wry.Unity
{
    [DefaultExecutionOrder(-32000)]
    internal sealed class WryWebViewManager : MonoBehaviour
    {
        private static WryWebViewManager instance;

        internal static void EnsureInitialized()
        {
            if (instance != null) return;
            var host = new GameObject("WRY Unity WebView Manager");
            instance = host.AddComponent<WryWebViewManager>();
            DontDestroyOnLoad(host);
#if UNITY_ANDROID && !UNITY_EDITOR
            using (var player = new AndroidJavaClass("com.unity3d.player.UnityPlayer"))
            using (var activity = player.GetStatic<AndroidJavaObject>("currentActivity"))
            using (var bridge = new AndroidJavaClass("com.wryunity.webview.WryUnity"))
                bridge.CallStatic("initialize", activity);
#endif
            if (WryNative.wry_unity_init() == 0) throw new InvalidOperationException(WryNative.LastError());
            WryNative.wry_unity_set_ipc_callback(WryWebViewManagerProxy.Ipc, IntPtr.Zero);
            WryNative.wry_unity_set_page_load_callback(WryWebViewManagerProxy.PageLoad, IntPtr.Zero);
            WryNative.wry_unity_set_navigation_callback(WryWebViewManagerProxy.Navigation, IntPtr.Zero);
            WryNative.wry_unity_set_eval_callback(WryWebViewManagerProxy.Eval, IntPtr.Zero);
        }

        private void Update() { WryNative.wry_unity_pump(); DispatchPending(); }
        internal static void DispatchPending() { WryWebView.DispatchPending(); }

        internal static NativeRect ToNativeRect(RectInt value)
        {
            // Unity uses a bottom-left origin; native child views use top-left.
            return new NativeRect { x = value.x, y = Screen.height - value.y - value.height, width = value.width, height = value.height };
        }
    }

    internal static class WryWebViewManagerProxy
    {
        internal static readonly WryNative.IpcCallback Ipc = WryWebView.OnIpc;
        internal static readonly WryNative.PageLoadCallback PageLoad = WryWebView.OnPageLoad;
        internal static readonly WryNative.NavigationCallback Navigation = WryWebView.OnNavigation;
        internal static readonly WryNative.EvalCallback Eval = WryWebView.OnEval;
    }
}
