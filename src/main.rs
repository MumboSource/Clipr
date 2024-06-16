use std::env;

use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};
use tray_icon::TrayIcon;
use winit::event_loop::EventLoop;
use winit::application::ApplicationHandler;

struct App {
    tray_icon: TrayIcon,
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

    let mut path = env::current_exe().unwrap();

    path.pop();
    path.push("icon.ico");

    println!("Icon path: {:?}", path);
    let icon: Icon = Icon::from_path(path, None).unwrap();

    let tray = TrayIconBuilder::new()
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(icon)
        .build()
        .unwrap();

    // Same thing as TrayIcon a
    let b = event_loop.run_app::<App>(&mut App {tray_icon: tray});
}
