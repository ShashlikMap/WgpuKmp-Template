uniffi::setup_scaffolding!();

mod platform;

use std::sync::RwLock;
use wgpu_app::WgpuApp;

#[derive(uniffi::Object)]
pub struct WgpuAppApi {
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

    fn change_color(&self) {
        let mut wgpu_app = self.wgpu_app.write().unwrap();
        wgpu_app.change_color()
    }
}
