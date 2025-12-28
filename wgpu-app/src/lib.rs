mod renderer;
pub mod wgpu_canvas;

extern crate core;

use renderer::WgpuRenderer;
use wgpu_canvas::WgpuCanvas;
pub struct WgpuApp {
    renderer: Box<WgpuRenderer>,
}

impl WgpuApp {
    pub async fn new(canvas: Box<dyn WgpuCanvas>) -> anyhow::Result<WgpuApp> {
        let renderer = WgpuRenderer::new(canvas).await?;

        let wgpu_app = WgpuApp {
            renderer: Box::new(renderer),
        };
        Ok(wgpu_app)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    pub fn update_and_render(&mut self) {
        self.renderer.update();
        self.renderer.render().unwrap();
    }
}
