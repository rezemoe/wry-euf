package com.wryunity.webview

import android.webkit.JavascriptInterface

class Ipc(private val webView: RustWebView, private val webViewClient: RustWebViewClient) {
    @JavascriptInterface fun postMessage(message: String?) { if (message != null) Rust.ipc(webView.id, webViewClient.currentUrl, message) }
}
