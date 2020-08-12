use std::fmt;

use libnotcurses_sys as nc;

// TODO: Join together NcError & NcVisualError in NotcursesError,
// change the name of the structs from NcSomething to just Something.


use thiserror::Error as ThisError;

/// NcError enumerates all possible errors returned by this library.
#[derive(ThisError, Debug)]
pub enum NcError {
    /// Represents the error of an _init() function
    #[error("Error initializing the structure.")]
    Init,

    /// Represents an error while manipulating the cursor
    #[error("Error manipulating the cursor.")]
    Cursor,

    /// Represents an error while rendering an image
    #[error("Error rendering the image.")]
    ImageRender,

    /// Represents an error while clearing the screen
    #[error("Error clearing the screen.")]
    Clear,
    /*
    /// Represents an empty source. For example, an empty text file being given
    #[error("Source contains no data")]
    EmptySource,

    /// Represents a failure to read from input.
    #[error("Read error")]
    ReadError { source: std::io::Error },

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    */
}
// impl fmt::Display for NcError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "error: {:?}", self)
//     }
// }


#[repr(u32)] // = nc_err_e
#[derive(ThisError, Debug, Copy, Clone, PartialEq)]
pub enum NcVisualError {
    #[error("error decoding")]
    Decode = nc::nc_err_e_NCERR_DECODE as u32,

    #[error("end of file")]
    Eof = nc::nc_err_e_NCERR_EOF as u32,

    #[error("invalid argument")]
    InvalidArg = nc::nc_err_e_NCERR_INVALID_ARG as u32,

    #[error("ENOMEM")]
    Nomem = nc::nc_err_e_NCERR_NOMEM as u32,

    #[error("success")]
    Success = nc::nc_err_e_NCERR_SUCCESS as u32,

    #[error("system error")]
    System = nc::nc_err_e_NCERR_SYSTEM as u32,

    #[error("system feature not available")]
    Unimplemented = nc::nc_err_e_NCERR_UNIMPLEMENTED as u32,
}
