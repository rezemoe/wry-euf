using System;
using System.Runtime.InteropServices;

namespace Wry.Unity
{
    [StructLayout(LayoutKind.Sequential)]
    internal struct NativeRect
    {
        public int x;
        public int y;
        public int width;
        public int height;
    }

    internal static class WryNative
    {
#if UNITY_IOS && !UNITY_EDITOR
        private const string Library = "__Internal";
#elif UNITY_ANDROID && !UNITY_EDITOR
        private const string Library = "wry_unity";
#elif UNITY_STANDALONE_WIN
        private const string Library = "wry_unity";
#else
        private const string Library = "wry_unity";
#endif

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        internal delegate void IpcCallback(uint id, IntPtr url, IntPtr body, IntPtr userData);
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        internal delegate void PageLoadCallback(uint id, IntPtr url, byte finished, IntPtr userData);
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        internal delegate byte NavigationCallback(uint id, IntPtr url, IntPtr userData);
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        internal delegate void EvalCallback(uint id, uint requestId, IntPtr result, IntPtr userData);

        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int wry_unity_init();
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_set_parent_window(IntPtr handle);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern uint wry_unity_create(IntPtr url, IntPtr html, NativeRect bounds, byte visible, byte transparent, IntPtr userAgent, IntPtr dataDirectory);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_destroy(uint id);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_set_bounds(uint id, NativeRect bounds);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_set_visible(uint id, byte visible);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_load_url(uint id, IntPtr url);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_load_html(uint id, IntPtr html);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_eval(uint id, IntPtr script, uint requestId);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_go_back(uint id);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_go_forward(uint id);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_reload(uint id);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int wry_unity_can_go_back(uint id);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_set_background_color(uint id, byte r, byte g, byte b, byte a);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_zoom(uint id, double factor);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_set_ipc_callback(IpcCallback callback, IntPtr userData);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_set_page_load_callback(PageLoadCallback callback, IntPtr userData);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_set_navigation_callback(NavigationCallback callback, IntPtr userData);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_set_eval_callback(EvalCallback callback, IntPtr userData);
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void wry_unity_pump();
        [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] private static extern IntPtr wry_unity_last_error();

        internal static string LastError()
        {
            var pointer = wry_unity_last_error();
            return pointer == IntPtr.Zero ? "native WebView error" : Marshal.PtrToStringAnsi(pointer);
        }
    }
}
