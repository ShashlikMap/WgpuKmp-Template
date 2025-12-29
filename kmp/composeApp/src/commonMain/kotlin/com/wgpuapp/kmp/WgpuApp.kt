package com.wgpuapp.kmp

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.size
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import uniffi.uniffi.WgpuAppApi

/**
 * Actual implementation is a native "surface" wrapped to [Composable] method
 * @see [WGPUTextureView] and [MetalView]
 */
@Composable
expect fun WgpuApp()

/**
 * [Composable] entry point for mobile application
 */
@Composable
fun App() {
    MaterialTheme {
        Box(
            modifier = Modifier.fillMaxSize()
        ) {
            Box(
                modifier = Modifier.size(400.dp).align(Alignment.Center)
            ) {
                WgpuApp()
            }
            Button(onClick = {
                WgpuAppApiHolder.wgpuAppApi?.counterIncrement()
            }, modifier = Modifier.align(Alignment.Center)) {
                Text("Increment")
            }
        }
    }
}

/**
 * Global handler to get access to [WgpuAppApi]
 * TODO Can it be done better?
 */
object WgpuAppApiHolder {
    var wgpuAppApi: WgpuAppApi? = null
}

