// notcurses::error
//
//!
//

use std::{io::Error as IoError, result};

use crate::sys::NcError;

/// The *Notcurses* result type.
pub type NotcursesResult<T> = result::Result<T, NotcursesError>;

/// The *Notcurses* error type.
#[derive(Debug)]
#[non_exhaustive]
pub enum NotcursesError {
    /// A `libnotcurses-sys` error.
    NcError(NcError),

    /// An `std::io::Error`.
    IoError(IoError),

    /// An error message string.
    Message(String),
}

/// # Methods
impl NotcursesError {
    /// Returns a `NotcursesError::Message` already wraped in a `Result`.
    pub fn msg<T>(string: &str) -> NotcursesResult<T> {
        Err(Self::Message(string.into()))
    }
}

mod core_impls {
    use super::{NcError, NotcursesError};
    use core::fmt;

    impl fmt::Display for NotcursesError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                NotcursesError::NcError(e) => e.fmt(f),
                NotcursesError::IoError(e) => e.fmt(f),
                NotcursesError::Message(string) => write!(f, "Message: {}", string),
            }
        }
    }

    impl From<NcError> for NotcursesError {
        fn from(e: NcError) -> Self {
            Self::NcError(e)
        }
    }
}

mod std_impls {
    use super::NotcursesError;

    impl std::error::Error for NotcursesError {}
}
