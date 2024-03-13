mod app;
mod error;

pub use app::{run_app, WinitApp, WinitContext};
pub use error::WinitAppError;
pub use winit;

#[cfg(feature = "wgpu")]
mod wgpu_app;

#[cfg(feature = "wgpu")]
pub use wgpu_app::{run_wgpu_app, WgpuContext, WinitWgpuApp};

#[cfg(feature = "wgpu")]
pub use wgpu;
