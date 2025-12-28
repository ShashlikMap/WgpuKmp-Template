package com.wgpuapp.kmp

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.viewinterop.AndroidView

@Composable
actual fun WgpuApp() {
    AndroidView(
        factory = { ctx ->
            WGPUTextureView(context = ctx)
        },
        modifier = Modifier.fillMaxSize()
    )
}
