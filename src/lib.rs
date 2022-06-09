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
    clippy::module_inception, // for plane & visual
    clippy::pattern_type_mismatch,
)]

use core::cell::RefCell;
use once_cell::sync::OnceCell;

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
pub use input::{Input, InputType, Key, KeyMod, MouseInput, Received};
pub use plane::{Align, Cell, Plane, PlaneBuilder, PlaneGeometry, Style};
pub use tuples::{Position, Size};
pub use visual::{Blitter, PixelImplementation, Scale, Visual, VisualBuilder, VisualGeometry};

thread_local!(
    /// Restricts initializing more than one `Notcurses` instance per thread, at the same time.
    static NOTCURSES_LOCK: RefCell<OnceCell<bool>> = RefCell::new(OnceCell::new());

    /// Restricts instancing the standard `Plane` more than once per `Notcurses` instance.
    pub(crate) static CLI_PLANE_LOCK: RefCell<OnceCell<bool>> = RefCell::new(OnceCell::new());
);

/// Reexport of [`libnotcurses-sys`](https://docs.rs/libnotcurses-sys).
///
/// ---
#[doc(inline)]
pub use libnotcurses_sys as sys;

pub use sys::sleep;

pub(crate) use sys::from_primitive;
pub(crate) use sys::unit_impl_fmt;
pub(crate) use sys::unit_impl_ops;
