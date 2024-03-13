use std::sync::Arc;
use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::EventLoopBuilder,
    window::{Window, WindowBuilder},
};

pub struct WinitContext<'a> {
    window: &'a Arc<Window>,
    should_quit: bool,
}

impl<'a> WinitContext<'a> {
    pub fn new(window: &'a Arc<Window>) -> Self {
        Self {
            window,
            should_quit: false,
        }
    }
    pub fn window(&self) -> &Arc<Window> {
        self.window
    }

    pub fn exit(&mut self) {
        self.should_quit = true;
    }
}

pub trait WinitApp {
    fn frame(&mut self, winit_ctx: &mut WinitContext);
    fn init(&mut self, winit_ctx: &mut WinitContext) {
        let _ = winit_ctx;
    }
    fn event(&mut self, winit_ctx: &mut WinitContext, event: WindowEvent) {
        let _ = winit_ctx;
        let _ = event;
    }
    fn will_close(&mut self, winit_ctx: &mut WinitContext) {
        let _ = winit_ctx;
    }
}

pub fn run_app(
    window_builder: WindowBuilder,
    mut app: impl WinitApp,
) -> Result<(), EventLoopError> {
    let event_loop = EventLoopBuilder::new().build()?;
    let window = window_builder.build(&event_loop)?;
    let window = Arc::new(window);

    let mut first_frame = true;

    event_loop.run(|event, target| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => {
                    let mut ctx = WinitContext::new(&window);
                    app.will_close(&mut ctx);
                    target.exit();
                }

                WindowEvent::RedrawRequested => {
                    let mut ctx = WinitContext::new(&window);
                    if first_frame {
                        app.init(&mut ctx);
                        if ctx.should_quit {
                            target.exit();
                            return;
                        }
                        first_frame = false;
                    }

                    app.frame(&mut ctx);
                    if ctx.should_quit {
                        target.exit();
                    };

                    window.request_redraw();
                }

                _ => {
                    let mut ctx = WinitContext::new(&window);
                    app.event(&mut ctx, event);
                    if ctx.should_quit {
                        target.exit();
                    }
                }
            }
        }
    })
}
