extern crate core;

use renderer::{Renderer, WgpuRenderer};
use wgpu_canvas::wgpu_canvas::WgpuCanvas;
pub struct WgpuApp {
    renderer: Box<WgpuRenderer>,
    screen_params: ScreenParam,
}

struct ScreenParam {
    width: u32,
    height: u32,
}
impl WgpuApp {
    pub async fn new(
        canvas: Box<dyn WgpuCanvas>,
    ) -> anyhow::Result<WgpuApp> {
        let screen_size = (canvas.config().width as f32, canvas.config().height as f32);

        let renderer = WgpuRenderer::new(canvas).await?;

        let map = WgpuApp {
            renderer: Box::new(renderer),
            screen_params: ScreenParam {
                width: screen_size.0 as u32,
                height: screen_size.1 as u32,
            },
        };
        Ok(map)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
        self.screen_params.width = width;
        self.screen_params.height = height;
    }

    pub fn update_and_render(&mut self) {

        self.renderer.update(
        );

        self.renderer.render().unwrap();
    }
}
