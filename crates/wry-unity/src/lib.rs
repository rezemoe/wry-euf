//! Unity-facing C ABI for the WRY fork.
//!
//! The API is intentionally small and uses platform pixels with a top-left origin.
//! Unity calls must be made from its main thread. On Linux, `wry_unity_pump` must
//! be called once per frame so GTK/WebKitGTK can process events.

use std::{
  cell::RefCell,
  collections::HashMap,
  ffi::{c_char, c_void, CStr, CString},
  ptr,
};

#[cfg(target_os = "windows")]
use std::num::NonZeroIsize;

use raw_window_handle::{HasWindowHandle, RawWindowHandle, WindowHandle};
use wry::{
  dpi::{LogicalPosition, LogicalSize},
  Rect, WebContext, WebView, WebViewBuilder,
};

#[cfg(target_os = "ios")]
extern "C" {
  fn UnityGetGLView() -> *mut c_void;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WryUnityRect {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
}

#[cfg(target_os = "android")]
use std::ptr::NonNull;

pub type WryUnityIpcCallback = unsafe extern "C" fn(u32, *const c_char, *const c_char, *mut c_void);
pub type WryUnityPageLoadCallback = unsafe extern "C" fn(u32, *const c_char, u8, *mut c_void);
pub type WryUnityNavigationCallback = unsafe extern "C" fn(u32, *const c_char, *mut c_void) -> u8;
pub type WryUnityEvalCallback = unsafe extern "C" fn(u32, u32, *const c_char, *mut c_void);

#[derive(Default)]
struct CallbackSlots {
  ipc: Option<(WryUnityIpcCallback, *mut c_void)>,
  page_load: Option<(WryUnityPageLoadCallback, *mut c_void)>,
  navigation: Option<(WryUnityNavigationCallback, *mut c_void)>,
  eval: Option<(WryUnityEvalCallback, *mut c_void)>,
}

// Callback pointers are supplied by Unity and are only invoked on the Unity/UI thread.
unsafe impl Send for CallbackSlots {}
unsafe impl Sync for CallbackSlots {}

static CALLBACKS: std::sync::Mutex<CallbackSlots> = std::sync::Mutex::new(CallbackSlots {
  ipc: None,
  page_load: None,
  navigation: None,
  eval: None,
});

thread_local! {
  static WEBVIEWS: RefCell<HashMap<u32, WebViewEntry>> = RefCell::new(HashMap::new());
}

struct WebViewEntry {
  webview: WebView,
  // WebContext owns the profile directory configuration used during construction.
  _context: Option<WebContext>,
}

#[cfg(target_os = "windows")]
static PARENT_HANDLE: std::sync::OnceLock<isize> = std::sync::OnceLock::new();
#[cfg(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]
static PARENT_HANDLE: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
#[cfg(target_os = "android")]
static PARENT_HANDLE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
#[cfg(target_os = "ios")]
static PARENT_HANDLE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();

static NEXT_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
thread_local! { static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) }; }

fn set_error(error: impl std::fmt::Display) {
  let value =
    CString::new(error.to_string()).unwrap_or_else(|_| CString::new("native error").unwrap());
  LAST_ERROR.with(|slot| *slot.borrow_mut() = Some(value));
}

fn clear_error() {
  LAST_ERROR.with(|slot| *slot.borrow_mut() = None);
}

fn c_string(value: *const c_char) -> Result<String, String> {
  if value.is_null() {
    return Err("null string argument".to_string());
  }
  unsafe { CStr::from_ptr(value) }
    .to_str()
    .map(str::to_owned)
    .map_err(|_| "argument is not valid UTF-8".to_string())
}

fn optional_c_string(value: *const c_char) -> Result<Option<String>, String> {
  if value.is_null() {
    Ok(None)
  } else {
    c_string(value).map(Some)
  }
}

