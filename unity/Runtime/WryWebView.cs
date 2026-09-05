using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Text;
using System.Runtime.InteropServices;
using UnityEngine;

namespace Wry.Unity
{
    public sealed class WryIpcMessage
    {
        public uint WebViewId;
        public string Url;
        public string Body;
    }

    public sealed class WryWebView : MonoBehaviour
    {
        private static readonly object Gate = new object();
        private static readonly Dictionary<uint, WryWebView> Instances = new Dictionary<uint, WryWebView>();
        private static readonly ConcurrentQueue<Action> Pending = new ConcurrentQueue<Action>();
        private static readonly WryNative.IpcCallback IpcHandler = OnIpc;
        private static readonly WryNative.PageLoadCallback PageLoadHandler = OnPageLoad;
        private static readonly WryNative.NavigationCallback NavigationHandler = OnNavigation;
        private static readonly WryNative.EvalCallback EvalHandler = OnEval;
        private static bool initialized;

        [SerializeField] private string initialUrl;
        [SerializeField] private RectInt bounds = new RectInt(0, 0, 640, 480);
        [SerializeField] private bool initiallyVisible = true;
        [SerializeField] private bool transparent;
        [SerializeField] private string profileDirectory;

        private uint nativeId;
        private uint nextRequestId;
        private GCHandle selfHandle;

        public event Action<WryIpcMessage> IpcMessage;
        public event Action<string, bool> PageLoad;
        public event Func<string, bool> NavigationRequested;
        public event Action<uint, string> ScriptResult;
        public uint Id { get { return nativeId; } }

        public void Create(string url, RectInt screenRect)
        {
            initialUrl = url;
            bounds = screenRect;
            CreateNative();
        }

        private void Awake()
        {
            WryWebViewManager.EnsureInitialized();
            if (!string.IsNullOrEmpty(initialUrl)) CreateNative();
        }

        private void Update()
        {
            DispatchPending();
        }

        private void OnDestroy()
        {
            if (nativeId == 0) return;
            var id = nativeId;
            WryNative.wry_unity_destroy(id);
            nativeId = 0;
            if (selfHandle.IsAllocated) selfHandle.Free();
            lock (Gate) Instances.Remove(id);
        }

        private void CreateNative()
        {
            if (nativeId != 0) return;
            WryWebViewManager.EnsureInitialized();
            var rect = WryWebViewManager.ToNativeRect(bounds);
            using (var url = Utf8(initialUrl))
            using (var ua = Utf8(null))
            using (var profile = Utf8(profileDirectory))
            {
                nativeId = WryNative.wry_unity_create(url.Handle, IntPtr.Zero, rect, initiallyVisible ? (byte)1 : (byte)0, transparent ? (byte)1 : (byte)0, ua.Handle, profile.Handle);
            }
            if (nativeId == 0) throw new InvalidOperationException(WryNative.LastError());
            selfHandle = GCHandle.Alloc(this);
            lock (Gate) Instances[nativeId] = this;
        }

        public void LoadUrl(string url) { EnsureCreated(); using (var value = Utf8(url)) WryNative.wry_unity_load_url(nativeId, value.Handle); }
        public void LoadHtml(string html) { EnsureCreated(); using (var value = Utf8(html)) WryNative.wry_unity_load_html(nativeId, value.Handle); }
        public void SetVisible(bool visible) { EnsureCreated(); WryNative.wry_unity_set_visible(nativeId, visible ? (byte)1 : (byte)0); }
        public void SetBounds(RectInt value) { EnsureCreated(); bounds = value; WryNative.wry_unity_set_bounds(nativeId, WryWebViewManager.ToNativeRect(value)); }
        public void GoBack() { EnsureCreated(); WryNative.wry_unity_go_back(nativeId); }
        public void GoForward() { EnsureCreated(); WryNative.wry_unity_go_forward(nativeId); }
        public void Reload() { EnsureCreated(); WryNative.wry_unity_reload(nativeId); }
        public bool CanGoBack() { EnsureCreated(); return WryNative.wry_unity_can_go_back(nativeId) == 1; }
        public void SetBackgroundColor(Color color) { EnsureCreated(); WryNative.wry_unity_set_background_color(nativeId, (byte)(color.r * 255), (byte)(color.g * 255), (byte)(color.b * 255), (byte)(color.a * 255)); }
        public uint EvaluateScript(string script)
        {
            EnsureCreated();
            var requestId = ++nextRequestId;
            using (var value = Utf8(script)) WryNative.wry_unity_eval(nativeId, value.Handle, requestId);
            return requestId;
        }

        public void SendMessageToPage(string json)
        {
            var escaped = json.Replace("\\", "\\\\").Replace("'", "\\'").Replace("\r", "\\r").Replace("\n", "\\n");
            EvaluateScript("window.dispatchEvent(new CustomEvent('wry-unity-message',{detail:'" + escaped + "'}));");
        }

        private void EnsureCreated() { if (nativeId == 0) throw new InvalidOperationException("WryWebView has not been created"); }

        internal static void DispatchPending()
        {
            while (Pending.TryDequeue(out var action)) action();
        }

        internal static void OnIpc(uint id, IntPtr url, IntPtr body, IntPtr userData)
        {
            var message = new WryIpcMessage { WebViewId = id, Url = Marshal.PtrToStringAnsi(url), Body = Marshal.PtrToStringAnsi(body) };
            Pending.Enqueue(() => { WryWebView view; lock (Gate) Instances.TryGetValue(id, out view); if (view != null) view.IpcMessage?.Invoke(message); });
        }
        internal static void OnPageLoad(uint id, IntPtr url, byte finished, IntPtr userData)
        {
            var value = Marshal.PtrToStringAnsi(url);
            Pending.Enqueue(() => { WryWebView view; lock (Gate) Instances.TryGetValue(id, out view); if (view != null) view.PageLoad?.Invoke(value, finished != 0); });
        }
        internal static byte OnNavigation(uint id, IntPtr url, IntPtr userData)
        {
            var value = Marshal.PtrToStringAnsi(url);
            WryWebView view; lock (Gate) Instances.TryGetValue(id, out view);
            if (view == null || view.NavigationRequested == null) return 1;
            var allowed = true;
            Pending.Enqueue(() => { });
            foreach (Func<string, bool> handler in view.NavigationRequested.GetInvocationList()) allowed &= handler(value);
            return allowed ? (byte)1 : (byte)0;
        }
        internal static void OnEval(uint id, uint requestId, IntPtr result, IntPtr userData)
        {
            var value = Marshal.PtrToStringAnsi(result);
            Pending.Enqueue(() => { WryWebView view; lock (Gate) Instances.TryGetValue(id, out view); if (view != null) view.ScriptResult?.Invoke(requestId, value); });
        }

        private sealed class Utf8String : IDisposable
        {
            public readonly IntPtr Handle;
            public Utf8String(string value)
            {
                if (value == null) { Handle = IntPtr.Zero; return; }
                var bytes = Encoding.UTF8.GetBytes(value);
                Handle = Marshal.AllocHGlobal(bytes.Length + 1);
                Marshal.Copy(bytes, 0, Handle, bytes.Length);
                Marshal.WriteByte(Handle, bytes.Length, 0);
            }
            public void Dispose() { if (Handle != IntPtr.Zero) Marshal.FreeHGlobal(Handle); }
        }
        private static Utf8String Utf8(string value) { return new Utf8String(value); }
    }
}
