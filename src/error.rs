use std::{fmt, io};

/// The notcurses `Error` type.
#[derive(Debug)]
pub enum NotcursesError {
    /// An IO error.
    IoError(io::Error),

    /// A libnotcurses-sys error.
    NcError {
        int: i32,
        msg: String,
    },

    BuildIncomplete(String),
    // UnknownWindowSize,
    // NotUtf8Input(Vec<u8>),
    // ControlCharInText(char),
    /// A generic exit message, mainly for debugging
    ExitMessage(String),
}

impl fmt::Display for NotcursesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NotcursesError::*;
        match self {
            IoError(err) => write!(f, "{}", err),
            NcError { int, msg } => write!(f, "NcError<{0}, {1}>", int, msg),
            BuildIncomplete(string) => write!(f, "BuildIncomplete: {}", string),
            ExitMessage(string) => write!(f, "ExitMessage: {}", string),
            // UnknownWindowSize => write!(f, "Could not detect terminal window size"),
            // NotUtf8Input(seq) => {
            //     write!(f, "Cannot handle non-UTF8 multi-byte input sequence: ")?;
            //     for byte in seq.iter() {
            //         write!(f, "\\x{:x}", byte)?;
            //     }
            //     Ok(())
            // }
            // ControlCharInText(c) => write!(f, "Invalid character for text is included: {:?}", c),
        }
    }
}

impl From<io::Error> for NotcursesError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<libnotcurses_sys::NcError> for NotcursesError {
    fn from(err: libnotcurses_sys::NcError) -> Self {
        Self::NcError {
            int: err.int,
            msg: err.msg,
        }
    }
}

/// The notcurses `Result` type.
pub type NotcursesResult<T> = std::result::Result<T, NotcursesError>;
