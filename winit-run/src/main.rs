use winit::event_loop::EventLoop;
use winit_run::App;

slint::include_modules!();

fn main() {
    env_logger::init();

    let app = App::new();
    let event_loop = EventLoop::with_user_event();

    slint::platform::set_platform(Box::new(
        i_slint_backend_winit::Backend::builder()
            .with_event_loop_builder(event_loop)
            .with_custom_application_handler(Box::new(app))
            .build()
            .unwrap(),
    ))
    .unwrap();

    let ui = AppWindow::new().unwrap();
    ui.run().unwrap();
}
