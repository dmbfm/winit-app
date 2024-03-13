use winit_app::{winit::window::WindowBuilder, WinitApp};

struct App;

impl WinitApp for App {
    fn init(&mut self, _: &mut winit_app::WinitContext) {
        println!("init");
    }

    fn frame(&mut self, _: &mut winit_app::WinitContext) {}

    fn event(&mut self, _: &mut winit_app::WinitContext, event: winit::event::WindowEvent) {
        println!("event = {:#?}", event);
    }

    fn will_close(&mut self, _: &mut winit_app::WinitContext) {
        println!("will close");
    }
}

pub fn main() -> Result<(), impl std::error::Error> {
    println!("winit-app example");

    winit_app::run_app(WindowBuilder::new().with_title("Winit App!"), App)
}
