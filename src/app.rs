use std::sync::Arc;
use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::EventLoopBuilder,
    window::{Window, WindowBuilder},
};

pub trait WinitApp {
    fn init(&mut self, window: &Arc<Window>);
    fn frame(&mut self, window: &Arc<Window>);
    fn event(&mut self, window: &Arc<Window>, event: WindowEvent);
    fn will_close(&mut self, window: &Arc<Window>);
}

pub fn run(window_builder: WindowBuilder, mut app: impl WinitApp) -> Result<(), EventLoopError> {
    let event_loop = EventLoopBuilder::new().build()?;
    let window = window_builder.build(&event_loop)?;
    let window = Arc::new(window);

    let mut first_frame = true;

    event_loop.run(|event, target| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => {
                    app.will_close(&window);
                    target.exit();
                }

                WindowEvent::RedrawRequested => {
                    if first_frame {
                        app.init(&window);
                        first_frame = false;
                    }

                    app.frame(&window);
                    window.request_redraw();
                }

                _ => {
                    app.event(&window, event);
                }
            }
        }
    })
}
