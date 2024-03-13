use std::sync::Arc;
use winit_app::{
    winit::{dpi::PhysicalSize, event::WindowEvent, window::WindowBuilder},
    WinitApp, WinitContext,
};

#[derive(Debug, Default)]
struct App {
    state: Option<State>,
}

#[derive(Debug)]
struct State {
    device: wgpu::Device,
    surface: wgpu::Surface<'static>,
    queue: wgpu::Queue,
    size: PhysicalSize<u32>,
    format: wgpu::TextureFormat,
}

impl State {
    pub async fn new(window: Arc<winit::window::Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance
            .create_surface(window)
            .expect("Failed to crate surface!");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to create adapter!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device!");

        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];

        Self {
            device,
            surface,
            queue,
            size,
            format,
        }
    }

    pub fn resize_surface(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.configure_surface();
        }
    }

    pub fn configure_surface(&mut self) {
        self.surface.configure(
            &self.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.format,
                width: self.size.width,
                height: self.size.height,
                present_mode: wgpu::PresentMode::Fifo,
                desired_maximum_frame_latency: 2,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
            },
        );
    }
}

impl WinitApp for App {
    fn init(&mut self, winit_ctx: &mut WinitContext) {
        self.state = Some(pollster::block_on(State::new(winit_ctx.window().clone())));
        self.state.as_mut().unwrap().configure_surface();
    }

    fn frame(&mut self, _: &mut WinitContext) {
        let Some(ref mut state) = self.state else {
            return;
        };

        let output = match state.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Lost) => todo!(),
            Err(wgpu::SurfaceError::Timeout) => todo!(),
            Err(wgpu::SurfaceError::Outdated) => todo!(),
            Err(wgpu::SurfaceError::OutOfMemory) => todo!(),
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut enc = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let _rpass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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

        state.queue.submit(Some(enc.finish()));
        output.present();
    }

    fn event(&mut self, _: &mut WinitContext, event: winit::event::WindowEvent) {
        let Some(ref mut state) = self.state else {
            return;
        };

        #[allow(clippy::single_match)]
        match event {
            WindowEvent::Resized(new_size) => {
                state.resize_surface(new_size);
            }

            _ => {}
        }
    }

    fn will_close(&mut self, _: &mut WinitContext) {
        println!("will close");
    }
}

pub fn main() -> Result<(), impl std::error::Error> {
    println!("winit-app example");

    winit_app::run_app(
        WindowBuilder::new().with_title("Winit App!"),
        App::default(),
    )
}
