package com.wgpuapp.kmp

import androidx.compose.ui.window.ComposeUIViewController
import platform.UIKit.UIViewController
import uniffi.ffi_run.WgpuAppApi

// FIXME Should be in Shared module
fun createWgpuAppApiForIos(view: ULong, metalLayer: ULong): WgpuAppApi {
    val api = uniffi.ffi_run.createWgpuAppApiForIos(view, metalLayer, 90, "")
    WgpuAppApiHolder.wgpuAppApi = api
    return api
}

fun MainViewController(createUIViewController: () -> UIViewController): UIViewController {
    WgpuAppUIViewProvider.createUIViewController = createUIViewController
    return ComposeUIViewController {
        App()
    }
}