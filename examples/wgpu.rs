#[cfg(feature = "wgpu")]
pub fn main() -> Result<(), impl std::error::Error> {
    use winit_app::{
        run_wgpu_app_ex, winit::window::WindowBuilder, WgpuContext, WgpuContextDescriptor,
        WinitAppError, WinitContext, WinitWgpuApp,
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
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
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

    run_wgpu_app_ex(
        WindowBuilder::new().with_title("wgpu window"),
        WgpuContextDescriptor {
            preferred_surface_format: Some(winit_app::PreferredSurfaceFormat::Srgb),
            ..Default::default()
        },
        App,
    )
}

#[cfg(not(feature = "wgpu"))]
pub fn main() {}
