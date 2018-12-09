//! Error Handling
//!
//! Error handling in `grr` only deals with runtime-only detectable errors.
//!
//! Other error codes returned by OpenGL are either treated as API miss use (see `Valid Usage` sections),
//! or indicate driver or implementation issues.
//!
//! API validation is provided by the debug functionality on device creation.

use __gl;

use device::Device;
use std;

///
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    OutOfMemory,
}

///
pub type Result<T> = std::result::Result<T, Error>;

impl Device {
    pub(crate) fn get_error(&self) -> Result<()> {
        let err = unsafe { self.0.GetError() };
        match err {
            __gl::OUT_OF_MEMORY => Err(Error::OutOfMemory),
            _ => Ok(()),
        }
    }
}
