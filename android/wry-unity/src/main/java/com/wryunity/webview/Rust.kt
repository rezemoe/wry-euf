package com.wryunity.webview

import android.app.Activity
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse

object Rust {
    init { System.loadLibrary("wry_unity") }

    @JvmStatic external fun unitySetup(activity: Activity)
    @JvmStatic external fun onFirstActivityCreateWry()
    @JvmStatic external fun ipc(webviewId: String, url: String, message: String)
    @JvmStatic external fun assetLoaderDomain(webviewId: String): String?
    @JvmStatic external fun handleRequest(webviewId: String, request: WebResourceRequest, isDocumentStartScriptEnabled: Boolean): WebResourceResponse?
    @JvmStatic external fun shouldOverride(webviewId: String, url: String): Boolean
    @JvmStatic external fun onPageLoading(webviewId: String, url: String)
    @JvmStatic external fun onPageLoaded(webviewId: String, url: String)
    @JvmStatic external fun onEval(webviewId: String, id: Int, result: String)
    @JvmStatic external fun handleReceivedTitle(webviewId: String, title: String)
}
