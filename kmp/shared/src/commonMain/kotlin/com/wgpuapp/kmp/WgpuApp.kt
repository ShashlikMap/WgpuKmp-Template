package com.wgpuapp.kmp

import androidx.compose.runtime.Composable
import uniffi.ffi_run.WgpuAppApi

@Composable
expect fun WgpuApp()

object WgpuAppApiHolder {
    var wgpuAppApi: WgpuAppApi? = null
}

