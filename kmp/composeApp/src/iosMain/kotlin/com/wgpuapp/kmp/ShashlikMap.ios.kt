package com.wgpuapp.kmp

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.viewinterop.UIKitViewController
import platform.UIKit.UIViewController

object WgpuAppUIViewProvider {
    lateinit var createUIViewController: () -> UIViewController
}

@Composable
actual fun WgpuApp() {
    // TODO Pass to iOS
    UIKitViewController(
        factory = WgpuAppUIViewProvider.createUIViewController,
        modifier = Modifier.fillMaxSize(),
    )
}