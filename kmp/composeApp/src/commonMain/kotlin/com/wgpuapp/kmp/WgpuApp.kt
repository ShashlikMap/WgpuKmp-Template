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

@Composable
expect fun WgpuApp()

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
                WgpuAppApiHolder.wgpuAppApi?.changeColor()
            }, modifier = Modifier.align(Alignment.Center)) {
                Text("Increment")
            }
        }
    }
}

object WgpuAppApiHolder {
    var wgpuAppApi: WgpuAppApi? = null
}

