// notcurses::error
//
//!
//

use std::fmt;

use libnotcurses_sys::NcError as SysNcError;

/// The Notcurses `Result` type.
pub type NcResult<T> = std::result::Result<T, NcError>;

/// The Notcurses `Error` type.
#[derive(Debug)]
#[non_exhaustive]
pub enum NcError {
    /// A libnotcurses-sys error.
    Sys(SysNcError),

    /// A generic error message (WIP).
    Message(String),
}

/// # Methods
impl NcError {
    /// Returns an `Error::Message` already wraped in a `Result`.
    pub fn msg(string: &str) -> NcResult<()> {
        Err(Self::Message(string.into()))
    }
}

mod std_impls {
    use super::*;

    impl fmt::Display for NcError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use NcError::*;
            match self {
                Sys(e) => e.fmt(f),
                Message(string) => write!(f, "Message: {}", string),
            }
        }
    }

    impl From<SysNcError> for NcError {
        fn from(e: SysNcError) -> Self {
            Self::Sys(e)
        }
    }
}
