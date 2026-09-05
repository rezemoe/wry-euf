#ifndef WRY_UNITY_H
#define WRY_UNITY_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct WryUnityRect { int32_t x, y, width, height; } WryUnityRect;
typedef void (*WryUnityIpcCallback)(uint32_t, const char*, const char*, void*);
typedef void (*WryUnityPageLoadCallback)(uint32_t, const char*, uint8_t, void*);
typedef uint8_t (*WryUnityNavigationCallback)(uint32_t, const char*, void*);
typedef void (*WryUnityEvalCallback)(uint32_t, uint32_t, const char*, void*);

int32_t wry_unity_init(void);
void wry_unity_set_parent_window(intptr_t handle);
uint32_t wry_unity_create(const char* url, const char* html, WryUnityRect bounds, uint8_t visible, uint8_t transparent, const char* user_agent, const char* data_directory);
void wry_unity_destroy(uint32_t id);
void wry_unity_set_bounds(uint32_t id, WryUnityRect bounds);
void wry_unity_set_visible(uint32_t id, uint8_t visible);
void wry_unity_load_url(uint32_t id, const char* url);
void wry_unity_load_html(uint32_t id, const char* html);
void wry_unity_eval(uint32_t id, const char* script, uint32_t request_id);
void wry_unity_go_back(uint32_t id);
void wry_unity_go_forward(uint32_t id);
void wry_unity_reload(uint32_t id);
void wry_unity_set_background_color(uint32_t id, uint8_t r, uint8_t g, uint8_t b, uint8_t a);
void wry_unity_zoom(uint32_t id, double factor);
int32_t wry_unity_can_go_back(uint32_t id);
void wry_unity_set_ipc_callback(WryUnityIpcCallback callback, void* user_data);
void wry_unity_set_page_load_callback(WryUnityPageLoadCallback callback, void* user_data);
void wry_unity_set_navigation_callback(WryUnityNavigationCallback callback, void* user_data);
void wry_unity_set_eval_callback(WryUnityEvalCallback callback, void* user_data);
void wry_unity_pump(void);
const char* wry_unity_last_error(void);

#ifdef __cplusplus
}
#endif

#endif
