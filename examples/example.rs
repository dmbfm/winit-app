use std::sync::Arc;
use winit_app::{
    winit::window::{Window, WindowBuilder},
    WinitApp,
};

struct App;

impl WinitApp for App {
    fn init(&mut self, _window: &Arc<Window>) {
        println!("init");
    }

    fn frame(&mut self, _window: &Arc<Window>) {}

    fn event(&mut self, _window: &Arc<Window>, event: winit::event::WindowEvent) {
        println!("event: {:#?}", event);
    }

    fn will_close(&mut self, _window: &Arc<Window>) {
        println!("will close");
    }
}

pub fn main() -> Result<(), impl std::error::Error> {
    println!("winit-app example");

    winit_app::run(WindowBuilder::new().with_title("Winit App!"), App)
}
