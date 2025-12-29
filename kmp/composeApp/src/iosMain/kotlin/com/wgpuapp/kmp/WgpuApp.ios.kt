package com.wgpuapp.kmp

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.viewinterop.UIKitView

@Composable
actual fun WgpuApp() {
    UIKitView(
        factory = {
            MetalView()
        },
        modifier = Modifier.fillMaxSize()
    )
}