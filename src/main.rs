use std::path::Path;
use std::{fs};

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

    let icon: Icon = Icon::from_path(Path::new("./src/icon.ico"), None).unwrap();

    // You have to assign it to a var that doesn't begin with underscore (ignore warning)
    let a = TrayIconBuilder::new()
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(icon)
        .build()
        .unwrap();

    // Same thing as TrayIcon a
    let b = event_loop.run_app::<App>(&mut App {});
}
