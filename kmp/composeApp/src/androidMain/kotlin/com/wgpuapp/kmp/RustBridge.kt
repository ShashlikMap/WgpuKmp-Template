package com.wgpuapp.kmp

import android.view.Surface

class RustBridge {

    init {
        System.loadLibrary("uniffi")
    }

    external fun createWgpuAppApi(surface: Surface, isEmulator: Boolean, tilesDb: String, dpiScale: Float): Long
}