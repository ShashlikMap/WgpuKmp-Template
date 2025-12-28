use app_surface::{AppSurface, SurfaceFrame};
use i_slint_backend_winit::{CustomApplicationHandler, EventResult};
use map::WgpuApp;
use std::sync::Arc;
use wgpu::{Device, Queue, SurfaceConfiguration, SurfaceError, SurfaceTexture};
use wgpu_canvas::wgpu_canvas::WgpuCanvas;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

pub struct App {
    pub wgpu_app: Option<WgpuApp>,
}
impl App {
    pub fn new() -> Self {
        Self { wgpu_app: None }
    }
}

pub struct WinitAppSurface {
    pub app_surface: AppSurface,
}
impl WgpuCanvas for WinitAppSurface {
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
        self.app_surface.pre_present_notify();
    }

    fn on_post_render(&self) {
        self.app_surface.request_redraw();
    }
}

impl CustomApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) -> EventResult {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let app_view = futures_lite::future::block_on(AppSurface::new(window));
        let winit_surface = WinitAppSurface {
            app_surface: app_view,
        };

        let wgpu_state = pollster::block_on(WgpuApp::new(Box::new(winit_surface))).unwrap();
        self.wgpu_app = Some(wgpu_state);
        EventResult::Propagate
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        winit_window: Option<&Window>,
        _slint_window: Option<&slint::Window>,
        event: &WindowEvent,
    ) -> EventResult {
        if self.wgpu_app.is_none() {
            return EventResult::Propagate;
        }

        let map = self.wgpu_app.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                drop(self.wgpu_app.take());
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                // FIXME Don't resize wgpu-app if the window present(this is Slint window with incorrect size)
                // Need to divide handlers?!
                if winit_window.is_none() {
                    map.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                map.update_and_render();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                let is_pressed = key_state.is_pressed();
                if *code == KeyCode::Escape && is_pressed {
                    event_loop.exit();
                } else {
                    match code {
                        KeyCode::KeyN => if is_pressed {},
                        _ => {}
                    }
                }
            }

            _ => {}
        }
        EventResult::Propagate
    }
}
