package com.wryunity.webview

import android.content.Context
import android.webkit.CookieManager
import android.webkit.WebView

class RustWebView(context: Context, val initScripts: Array<String>, val id: String) : WebView(context) {
    val isDocumentStartScriptEnabled = false

    init {
        settings.javaScriptEnabled = true
        settings.domStorageEnabled = true
        settings.databaseEnabled = true
        settings.mediaPlaybackRequiresUserGesture = false
    }

    fun loadUrlMainThread(url: String) { post { loadUrl(url) } }
    fun loadUrlMainThread(url: String, headers: Map<String, String>) { post { loadUrl(url, headers) } }
    override fun loadUrl(url: String) { if (!Rust.shouldOverride(id, url)) super.loadUrl(url) }
    override fun loadUrl(url: String, headers: Map<String, String>) { if (!Rust.shouldOverride(id, url)) super.loadUrl(url, headers) }
    fun loadHTMLMainThread(html: String) { post { loadDataWithBaseURL(null, html, "text/html", "UTF-8", null) } }
    fun evalScript(requestId: Int, script: String) { post { evaluateJavascript(script) { Rust.onEval(id, requestId, it) } } }
    fun clearAllBrowsingData() { clearCache(true); clearHistory(); clearFormData() }
    fun getCookies(url: String): String = CookieManager.getInstance().getCookie(url) ?: ""
}
