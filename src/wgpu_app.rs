use crate::{WinitApp, WinitAppError, WinitContext};
use std::sync::Arc;
use winit::{
    dpi::PhysicalSize,
    error::EventLoopError,
    event::WindowEvent,
    window::{Window, WindowBuilder},
};

#[derive(Debug, Default, Clone)]
pub enum PreferredSurfaceFormat {
    #[default]
    Srgb,
    NonSrgb,
    Format(wgpu::TextureFormat),
    Formats(Vec<wgpu::TextureFormat>),
}

#[derive(Debug, Default, Clone)]
pub struct WgpuContextDescriptor {
    pub preferred_surface_format: Option<PreferredSurfaceFormat>,
    pub power_preference: Option<wgpu::PowerPreference>,
    pub required_features: Option<wgpu::Features>,
    pub required_limits: Option<wgpu::Limits>,
}

pub struct WgpuContext {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub surface: wgpu::Surface<'static>,
    size: PhysicalSize<u32>,
    format: wgpu::TextureFormat,
}

impl WgpuContext {
    pub async fn new(
        window: Arc<Window>,
        desc: WgpuContextDescriptor,
    ) -> Result<Self, crate::error::WinitAppError> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: desc.power_preference.unwrap_or_default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(crate::error::WinitAppError::WgpuRequestAdapterError)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: desc.required_features.unwrap_or_default(),
                    required_limits: desc.required_limits.unwrap_or_default(),
                },
                None,
            )
            .await?;

        let caps = surface.get_capabilities(&adapter);

        let format = caps
            .get_format(desc.preferred_surface_format.unwrap_or_default())
            .ok_or(WinitAppError::WgpuPreferredSurfaceFormatError)?;

        Ok(Self {
            device: Arc::new(device),
            queue: Arc::new(queue),
            surface,
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
    desc: WgpuContextDescriptor,
}

impl<A: WinitWgpuApp> WinitApp for App<A> {
    fn init(&mut self, winit_ctx: &mut WinitContext) {
        match pollster::block_on(WgpuContext::new(
            winit_ctx.window().clone(),
            self.desc.clone(),
        )) {
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

trait SurfaceCapabilitiesEx {
    fn get_first_srgb(&self) -> Option<wgpu::TextureFormat>;
    fn get_first_non_srgb(&self) -> Option<wgpu::TextureFormat>;
    fn get_format_or_first(&self, format: wgpu::TextureFormat) -> wgpu::TextureFormat;
    fn get_one_of_formats_or_first(&self, formats: &[wgpu::TextureFormat]) -> wgpu::TextureFormat;
    fn has_format(&self, format: wgpu::TextureFormat) -> bool;
    fn get_format(&self, preferred: PreferredSurfaceFormat) -> Option<wgpu::TextureFormat>;
}

impl SurfaceCapabilitiesEx for wgpu::SurfaceCapabilities {
    fn get_first_srgb(&self) -> Option<wgpu::TextureFormat> {
        self.formats.iter().find(|f| f.is_srgb()).copied()
    }

    fn get_first_non_srgb(&self) -> Option<wgpu::TextureFormat> {
        self.formats.iter().find(|f| !f.is_srgb()).copied()
    }

    fn has_format(&self, format: wgpu::TextureFormat) -> bool {
        self.formats.contains(&format)
    }

    fn get_format_or_first(&self, format: wgpu::TextureFormat) -> wgpu::TextureFormat {
        self.formats
            .iter()
            .find(|f| **f == format)
            .copied()
            .unwrap_or(self.formats[0])
    }

    fn get_one_of_formats_or_first(&self, formats: &[wgpu::TextureFormat]) -> wgpu::TextureFormat {
        for format in formats.iter() {
            if self.has_format(*format) {
                return *format;
            }
        }

        self.formats[0]
    }

    fn get_format(&self, preferred: PreferredSurfaceFormat) -> Option<wgpu::TextureFormat> {
        match preferred {
            PreferredSurfaceFormat::Srgb => self.get_first_srgb(),
            PreferredSurfaceFormat::NonSrgb => self.get_first_non_srgb(),
            PreferredSurfaceFormat::Format(f) => Some(self.get_format_or_first(f)),
            PreferredSurfaceFormat::Formats(formats) => {
                Some(self.get_one_of_formats_or_first(&formats))
            }
        }
    }
}

pub fn run_wgpu_app(
    window_builder: WindowBuilder,
    app: impl WinitWgpuApp,
) -> Result<(), EventLoopError> {
    crate::run_app(
        window_builder,
        App {
            app,
            ctx: None,
            desc: WgpuContextDescriptor::default(),
        },
    )
}

pub fn run_wgpu_app_ex(
    window_builder: WindowBuilder,
    desc: WgpuContextDescriptor,
    app: impl WinitWgpuApp,
) -> Result<(), EventLoopError> {
    crate::run_app(
        window_builder,
        App {
            app,
            ctx: None,
            desc,
        },
    )
}
