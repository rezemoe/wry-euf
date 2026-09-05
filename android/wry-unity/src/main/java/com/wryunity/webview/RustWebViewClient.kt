package com.wryunity.webview

import android.graphics.Bitmap
import android.webkit.WebResourceRequest
import android.webkit.WebView
import android.webkit.WebViewClient

class RustWebViewClient(private val webView: RustWebView, private val context: android.content.Context) : WebViewClient() {
    var currentUrl: String = "about:blank"
    override fun shouldOverrideUrlLoading(view: WebView, request: WebResourceRequest): Boolean = Rust.shouldOverride(webView.id, request.url.toString())
    override fun onPageStarted(view: WebView, url: String, favicon: Bitmap?) {
        currentUrl = url
        for (script in webView.initScripts) view.evaluateJavascript(script, null)
        Rust.onPageLoading(webView.id, url)
    }
    override fun onPageFinished(view: WebView, url: String) { Rust.onPageLoaded(webView.id, url) }
}
