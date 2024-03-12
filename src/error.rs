use thiserror::*;
use winit::error::{EventLoopError, OsError};

#[derive(Debug, Error)]
pub enum WinitAppError {
    #[error("event loop error: {}", .0)]
    EventLoopError(EventLoopError),

    #[error("event loop error: {}", .0)]
    OsError(OsError),
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