fn rect(value: WryUnityRect) -> Rect {
  Rect {
    position: LogicalPosition::new(value.x, value.y).into(),
    size: LogicalSize::new(value.width.max(1), value.height.max(1)).into(),
  }
}

struct ParentWindow {
  #[cfg(target_os = "windows")]
  handle: isize,
  #[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
  ))]
  handle: u64,
  #[cfg(target_os = "android")]
  handle: NonNull<c_void>,
  #[cfg(target_os = "ios")]
  handle: NonNull<c_void>,
}

impl HasWindowHandle for ParentWindow {
  fn window_handle(&self) -> Result<WindowHandle<'_>, raw_window_handle::HandleError> {
    #[cfg(target_os = "windows")]
    {
      let handle = raw_window_handle::Win32WindowHandle::new(
        NonZeroIsize::new(self.handle).ok_or(raw_window_handle::HandleError::Unavailable)?,
      );
      return Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Win32(handle)) });
    }
    #[cfg(any(
      target_os = "linux",
      target_os = "dragonfly",
      target_os = "freebsd",
      target_os = "netbsd",
      target_os = "openbsd"
    ))]
    {
      let handle = raw_window_handle::XlibWindowHandle::new(self.handle);
      return Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Xlib(handle)) });
    }
    #[cfg(target_os = "android")]
    {
      let handle = raw_window_handle::AndroidNdkWindowHandle::new(self.handle);
      return Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::AndroidNdk(handle)) });
    }
    #[cfg(target_os = "ios")]
    {
      let handle = raw_window_handle::UiKitWindowHandle::new(self.handle);
      return Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::UiKit(handle)) });
    }
    #[allow(unreachable_code)]
    Err(raw_window_handle::HandleError::Unavailable)
  }
}

fn parent_window() -> Result<ParentWindow, String> {
  #[cfg(target_os = "windows")]
  {
    let handle = PARENT_HANDLE
      .get()
      .copied()
      .or_else(auto_windows_parent)
      .ok_or_else(|| "Unity HWND was not found; call wry_unity_set_parent_window".to_string())?;
    return Ok(ParentWindow { handle });
  }
  #[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
  ))]
  {
    let handle = PARENT_HANDLE
      .get()
      .copied()
      .or_else(auto_x11_parent)
      .ok_or_else(|| {
        "Unity X11 window was not found; call wry_unity_set_parent_window".to_string()
      })?;
    return Ok(ParentWindow { handle });
  }
  #[cfg(target_os = "android")]
  {
    return PARENT_HANDLE
      .get()
      .copied()
      .and_then(|handle| NonNull::new(handle as *mut c_void))
      .map(|handle| ParentWindow { handle })
      .ok_or_else(|| "Android WRY is not initialized; call WryUnity.Initialize first".to_string());
  }
  #[cfg(target_os = "ios")]
  {
    let handle = PARENT_HANDLE.get().copied().or_else(|| unsafe {
      UnityGetGLView()
        .as_ref()
        .map(|value| value as *const _ as usize)
    });
    return handle
      .and_then(|handle| NonNull::new(handle as *mut c_void))
      .map(|handle| ParentWindow { handle })
      .ok_or_else(|| "Unity GL view was not found".to_string());
  }
  Err("this platform is not implemented by the desktop bridge".to_string())
}

