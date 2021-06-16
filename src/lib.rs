//! A simple *rusty* wrapper for the [notcurses C library][0]
//!
//! [0]: https://github.com/dankamongmen/notcurses

#![deny(clippy::default_numeric_fallback)]

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
    // Probably related to https://github.com/rust-lang/rust/issues/24305
    pub use libnotcurses_sys::*;
}

mod capabilities;
mod channel;
mod direct;
mod error;
mod macros;
mod notcurses;
mod plane;
mod rgb;
mod style;
mod visual;

pub use self::notcurses::Notcurses;
pub use self::rgb::{Rgb, Rgba};
pub use capabilities::Capabilities;
pub use channel::{AlphaBits, Channel, Channels};
pub use direct::NotcursesDirect;
pub use error::{Error, Result};
pub use macros::*;
pub use plane::{Plane, PlaneBuilder};
pub use style::Style;
pub use visual::{Blitter, Scale, Visual, VisualBuilder};

/// Represents a dimension in rows or columns. Can't be negative.
pub type Dimension = sys::NcDim;

/// Represents an offset in rows or columns. Can be negative.
pub type Offset = sys::NcOffset;

#[macro_use]
extern crate bitflags;
bitflags! {
    /// Represents the alignment within a plane or terminal.
    /// Either left/right-justified, centered, or unaligned.
    pub struct Align: u32 {
        const NCALIGN_LEFT = sys::NCALIGN_LEFT;
        const NCALIGN_RIGHT = sys::NCALIGN_RIGHT;
        const NCALIGN_CENTER = sys::NCALIGN_CENTER;
        const NCALIGN_UNALIGNED = sys::NCALIGN_UNALIGNED;
    }
}
