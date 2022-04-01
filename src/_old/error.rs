use std::{fmt, io};

/// The Notcurses `Error` type.
#[derive(Debug)]
#[non_exhaustive]
pub enum NError {
    /// An IO error.
    IoError(io::Error),

    /// A libnotcurses-sys error.
    NcError {
        int: i32,
        msg: String,
    },

    BuildIncomplete(String),

    /// A generic error message, mainly for debugging
    Message(String),
}

impl fmt::Display for NError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NError::*;
        match self {
            IoError(err) => write!(f, "{}", err),
            NcError { int, msg } => write!(f, "NcError<{0}, {1}>", int, msg),
            BuildIncomplete(string) => write!(f, "BuildIncomplete: {}", string),
            Message(string) => write!(f, "Message: {}", string),
        }
    }
}

impl NError {
    /// Returns an `NError::Message` already wraped in a `Result`.
    pub fn msg(string: &str) -> Result<(), Self> {
        Err(Self::Message(string.into()))
    }
}

impl From<io::Error> for NError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<libnotcurses_sys::NcError> for NError {
    fn from(err: libnotcurses_sys::NcError) -> Self {
        Self::NcError {
            int: err.int,
            msg: err.msg,
        }
    }
}

/// The Notcurses `Result` type.
pub type NResult<T> = std::result::Result<T, NError>;
