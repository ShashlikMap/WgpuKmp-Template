package com.wgpuapp.kmp

import kotlinx.cinterop.BetaInteropApi
import kotlinx.cinterop.ExperimentalForeignApi
import kotlinx.cinterop.ObjCAction
import kotlinx.cinterop.cValue
import kotlinx.cinterop.objcPtr
import kotlinx.cinterop.useContents
import platform.CoreGraphics.CGRectMake
import platform.Foundation.NSDefaultRunLoopMode
import platform.Foundation.NSRunLoop
import platform.Foundation.NSSelectorFromString
import platform.QuartzCore.CADisplayLink
import platform.QuartzCore.CAFrameRateRange
import platform.QuartzCore.CAMetalLayer
import platform.UIKit.UIColor
import platform.UIKit.UIScreen
import platform.UIKit.UIView
import platform.UIKit.UIViewMeta
import platform.darwin.NSObject
import uniffi.uniffi.WgpuAppApi

/**
 * Kotlin/Native implementation of UIView with CAMetalLayer to communicate with [WgpuAppApi]
 * This is much more convenient than Swift implementation
 */
@OptIn(ExperimentalForeignApi::class, BetaInteropApi::class)
internal class MetalView : UIView(CGRectMake(0.0, 0.0, 0.0, 0.0)) {
    private var displayLink: CADisplayLink? = null
    private var wgpuAppApi: WgpuAppApi? = null

    companion object : UIViewMeta() {
        override fun layerClass(): kotlinx.cinterop.ObjCClass {
            return CAMetalLayer
        }
    }

    init {
        contentScaleFactor = UIScreen.mainScreen.scale()
        backgroundColor = UIColor.redColor
    }

    @ObjCAction
    override fun didMoveToWindow() {
        super.didMoveToWindow()
        if (window == null) {
            displayLink?.invalidate()
            displayLink = null
        }
    }

    @ObjCAction
    override fun layoutSubviews() {
        super.layoutSubviews()
        val boundsWidth = bounds.useContents { size.width }
        val boundsHeight = bounds.useContents { size.height }
        if (boundsWidth > 0 && boundsWidth > 0) {
            initializeApiIfNeeded()
            wgpuAppApi?.resize(boundsWidth.toUInt(), boundsHeight.toUInt())
        }
    }

    private fun initializeApiIfNeeded() {
        if (wgpuAppApi != null) return

        val opaquePtrThis = this.objcPtr().toLong()
        val opaquePtrLayer = this.layer.objcPtr().toLong()

        // @see uniffi/src/platform/ios.rs
        val api = uniffi.uniffi.createWgpuAppApiForIos(
            opaquePtrThis.toULong(), opaquePtrLayer.toULong(), 90, ""
        )
        wgpuAppApi = api
        WgpuAppApiHolder.wgpuAppApi = api

        startRendering()
    }

    private fun startRendering() {
        displayLink = CADisplayLink.displayLinkWithTarget(
            target = DisplayLinkProxy {
                wgpuAppApi?.render()
            }, selector = NSSelectorFromString(DisplayLinkProxy::handleDisplayLinkTick.name)
        )

        displayLink?.preferredFrameRateRange = cValue<CAFrameRateRange> {
            minimum = 30F
            preferred = 60F
            maximum = 60F
        }
        displayLink?.addToRunLoop(NSRunLoop.mainRunLoop, NSDefaultRunLoopMode)
    }
}

@OptIn(BetaInteropApi::class)
private class DisplayLinkProxy(
    private val callback: () -> Unit
) : NSObject() {
    @ObjCAction
    fun handleDisplayLinkTick() {
        callback()
    }
}