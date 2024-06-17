use std::env;

use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use winit::event_loop::EventLoop;
use winit::application::ApplicationHandler;

use clipboard::{ClipboardContext, ClipboardProvider};
use reqwest::blocking::Client;

use serde::{Deserialize, Serialize};
use serde_json::Result;

struct App {
    tray_icon: TrayIcon,
    api_key: String,
    http_client: Client,
    clipboard_context: ClipboardContext
}

#[derive(Serialize, Deserialize)]
struct HashResponse {
    key: String
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        
    }

    fn window_event(
            &mut self,
            _event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            _event: winit::event::WindowEvent,
        ) {
            
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Ok(event) = TrayIconEvent::receiver().try_recv() {
            if let TrayIconEvent::Click{button, button_state, ..} = event {
                
                match button {
                    MouseButton::Left => {
                        if button_state == MouseButtonState::Up {
                            
                            let contents = self.clipboard_context.get_contents().unwrap();

                            println!("Posting to hastebin.com");
                            let hastebin_response = self.http_client.post("https://hastebin.com/documents")
                                .header("Content-Type", "text/plain")
                                .header("Authorization", format!("Bearer {}", self.api_key))
                                .body(contents)
                                .send()
                                .unwrap();


                            let contents : HashResponse = serde_json::from_str(&hastebin_response.text().unwrap()).unwrap();

                            self.clipboard_context.set_contents(format!("https://hastebin.com/share/{}", contents.key)).expect("Couldn't set clipboard contents");
                        }
                    },

                    _ => {}
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

    let http_client = Client::new();
    let mut clipboard_context: ClipboardContext = ClipboardProvider::new().unwrap();

    let _e_loop = event_loop.run_app::<App>(&mut App {
        tray_icon: tray, 
        api_key: api_key, 
        http_client: http_client, 
        clipboard_context: clipboard_context
    });

}
