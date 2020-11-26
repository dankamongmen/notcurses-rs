use std::fmt;

use libnotcurses_sys as nc;

use thiserror::Error as ThisError;

/// Error enumerates all possible errors returned by this library.
#[repr(u32)] // = nc_err_e
#[derive(ThisError, Debug, Copy, Clone, PartialEq)]
pub enum Error {
    /// Temporary generic error (TBD more specific)
    #[error("ERROR: Generic (TBD more specific)")]
    Generic,

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

    /// Represents an error while trying to get a Cell
    #[error("Error fetching the cell.")]
    Cell,
}
