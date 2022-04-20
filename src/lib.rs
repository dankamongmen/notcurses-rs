// notcurses::lib
//
//! A simple high level notcurses wrapper.
//

#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch
)]

mod error;
mod geometry;
mod notcurses;
mod plane;
mod visual;

pub use self::notcurses::{Capabilities, Notcurses};
pub use error::{Error, Result};
pub use geometry::PixelGeometry;
pub use plane::{Plane, PlaneBuilder};
pub use visual::{Visual, VisualBuilder};

// reexports

#[rustfmt::skip]
macro_rules! reexport_doc { ($name:literal, $sysname:literal) => { concat![
    "\n\n---\n---\n\n_(`", $name , "` is actually `sys::", $sysname,
    "` reexported)_\n\n# Original documentation for `", $sysname ,"`:\n---\n"
]}}

/// Reexport of [`libnotcurses-sys`](https://crates.io/crates/libnotcurses-sys).
///
/// ---
#[doc(inline)]
pub use libnotcurses_sys as sys;

/// Alignment within a [`Plane`] or terminal.
#[doc = reexport_doc!("Align", "NcAlign")]
pub use sys::NcAlign as Align;

/// Alpha information, part of a *channel*, applies to a [`Cell`]'s fg|bg color.
#[doc = reexport_doc!("Alpha", "NcAlpha")]
pub use sys::NcAlpha as Alpha;

/// Blitter mode to use for rasterizing a [`Visual`].
#[doc = reexport_doc!("Blitter", "NcBlitter")]
pub use sys::NcBlitter as Blitter;

/// Indicates how to scale a [`Visual`] during rendering.
#[doc = reexport_doc!("Scale", "NcScale")]
pub use sys::NcScale as Scale;

/// A bitmap of styles.
#[doc = reexport_doc!("Style", "NcStyle")]
pub use sys::NcStyle as Style;
