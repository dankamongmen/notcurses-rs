// notcurses::error
//
//!
//

use std::fmt;

use libnotcurses_sys::NcError;

/// The Notcurses `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

/// The Notcurses `Error` type.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A `libnotcurses-sys` error.
    NcError(NcError),

    /// A generic error message (WIP).
    Message(String),
}

/// # Methods
impl Error {
    /// Returns an `Error::Message` already wraped in a `Result`.
    pub fn msg(string: &str) -> Result<()> {
        Err(Self::Message(string.into()))
    }
}

mod std_impls {
    use super::*;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use Error::*;
            match self {
                NcError(e) => e.fmt(f),
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
