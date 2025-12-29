mod renderer;
pub mod wgpu_surface;

extern crate core;

use renderer::WgpuRenderer;
use wgpu_surface::WgpuSurface;
pub struct WgpuApp {
    renderer: Box<WgpuRenderer>,
}

impl WgpuApp {
    pub async fn new(canvas: Box<dyn WgpuSurface>) -> anyhow::Result<WgpuApp> {
        let renderer = WgpuRenderer::new(canvas).await?;

        let wgpu_app = WgpuApp {
            renderer: Box::new(renderer),
        };
        Ok(wgpu_app)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    pub fn counter_increment(&mut self) {
        self.renderer.just_counter += 1;
    }

    pub fn update_and_render(&mut self) {
        self.renderer.update();
        self.renderer.render().unwrap();
    }
}
