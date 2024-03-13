use crate::{WinitApp, WinitContext};
use std::sync::Arc;
use winit::{
    dpi::PhysicalSize,
    error::EventLoopError,
    event::WindowEvent,
    window::{Window, WindowBuilder},
};

pub struct WgpuContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    size: PhysicalSize<u32>,
    format: wgpu::TextureFormat,
}

impl WgpuContext {
    pub async fn new(window: Arc<Window>) -> Result<Self, crate::error::WinitAppError> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(crate::error::WinitAppError::WgpuRequestAdapterError)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];

        Ok(Self {
            instance,
            adapter,
            device,
            surface,
            queue,
            size,
            format,
        })
    }

    pub fn surface_size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.format
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

pub trait WinitWgpuApp {
    fn frame(
        &mut self,
        winit_ctx: &mut WinitContext,
        wgpu_ctx: &mut WgpuContext,
        surface_view: &wgpu::TextureView,
    );

    fn init(&mut self, winit_ctx: &mut WinitContext, wgpu_ctx: &mut WgpuContext) {
        let _ = winit_ctx;
        let _ = wgpu_ctx;
    }

    fn init_error(&mut self, error: &crate::WinitAppError) {
        let _ = error;
    }

    fn event(
        &mut self,
        winit_ctx: &mut WinitContext,
        wgpu_ctx: &mut WgpuContext,
        event: WindowEvent,
    ) {
        let _ = winit_ctx;
        let _ = wgpu_ctx;
        let _ = event;
    }
    fn will_close(&mut self, winit_ctx: &mut WinitContext, wgpu_ctx: &mut WgpuContext) {
        let _ = winit_ctx;
        let _ = wgpu_ctx;
    }
}

struct App<A: WinitWgpuApp> {
    app: A,
    ctx: Option<WgpuContext>,
}

impl<A: WinitWgpuApp> WinitApp for App<A> {
    fn init(&mut self, winit_ctx: &mut WinitContext) {
        match pollster::block_on(WgpuContext::new(winit_ctx.window().clone())) {
            Ok(ctx) => {
                self.ctx = Some(ctx);
                self.ctx.as_mut().unwrap().configure_surface();
                self.app.init(winit_ctx, self.ctx.as_mut().unwrap());
            }
            Err(err) => {
                self.app.init_error(&err);
                winit_ctx.exit();
            }
        }
    }

    fn frame(&mut self, winit_ctx: &mut WinitContext) {
        if let Some(ref mut ctx) = self.ctx {
            let output = match ctx.surface.get_current_texture() {
                Ok(tex) => tex,
                Err(_) => todo!(),
            };
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            self.app.frame(winit_ctx, ctx, &view);

            output.present();
        }
    }

    fn event(&mut self, winit_ctx: &mut WinitContext, event: winit::event::WindowEvent) {
        if let Some(ref mut ctx) = self.ctx {
            if let WindowEvent::Resized(new_size) = event {
                ctx.resize_surface(new_size);
            }

            self.app.event(winit_ctx, ctx, event);
        }
    }

    fn will_close(&mut self, winit_ctx: &mut WinitContext) {
        if let Some(ref mut ctx) = self.ctx {
            self.app.will_close(winit_ctx, ctx);
        }
    }
}

pub fn run_wgpu_app(
    window_builder: WindowBuilder,
    app: impl WinitWgpuApp,
) -> Result<(), EventLoopError> {
    crate::run_app(window_builder, App { app, ctx: None })
}
