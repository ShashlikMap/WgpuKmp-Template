use wgpu::{SurfaceError, SurfaceTexture};

/// Provides a communication between renderer and a native surface.
/// See ffi or winit-run modules for details
pub trait WgpuSurface: Send + Sync {
    fn queue(&self) -> &wgpu::Queue;
    fn config(&self) -> &wgpu::SurfaceConfiguration;
    fn device(&self) -> &wgpu::Device;
    fn get_current_texture(&self) -> Result<SurfaceTexture, SurfaceError>;

    fn on_resize(&mut self);
    fn on_pre_render(&self);
    fn on_post_render(&self);
}
