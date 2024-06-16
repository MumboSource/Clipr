use std::env;

use tray_icon::{menu::{Menu, MenuEvent, MenuItemBuilder}, Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use winit::event_loop::EventLoop;
use winit::application::ApplicationHandler;

struct App {
    tray_icon: TrayIcon,
    api_key: String,
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

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Ok(event) = TrayIconEvent::receiver().try_recv() {
            if let TrayIconEvent::Click{button, button_state, ..} = event {
                
                match button {
                    MouseButton::Left => println!("Left"),
                    MouseButton::Middle => println!("Middle"),
                    MouseButton::Right => println!("Right"),
                }
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut path = env::current_exe().unwrap();

    path.pop();
    path.push("assets");
    path.push("icon.ico");

    println!("Icon path: {:?}", path);
    let icon: Icon = Icon::from_path(path, None).unwrap();

    let tray_menu = Menu::new();

    let upload_button = MenuItemBuilder::new()
        .text("Upload")
        .enabled(true)
        .build();

    tray_menu.append(&upload_button).unwrap();

    let tray = TrayIconBuilder::new()
        .with_tooltip("Clipr")
        .with_icon(icon)
        .build()
        .unwrap();

    let mut api_key_path = env::current_exe().unwrap();

    api_key_path.pop();
    api_key_path.push("assets");
    api_key_path.push("api_key.txt");

    let api_key = std::fs::read_to_string(api_key_path)
        .expect("Couldn't find api_key for hastebin.com in assets/api_key.txt. Please create one.");

    println!("API Key: {}", api_key);

    let _e_loop = event_loop.run_app::<App>(&mut App {tray_icon: tray, api_key: api_key});
}
