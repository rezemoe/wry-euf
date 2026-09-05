# Native API

The C ABI is declared in `crates/wry-unity/include/wry_unity.h`. The C# package
uses the same functions through P/Invoke.

All functions use C calling conventions. String arguments are UTF-8 and must be
null-terminated. Native functions do not retain string pointers after returning.

## Lifecycle

```c
int32_t wry_unity_init(void);
uint32_t wry_unity_create(
    const char* url,
    const char* html,
    WryUnityRect bounds,
    uint8_t visible,
    uint8_t transparent,
    const char* user_agent,
    const char* data_directory);
void wry_unity_destroy(uint32_t id);
```

Pass either `url` or `html`. A null pointer means that option is not supplied.
`wry_unity_create` returns `0` on failure. A successful ID is valid until
`wry_unity_destroy` is called.

`wry_unity_init` is idempotent for normal Unity usage. On Linux it initializes
GTK and must run on the thread that will own the WebView.

## Parent windows

```c
void wry_unity_set_parent_window(intptr_t handle);
```

This is optional when automatic parent lookup is sufficient. The handle is an
HWND on Windows and an X11 window ID on Linux. Android and iOS obtain their parent
from the Activity or Unity view.

## View control

```c
void wry_unity_set_bounds(uint32_t id, WryUnityRect bounds);
void wry_unity_set_visible(uint32_t id, uint8_t visible);
void wry_unity_set_background_color(uint32_t id, uint8_t r, uint8_t g, uint8_t b, uint8_t a);
void wry_unity_zoom(uint32_t id, double factor);
```

Native rectangles use a top-left origin. The C# wrapper converts Unity's
bottom-left rectangles before calling these functions.

## Navigation and script execution

```c
void wry_unity_load_url(uint32_t id, const char* url);
void wry_unity_load_html(uint32_t id, const char* html);
void wry_unity_eval(uint32_t id, const char* script, uint32_t request_id);
void wry_unity_go_back(uint32_t id);
void wry_unity_go_forward(uint32_t id);
void wry_unity_reload(uint32_t id);
int32_t wry_unity_can_go_back(uint32_t id);
```

`wry_unity_eval` reports its result through the evaluation callback, including
the request ID supplied by the caller. `wry_unity_can_go_back` returns `1`, `0`,
or `-1` on error.

## Callbacks

Callbacks are registered globally:

```c
void wry_unity_set_ipc_callback(WryUnityIpcCallback callback, void* user_data);
void wry_unity_set_page_load_callback(WryUnityPageLoadCallback callback, void* user_data);
void wry_unity_set_navigation_callback(WryUnityNavigationCallback callback, void* user_data);
void wry_unity_set_eval_callback(WryUnityEvalCallback callback, void* user_data);
```

The `user_data` pointer is returned unchanged. Keep callback functions and any
user data they reference alive until the callbacks are cleared or the native
library is unloaded.

The navigation callback returns non-zero to allow navigation and zero to cancel.
It is synchronous because the WebView needs a decision before continuing.

## Errors and event processing

```c
const char* wry_unity_last_error(void);
void wry_unity_pump(void);
```

After a function reports failure, `wry_unity_last_error` returns a pointer to a
thread-local error string. Read it before making another native call on that
thread. `wry_unity_pump` is required for Linux GTK event processing and is a
no-op on the other supported platforms.
