uniffi::setup_scaffolding!();

mod platform;

use map::WgpuApp;
use std::sync::RwLock;

#[derive(uniffi::Object)]
pub struct WgpuAppApi {
    // TODO ?Can't use generic for FFI WgpuApp?
    wgpu_app: RwLock<WgpuApp>,
}

unsafe impl Sync for WgpuAppApi {}
unsafe impl Send for WgpuAppApi {}

#[uniffi::export]
impl WgpuAppApi {
    fn render(&self) {
        let mut wgpu_app = self.wgpu_app.write().unwrap();
        wgpu_app.update_and_render();
    }

    fn resize(&self, width: u32, height: u32) {
        let mut wgpu_app = self.wgpu_app.write().unwrap();
        wgpu_app.resize(width, height);
    }

    fn set_cam_follow_mode(&self, enabled: bool) {}
}
