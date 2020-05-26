//! Error Handling

use crate::__gl;

use crate::device::Device;
use crate::pipeline::Shader;
use std::{error, fmt, result};

/// Error return codes
///
/// Error handling in `grr` only deals with runtime-only detectable errors.
///
/// Other error codes returned by OpenGL are either treated as API miss use (see `Valid Usage` sections),
/// or indicate driver or implementation issues.
///
/// API validation is provided by the debug functionality on device creation.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    OutOfMemory,

    /// Shader compilation failure.
    CompileError(Option<Shader>),
}

/// A specialized Result type for `grr` operations.
pub type Result<T> = result::Result<T, Error>;

impl Device {
    pub(crate) unsafe fn get_error(&self) -> Result<()> {
        let err = self.0.GetError();
        match err {
            __gl::OUT_OF_MEMORY => Err(Error::OutOfMemory),
            _ => Ok(()),
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Error::OutOfMemory => write!(fmt, "OutOfMemory"),
            Error::CompileError(_) => write!(fmt, "CompileError")
        }
    }
}
