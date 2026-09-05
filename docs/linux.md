# Linux

The Linux implementation uses WebKitGTK inside a child X11 window. Unity's Linux
player normally uses X11 directly or through XWayland.

## System dependencies

The build machine needs GTK3 and WebKitGTK 4.1 development files. The target
machine needs the matching runtime libraries. Package names differ by distribution;
see [Building](./BUILDING) for common examples.

## Event pumping

GTK has its own event processing. Unity owns the main loop, so the bridge cannot
run `gtk::main()` itself. The native function `wry_unity_pump` processes pending
GTK events and the package calls it from the persistent WebView manager.

If you use the C ABI without the package, call the function from the same thread
that initialized GTK, once per frame.

## X11 and Wayland

This integration targets X11. On a Wayland desktop, Unity may run through
XWayland, in which case the X11 parent window can still work. A Unity player using
a native Wayland surface is not supported by this child-window path.

## Window and input behavior

The bridge uses the focused or explicitly supplied Unity X11 window. If the wrong
window is selected, create the WebView only after the player window is active or
provide an explicit X11 window ID through the C API.

Clicking the WebView moves native focus away from Unity. Games that should keep
running while an overlay is focused should enable their background-running policy.

## Shipping checklist

- Test on the distributions you plan to support.
- Verify WebKitGTK is installed on a clean machine.
- Test X11 and an XWayland session.
- Call `wry_unity_pump` every frame when not using the C# manager.
- Test keyboard focus and window resizing.
- Keep the WebKitGTK profile directory writable and private.