#[cfg(target_os = "windows")]
fn auto_windows_parent() -> Option<isize> {
  use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM},
    UI::WindowsAndMessaging::{
      EnumWindows, GetWindow, GetWindowRect, GetWindowThreadProcessId, IsWindowVisible, GW_OWNER,
    },
  };
  let pid = std::process::id();
  let mut best = (0i64, 0isize);
  unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let state = &mut *(lparam.0 as *mut (u32, i64, isize));
    let mut process = 0u32;
    GetWindowThreadProcessId(hwnd, Some(&mut process));
    if process != state.0 || !IsWindowVisible(hwnd).as_bool() || GetWindow(hwnd, GW_OWNER).0 != 0 {
      return BOOL(1);
    }
    let mut rect = windows::Win32::Foundation::RECT::default();
    if GetWindowRect(hwnd, &mut rect).is_ok() {
      let area =
        i64::from((rect.right - rect.left).max(0)) * i64::from((rect.bottom - rect.top).max(0));
      if area > state.1 {
        state.1 = area;
        state.2 = hwnd.0 as isize;
      }
    }
    BOOL(1)
  }
  let mut state = (pid, best.0, best.1);
  unsafe {
    let _ = EnumWindows(Some(callback), LPARAM(&mut state as *mut _ as isize));
  }
  if state.2 == 0 {
    None
  } else {
    Some(state.2)
  }
}

#[cfg(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]
fn auto_x11_parent() -> Option<u64> {
  // Unity's Linux player is X11/XWayland. The focused window is a reliable fallback
  // while the plugin is initialized from a Unity frame on the main thread.
  let xlib = unsafe { x11_dl::xlib::Xlib::open().ok()? };
  let display = unsafe { (xlib.XOpenDisplay)(ptr::null()) };
  if display.is_null() {
    return None;
  }
  let mut focused = 0;
  let mut revert = 0;
  unsafe {
    (xlib.XGetInputFocus)(display, &mut focused, &mut revert);
    (xlib.XCloseDisplay)(display);
  }
  if focused == 0 {
    None
  } else {
    Some(focused as u64)
  }
}

fn build_webview(
  id: u32,
  url: Option<String>,
  html: Option<String>,
  bounds: WryUnityRect,
  visible: bool,
  transparent: bool,
  user_agent: Option<String>,
  data_directory: Option<String>,
) -> Result<WebViewEntry, String> {
  let parent = parent_window()?;
  let mut context = data_directory.map(|path| WebContext::new(Some(path.into())));
  let mut builder = match context.as_mut() {
    Some(context) => WebViewBuilder::new_with_web_context(context),
    None => WebViewBuilder::new(),
  };
  builder = builder
    .with_id(Box::leak(id.to_string().into_boxed_str()))
    .with_bounds(rect(bounds))
    .with_visible(visible)
    .with_transparent(transparent)
    .with_autoplay(true)
    .with_ipc_handler(move |request| {
      let callback = CALLBACKS.lock().unwrap().ipc;
      if let Some((callback, user_data)) = callback {
        let url = CString::new(request.uri().to_string()).unwrap_or_default();
        let body = CString::new(request.body().as_str()).unwrap_or_default();
        unsafe {
          callback(id, url.as_ptr(), body.as_ptr(), user_data);
        }
      }
    })
    .with_navigation_handler(move |url| {
      let callback = CALLBACKS.lock().unwrap().navigation;
      callback
        .map(|(callback, user_data)| {
          let url = CString::new(url).unwrap_or_default();
          unsafe { callback(id, url.as_ptr(), user_data) != 0 }
        })
        .unwrap_or(true)
    })
    .with_on_page_load_handler(move |event, url| {
      let callback = CALLBACKS.lock().unwrap().page_load;
      if let Some((callback, user_data)) = callback {
        let url = CString::new(url).unwrap_or_default();
        unsafe {
          callback(
            id,
            url.as_ptr(),
            matches!(event, wry::PageLoadEvent::Finished) as u8,
            user_data,
          );
        }
      }
    });
  if let Some(user_agent) = user_agent {
    builder = builder.with_user_agent(&user_agent);
  }
  if let Some(url) = url {
    builder = builder.with_url(&url);
  }
  if let Some(html) = html {
    builder = builder.with_html(&html);
  }
  let webview = builder
    .build_as_child(&parent)
    .map_err(|error| error.to_string())?;
  Ok(WebViewEntry {
    webview,
    _context: context,
  })
}

