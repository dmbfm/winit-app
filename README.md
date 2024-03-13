# winit-app

A simple convenience wrapper around `winit`.

# Basic Example

```rust
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
```

# wgpu

If the `wgpu` feature is enabled, you can use the `run_wgpu_app` function and
the `WinitWgpuApp` trait to run a winit application with all the basic `wgpu` boilerplate
taken care of:
```
pub fn main() -> Result<(), impl std::error::Error> {
    use winit_app::{
        run_wgpu_app, winit::window::WindowBuilder, WgpuContext, WinitAppError, WinitContext,
        WinitWgpuApp,
    };

    struct App;

    impl WinitWgpuApp for App {
        fn init(&mut self, _: &mut WinitContext, _: &mut WgpuContext) {}

        fn init_error(&mut self, error: &WinitAppError) {
            println!("Error: {}", error);
        }

        fn frame(
            &mut self,
            _: &mut WinitContext,
            wgpu_ctx: &mut WgpuContext,
            view: &wgpu::TextureView,
        ) {
            let mut enc = wgpu_ctx
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            {
                let _rpass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.541f64.powf(2.2),
                                g: 0.714f64.powf(2.2),
                                b: 0.675f64.powf(2.2),
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
            }

            wgpu_ctx.queue.submit(Some(enc.finish()));
        }

        fn event(
            &mut self,
            _: &mut WinitContext,
            _: &mut WgpuContext,
            _: winit::event::WindowEvent,
        ) {
        }

        fn will_close(&mut self, _: &mut WinitContext, _: &mut WgpuContext) {}
    }

    run_wgpu_app(WindowBuilder::new().with_title("wgpu window"), App)
}

```
