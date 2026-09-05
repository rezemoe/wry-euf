package com.wryunity.webview

import android.app.Activity
import android.webkit.ConsoleMessage
import android.webkit.PermissionRequest
import android.webkit.WebChromeClient

class RustWebChromeClient(private val activity: Activity, private val webViewId: String) : WebChromeClient() {
    private external fun onPermissionRequestNative(webviewId: String, resource: String): Int
    private external fun onGeolocationPermissionRequestNative(webviewId: String, origin: String): Boolean
    override fun onPermissionRequest(request: PermissionRequest) { request.deny() }
    override fun onConsoleMessage(consoleMessage: ConsoleMessage): Boolean = true
}
