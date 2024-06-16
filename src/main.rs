use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};
use winit::event_loop::EventLoop;
use winit::application::ApplicationHandler;

struct App {

}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {

    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {

    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let icon: Icon = Icon::from_path("D:\\Code\\RUST\\Clipr\\src\\icon.ico", None).unwrap();

    TrayIconBuilder::new()
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(icon)
        .build()
        .unwrap();

    let _ = event_loop.run_app::<App>(&mut App {});
}
