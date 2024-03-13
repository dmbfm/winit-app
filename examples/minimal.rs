use winit::window::WindowBuilder;
use winit_app::{run_app, WinitApp};

pub fn main() {
    struct MyApp;

    impl WinitApp for MyApp {
        fn frame(&mut self, _winit_ctx: &mut winit_app::WinitContext) {
            println!("frame!");
        }
    }

    run_app(
        WindowBuilder::new().with_title("winit-app: minimal example"),
        MyApp,
    )
    .expect("Something went wrong!");
}
