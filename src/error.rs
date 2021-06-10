use std::{fmt, io};

/// The notcurses `Error` type.
#[derive(Debug)]
pub enum Error {
    /// An IO error.
    IoError(io::Error),

    /// A libnotcurses-sys error.
    NcError { int: i32, msg: String },
    // UnknownWindowSize,
    // NotUtf8Input(Vec<u8>),
    // ControlCharInText(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            IoError(err) => write!(f, "{}", err),
            NcError { int, msg } => write!(f, "NcError<{0}, {1}>", int, msg),
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

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<libnotcurses_sys::NcError> for Error {
    fn from(err: libnotcurses_sys::NcError) -> Error {
        Error::NcError {
            int: err.int,
            msg: err.msg,
        }
    }
}

/// The notcurses `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
