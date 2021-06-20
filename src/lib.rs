//! A simple, higher-level Rust wrapper for the [notcurses C library][0].
//! It depends on [`libnotcurses-sys`][1].
//!
//! ## Main API differences with `libnotcurses-sys`
//!
//! - Instead of using option structures, you now use the builder pattern
//!   to construct [`Plane`] and [`Visual`] objects.
//! - The concept of the standard plane disappears, you just use [`Plane`]s.
//! - Types have the `Drop` trait implemented so that you don't have to manually
//!   stop the [`Notcurses`] context, or to destroy [`Plane`]s or [`Visual`]s anymore.
//! - All coordinate pairs (`X`,`Y`), (`cols`,`rows`) are used in alphabetic
//!   order, either as part of the function name or as parameters.
//! - Many types have several `From` implementations in order to make it easier
//!   to use them in different contexts using `.into()`.
//! - [`Align`], [`Alpha`], [`Blitter`] and [`Scale`] are now enums.
//!   [`Style`] is a bitfield.
//! - New [`Error`] and [`Result`] types.
//!
//! [0]: https://github.com/dankamongmen/notcurses
//! [1]: https://github.com/dankamongmen/notcurses/tree/master/rust

#![deny(clippy::default_numeric_fallback)]

#[macro_use]
extern crate bitflags;

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

mod align;
mod cell;
mod channels;
mod error;
mod macros;
mod mode;
mod plane;
mod style;
mod visual;

pub use align::Align;
pub use cell::{Cell, BACKSTOP};
pub use channels::{Alpha, Channel, Channels, Rgb};
pub use error::{Error, Result};
pub use macros::*;
pub use mode::{Capabilities, Notcurses, NotcursesDirect};
pub use plane::Plane;
pub use style::Style;
pub use visual::{Blitter, Rgba, Scale, Visual};

pub mod builders {
    //! All the builders
    pub use crate::mode::{NotcursesBuilder, NotcursesDirectBuilder};
    pub use crate::plane::PlaneBuilder;
    pub use crate::visual::VisualBuilder;
}

/// Represents a dimension in rows or columns. Can't be negative.
pub type Dimension = sys::NcDim;

/// Represents an offset in rows or columns. Can be negative.
pub type Offset = sys::NcOffset;
