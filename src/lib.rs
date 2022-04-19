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

/// Reexport of [`libnotcurses-sys`](https://crates.io/crates/libnotcurses-sys).
///
/// ---
///
#[doc(inline)]
pub use libnotcurses_sys as sys;
pub use sys::NcAlign as Align;
pub use sys::NcAlpha as Alpha;
pub use sys::NcBlitter as Blitter;
pub use sys::NcScale as Scale;

mod error;
mod geometry;
mod notcurses;
mod plane;
mod visual;

pub use self::notcurses::{Capabilities, Notcurses};
pub use error::{Error, Result};
pub use geometry::Geometry;
pub use plane::{Plane, PlaneBuilder};
pub use visual::Visual;
