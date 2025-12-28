package com.wgpuapp.kmp

import android.annotation.SuppressLint
import android.content.Context
import android.graphics.SurfaceTexture
import android.os.Build
import android.util.AttributeSet
import android.view.Surface
import android.view.TextureView
import timber.log.Timber
import uniffi.uniffi.WgpuAppApi
import uniffi.uniffi.toPointer

@SuppressLint("ClickableViewAccessibility")
class WGPUTextureView : TextureView {

    private var rustBridge = RustBridge()

    constructor(context: Context) : super(context) {
    }

    constructor(context: Context, attrs: AttributeSet) : super(context, attrs) {
    }

    constructor(context: Context, attrs: AttributeSet, defStyle: Int) : super(
        context,
        attrs,
        defStyle
    )

    init {
        Timber.d("WGPUTextureView created")

        surfaceTextureListener = object : SurfaceTextureListener {
            override fun onSurfaceTextureAvailable(
                st: SurfaceTexture,
                width: Int,
                height: Int
            ) {
                val surface = Surface(st)
                val ptr = rustBridge.createWgpuAppApi(
                    surface,
                    Build.FINGERPRINT.contains("generic") ||
                            Build.FINGERPRINT.contains("sdk_gphone"),
                    context.filesDir.absolutePath + "/tiles.db",
                    context.resources.displayMetrics.density / 2.0f
                )
                Timber.d("surfaceCreated = $ptr, surface = $surface")

                WgpuAppApiHolder.wgpuAppApi = WgpuAppApi(ptr.toPointer()).apply {
                    resize(width.toUInt(), height.toUInt())
                    render()
                }
            }

            override fun onSurfaceTextureSizeChanged(
                p0: SurfaceTexture,
                width: Int,
                height: Int
            ) {
                Timber.d("onSurfaceTextureSizeChanged $width, $height")
                WgpuAppApiHolder.wgpuAppApi?.resize(width.toUInt(), height.toUInt())
            }

            override fun onSurfaceTextureDestroyed(p0: SurfaceTexture): Boolean {
                Timber.d("onSurfaceTextureDestroyed")
                return true
            }

            override fun onSurfaceTextureUpdated(p0: SurfaceTexture) {
                WgpuAppApiHolder.wgpuAppApi?.render()
            }
        }
    }
}