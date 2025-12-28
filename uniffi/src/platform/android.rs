use crate::WgpuAppApi;
use app_surface::AppSurface;
use jni::objects::JClass;
use jni::sys::{jboolean, jlong, jobject};
use jni::JNIEnv;
use jni_fn::jni_fn;
use wgpu_app::WgpuApp;
use std::sync::{Arc, RwLock};
use wgpu::{Device, Queue, SurfaceConfiguration, SurfaceError, SurfaceTexture};
use wgpu_app::wgpu_canvas::WgpuCanvas;
use app_surface::SurfaceFrame;
use pollster::FutureExt;

//FIXME https://github.com/gobley/gobley/issues/20
#[uniffi::export]
pub fn create_wgpu_app_api_for_ios(view: u64, metal_layer: u64, maximum_frames: i32, _tiles_db: String) -> WgpuAppApi {
    panic!("Android not supported")
}

struct AndroidSurfaceAppSurface {
    app_surface: AppSurface,
}

impl WgpuCanvas for AndroidSurfaceAppSurface {
    fn queue(&self) -> &Queue {
        &self.app_surface.queue
    }

    fn config(&self) -> &SurfaceConfiguration {
        &self.app_surface.config
    }

    fn device(&self) -> &Device {
        &self.app_surface.device
    }

    fn get_current_texture(&self) -> Result<SurfaceTexture, SurfaceError> {
        self.app_surface.surface.get_current_texture()
    }

    fn on_resize(&mut self) {
        self.app_surface.resize_surface();
    }

    fn on_pre_render(&self) {
        // not required
    }

    fn on_post_render(&self) {
        // not required
    }
}

#[unsafe(no_mangle)]
#[jni_fn("com.wgpuapp.kmp.WGPUTextureView")] // TODO How to pass as a build param?
pub fn createWgpuAppApi(
    env: *mut JNIEnv<'_>,
    _: JClass,
    surface: jobject,
    emulator: jboolean,
) -> jlong {
    init_logger();
    let app_surface = AppSurface::new(env, surface, emulator != 0).block_on();
    let surface = AndroidSurfaceAppSurface { app_surface };
    let wgpu_app = pollster::block_on(WgpuApp::new(Box::new(surface))).unwrap();
    let map_api = WgpuAppApi {
        wgpu_app: RwLock::new(wgpu_app),
    };
    Arc::into_raw(Arc::new(map_api)) as jlong
}

fn init_logger() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
    );
    log_panics::init();
}