#[no_mangle]
pub extern "C" fn wry_unity_init() -> i32 {
  clear_error();
  #[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
  ))]
  if !gtk::is_initialized() {
    if let Err(error) = gtk::init() {
      set_error(error);
      return 0;
    }
  }
  1
}

#[no_mangle]
pub extern "C" fn wry_unity_set_parent_window(handle: isize) {
  clear_error();
  #[cfg(target_os = "windows")]
  {
    let _ = PARENT_HANDLE.set(handle);
  }
  #[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
  ))]
  {
    let _ = PARENT_HANDLE.set(handle as u64);
  }
  #[cfg(target_os = "ios")]
  {
    let _ = PARENT_HANDLE.set(handle as usize);
  }
}

#[no_mangle]
pub extern "C" fn wry_unity_create(
  url: *const c_char,
  html: *const c_char,
  bounds: WryUnityRect,
  visible: u8,
  transparent: u8,
  user_agent: *const c_char,
  data_directory: *const c_char,
) -> u32 {
  clear_error();
  let result = (|| {
    let url = optional_c_string(url)?;
    let html = optional_c_string(html)?;
    let user_agent = optional_c_string(user_agent)?;
    let data_directory = optional_c_string(data_directory)?;
    let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let entry = build_webview(
      id,
      url,
      html,
      bounds,
      visible != 0,
      transparent != 0,
      user_agent,
      data_directory,
    )?;
    WEBVIEWS.with(|views| {
      views.borrow_mut().insert(id, entry);
    });
    Ok::<u32, String>(id)
  })();
  match result {
    Ok(id) => id,
    Err(error) => {
      set_error(error);
      0
    }
  }
}

fn with_webview(id: u32, action: impl FnOnce(&WebView) -> wry::Result<()>) {
  WEBVIEWS.with(|views| {
    if let Some(entry) = views.borrow().get(&id) {
      if let Err(error) = action(&entry.webview) {
        set_error(error);
      }
    } else {
      set_error(format!("unknown webview id {id}"));
    }
  });
}

#[no_mangle]
pub extern "C" fn wry_unity_destroy(id: u32) {
  clear_error();
  WEBVIEWS.with(|views| {
    if views.borrow_mut().remove(&id).is_none() {
      set_error(format!("unknown webview id {id}"));
    }
  });
}

#[no_mangle]
pub extern "C" fn wry_unity_set_bounds(id: u32, bounds: WryUnityRect) {
  clear_error();
  with_webview(id, |view| view.set_bounds(rect(bounds)));
}
#[no_mangle]
pub extern "C" fn wry_unity_set_visible(id: u32, visible: u8) {
  clear_error();
  with_webview(id, |view| view.set_visible(visible != 0));
}
#[no_mangle]
pub extern "C" fn wry_unity_load_url(id: u32, url: *const c_char) {
  clear_error();
  match c_string(url) {
    Ok(url) => with_webview(id, |view| view.load_url(&url)),
    Err(error) => set_error(error),
  }
}
#[no_mangle]
pub extern "C" fn wry_unity_load_html(id: u32, html: *const c_char) {
  clear_error();
  match c_string(html) {
    Ok(html) => with_webview(id, |view| view.load_html(&html)),
    Err(error) => set_error(error),
  }
}
#[no_mangle]
pub extern "C" fn wry_unity_eval(id: u32, script: *const c_char, request_id: u32) {
  clear_error();
  let script = match c_string(script) {
    Ok(value) => value,
    Err(error) => {
      set_error(error);
      return;
    }
  };
  with_webview(id, |view| {
    let callback = move |result: String| {
      let callback = CALLBACKS.lock().unwrap().eval;
      if let Some((callback, user_data)) = callback {
        let result = CString::new(result).unwrap_or_default();
        unsafe {
          callback(id, request_id, result.as_ptr(), user_data);
        }
      }
    };
    view.evaluate_script_with_callback(&script, callback)
  });
}
#[no_mangle]
pub extern "C" fn wry_unity_go_back(id: u32) {
  clear_error();
  with_webview(id, WebView::go_back);
}
#[no_mangle]
pub extern "C" fn wry_unity_go_forward(id: u32) {
  clear_error();
  with_webview(id, WebView::go_forward);
}
#[no_mangle]
pub extern "C" fn wry_unity_reload(id: u32) {
  clear_error();
  with_webview(id, WebView::reload);
}
#[no_mangle]
pub extern "C" fn wry_unity_set_background_color(id: u32, r: u8, g: u8, b: u8, a: u8) {
  clear_error();
  with_webview(id, |view| view.set_background_color((r, g, b, a)));
}
#[no_mangle]
pub extern "C" fn wry_unity_zoom(id: u32, factor: f64) {
  clear_error();
  with_webview(id, |view| view.zoom(factor));
}

