use thiserror::*;
use winit::error::{EventLoopError, OsError};

#[derive(Debug, Error)]
pub enum WinitAppError {
    #[error("event loop error: {}", .0)]
    EventLoopError(EventLoopError),

    #[error("event loop error: {}", .0)]
    OsError(OsError),

    #[cfg(feature = "wgpu")]
    #[error("wgpu error: {}", .0)]
    WgpuCreateSurfaceError(wgpu::CreateSurfaceError),

    #[cfg(feature = "wgpu")]
    #[error("wgpu error: {}", .0)]
    WgpuSurfaceError(wgpu::SurfaceError),

    #[cfg(feature = "wgpu")]
    #[error("wgpu error: {}", .0)]
    WgpuRequestDeviceError(wgpu::RequestDeviceError),

    #[cfg(feature = "wgpu")]
    #[error("wgpu error: failed to request adapter")]
    WgpuRequestAdapterError,

    #[cfg(feature = "wgpu")]
    #[error("wgpu error: failed to create surface with preferred format")]
    WgpuPreferredSurfaceFormatError,
}

impl From<EventLoopError> for WinitAppError {
    fn from(value: EventLoopError) -> Self {
        Self::EventLoopError(value)
    }
}

impl From<OsError> for WinitAppError {
    fn from(value: OsError) -> Self {
        Self::OsError(value)
    }
}

#[cfg(feature = "wgpu")]
impl From<wgpu::CreateSurfaceError> for WinitAppError {
    fn from(value: wgpu::CreateSurfaceError) -> Self {
        Self::WgpuCreateSurfaceError(value)
    }
}

#[cfg(feature = "wgpu")]
impl From<wgpu::SurfaceError> for WinitAppError {
    fn from(value: wgpu::SurfaceError) -> Self {
        Self::WgpuSurfaceError(value)
    }
}

#[cfg(feature = "wgpu")]
impl From<wgpu::RequestDeviceError> for WinitAppError {
    fn from(value: wgpu::RequestDeviceError) -> Self {
        Self::WgpuRequestDeviceError(value)
    }
}
