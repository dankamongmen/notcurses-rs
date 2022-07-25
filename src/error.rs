// notcurses::error
//
//!
//

use std::{io::Error as IoError, result};

use crate::sys::NcError;

/// The *Notcurses* result type.
pub type Result<T> = result::Result<T, Error>;

/// The *Notcurses* error type.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A `libnotcurses-sys` error.
    NcError(NcError),

    IoError(IoError),

    /// A generic error message (WIP).
    Message(String),
}

/// # Methods
impl Error {
    /// Returns an `Error::Message` already wraped in a `Result`.
    pub fn msg<T>(string: &str) -> Result<T> {
        Err(Self::Message(string.into()))
    }
}

mod std_impls {
    use super::{Error, NcError};
    use std::fmt;

    impl std::error::Error for Error {}

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use Error::*;
            match self {
                NcError(e) => e.fmt(f),
                IoError(e) => e.fmt(f),
                Message(string) => write!(f, "Message: {}", string),
            }
        }
    }

    impl From<NcError> for Error {
        fn from(e: NcError) -> Self {
            Self::NcError(e)
        }
    }
}