#[no_mangle]
pub extern "C" fn wry_unity_can_go_back(id: u32) -> i32 {
  clear_error();
  WEBVIEWS.with(|views| match views.borrow().get(&id) {
    Some(entry) => match entry.webview.can_go_back() {
      Ok(value) => value as i32,
      Err(error) => {
        set_error(error);
        -1
      }
    },
    None => {
      set_error(format!("unknown webview id {id}"));
      -1
    }
  })
}

#[no_mangle]
pub extern "C" fn wry_unity_set_ipc_callback(
  callback: Option<WryUnityIpcCallback>,
  user_data: *mut c_void,
) {
  CALLBACKS.lock().unwrap().ipc = callback.map(|callback| (callback, user_data));
}
#[no_mangle]
pub extern "C" fn wry_unity_set_page_load_callback(
  callback: Option<WryUnityPageLoadCallback>,
  user_data: *mut c_void,
) {
  CALLBACKS.lock().unwrap().page_load = callback.map(|callback| (callback, user_data));
}
#[no_mangle]
pub extern "C" fn wry_unity_set_navigation_callback(
  callback: Option<WryUnityNavigationCallback>,
  user_data: *mut c_void,
) {
  CALLBACKS.lock().unwrap().navigation = callback.map(|callback| (callback, user_data));
}
#[no_mangle]
pub extern "C" fn wry_unity_set_eval_callback(
  callback: Option<WryUnityEvalCallback>,
  user_data: *mut c_void,
) {
  CALLBACKS.lock().unwrap().eval = callback.map(|callback| (callback, user_data));
}

#[no_mangle]
pub extern "C" fn wry_unity_pump() {
  #[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
  ))]
  while gtk::events_pending() {
    gtk::main_iteration_do(false);
  }
}

#[no_mangle]
pub extern "C" fn wry_unity_last_error() -> *const c_char {
  LAST_ERROR.with(|slot| {
    slot
      .borrow()
      .as_ref()
      .map_or(ptr::null(), |value| value.as_ptr())
  })
}

#[cfg(target_os = "android")]
#[no_mangle]
pub unsafe extern "C" fn Java_com_wryunity_webview_Rust_unitySetup(
  mut env: jni::JNIEnv,
  _: jni::objects::JClass,
  activity: jni::objects::JObject,
) {
  let window_manager = match env
    .call_method(
      &activity,
      "getWindowManager",
      "()Landroid/view/WindowManager;",
      &[],
    )
    .and_then(|value| value.l())
  {
    Ok(value) => value,
    Err(_) => return,
  };
  let raw = window_manager.as_raw() as *mut c_void;
  let Some(raw) = NonNull::new(raw) else {
    return;
  };
  let _ = PARENT_HANDLE.set(raw as usize);
  let Ok(activity) = env.new_global_ref(activity) else {
    return;
  };
  wry::android_setup_unity("com.wryunity.webview", env, activity);
}

#[cfg(target_os = "android")]
wry::android_binding!(com_wryunity, webview);
