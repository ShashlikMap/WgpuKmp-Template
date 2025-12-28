package com.wgpuapp.kmp

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import uniffi.ffi_run.WgpuAppApi

@Composable
expect fun WgpuApp()

@Composable
fun App() {
    MaterialTheme {
        Box(
            modifier = Modifier.fillMaxSize()
        ) {
            WgpuApp()
        }
    }
}

object WgpuAppApiHolder {
    var wgpuAppApi: WgpuAppApi? = null
}

