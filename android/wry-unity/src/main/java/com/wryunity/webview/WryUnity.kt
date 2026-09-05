package com.wryunity.webview

import android.app.Activity

object WryUnity {
    @JvmStatic fun initialize(activity: Activity) { Rust.unitySetup(activity) }
}
