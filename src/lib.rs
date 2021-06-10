//! A more *rusty*, higher level wrapper for the [notcurses C library][0]
//!
//! [0]: https://github.com/dankamongmen/notcurses

pub mod sys {
    //! `libnotcurses-sys` bindings.
    //!
    //! Please refer to the [documentation in libnotcurses-sys][doc] instead of
    //! the one re-exported in this module, since Rust doesn't seem to correctly
    //! re-export the methods and traits implementations.
    //!
    //! [doc]:https://dankamongmen.github.io/notcurses/rustdoc/libnotcurses_sys
    //!
    // FIXME: the methods implementations are not shown in the re-exported docs.
    // Probably related: https://github.com/rust-lang/rust/issues/24305
    pub use libnotcurses_sys::*;
}

mod direct;
mod error;
mod macros;
mod notcurses;
mod plane;
mod visual;

// pub use self::notcurses::{NcD, NcDCaps};
pub use self::notcurses::Nc;
pub use direct::NcD;
pub use error::{Error, Result};
pub use macros::*;
pub use plane::{Plane, PlaneBuilder};
pub use visual::Visual;

// TODO: move to the appropriate modules:

#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Style: u16 {
        ///
        const BLINK= sys::ffi::NCSTYLE_BLINK as u16;

        ///
        const BOLD = sys::ffi::NCSTYLE_BOLD as u16;

        ///
        const DIM = sys::ffi::NCSTYLE_DIM as u16;

        ///
        const INVIS = sys::ffi::NCSTYLE_INVIS as u16;

        ///
        const ITALIC = sys::ffi::NCSTYLE_ITALIC as u16;

        ///
        const MASK = sys::ffi::NCSTYLE_MASK as u16;

        ///
        const NONE = sys::ffi::NCSTYLE_NONE as u16;

        ///
        const PROTECT = sys::ffi::NCSTYLE_PROTECT as u16;

        ///
        const REVERSE = sys::ffi::NCSTYLE_REVERSE as u16;

        ///
        const STANDOUT = sys::ffi::NCSTYLE_STANDOUT as u16;

        ///
        const STRUCK = sys::ffi::NCSTYLE_STRUCK as u16;

        ///
        const UNDERLINE = sys::ffi::NCSTYLE_UNDERLINE as u16;
    }
}

/// Represents a dimension in rows or columns. Can't be negative.
pub type Dimension = sys::NcDim;

/// Represents an offset in rows or columns. Can be negative.
pub type Offset = sys::NcOffset;
