mod wrapped_std_io;

use crate::consts::*;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{} {:?}", msg::ERR_FILE_READ, 0)]
    FileRead(wrapped_std_io::Error),
    #[error("{} {:?}", msg::ERR_FILE_WRITE, 0)]
    FileWrite(wrapped_std_io::Error),
}

impl Error {
    pub fn file_read<TError>(error: TError) -> Self where TError: Into<wrapped_std_io::Error> {
        Self::FileRead(error.into())
    }

    pub fn file_write<TError>(error: TError) -> Self where TError: Into<wrapped_std_io::Error> {
        Self::FileWrite(error.into())
    }
}