// notcurses::lib
//
//! A simple high level notcurses wrapper.
//!
//!
//! ### Example
#![doc = concat!["```\n", include_str!("../examples/hello-world.rs"), "\n```" ]]
//
#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch
)]

mod color;
mod error;
mod input;
mod macros;
mod notcurses;
mod plane;
mod tuples;
mod visual;

pub use self::notcurses::{Capabilities, LogLevel, Notcurses, NotcursesBuilder, Statistics};
pub use color::{Alpha, Channel, Channels, Palette, Rgb, Rgba};
pub use error::{Error, Result};
pub use input::Input;
pub use plane::{Align, Cell, Plane, PlaneBuilder, PlaneGeometry, Style};
pub use tuples::{Position, Size};
pub use visual::{Blitter, PixelImplementation, Scale, Visual, VisualBuilder, VisualGeometry};

// reexports

pub use sys::sleep;

pub(crate) use sys::from_primitive;
pub(crate) use sys::unit_impl_fmt;
pub(crate) use sys::unit_impl_ops;

#[rustfmt::skip]
macro_rules! reexport_doc { ($name:literal, $sysname:literal) => { concat![
    "\n\n---\n---\n\n_(`", $name , "` is actually `sys::", $sysname,
    "` reexported)_\n\n# Original documentation for `", $sysname ,"`:\n---\n"
]}}

/// Reexport of [`libnotcurses-sys`](https://docs.rs/libnotcurses-sys).
///
/// ---
#[doc(inline)]
pub use libnotcurses_sys as sys;

/// A received [`Input`] event.
#[doc = reexport_doc!("Received", "NcReceived")]
pub use sys::NcReceived as Received;

/// The type of the [`Input`] event.
#[doc = reexport_doc!("InputType", "NcInputType")]
pub use sys::NcInputType as InputType;

/// A synthesized [`Received`] event other than a `char`.
#[doc = reexport_doc!("Key", "NcKey")]
pub use sys::NcKey as Key;

/// A bitmask of mice events.
#[doc = reexport_doc!("MouseInput", "NcMiceEvents")]
pub use sys::NcMiceEvents as MouseInput;

/// [`Key`] modifiers bitflag.
#[doc = reexport_doc!("KeyMod", "NcKeyMod")]
pub use sys::NcKeyMod as KeyMod;
