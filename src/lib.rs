// notcurses::lib
//
//! A rusty notcurses wrapper.
//!
//!
//! ### Example
#![doc = concat!["```\n", include_str!("../examples/hello-world.rs"), "\n```" ]]
//!
//

#![warn(clippy::all)]
#![allow(clippy::module_inception, non_upper_case_globals)]

use core::cell::RefCell;
use once_cell::sync::OnceCell;

mod color;
mod error;
mod input;
mod macros;
mod notcurses;
mod plane;
mod visual;

pub use self::notcurses::{Capabilities, LogLevel, Notcurses, NotcursesBuilder, Statistics};
pub use color::{Alpha, Channel, Channels, Palette, Rgb, Rgba};
pub use error::{Error, Result};
pub use input::{Input, InputType, Key, KeyMod, MiceEvents, Received};
pub use plane::{Align, Cell, Plane, PlaneBuilder, PlaneGeometry, Position, Size, Style};
pub use visual::{Blitter, PixelImplementation, Scale, Visual, VisualBuilder, VisualGeometry};

//

thread_local!(
    /// Restricts initializing more than one `Notcurses` instance per thread, at the same time.
    static NOTCURSES_LOCK: RefCell<OnceCell<bool>> = RefCell::new(OnceCell::new());

    /// Restricts instancing the standard `Plane` more than once per `Notcurses` instance.
    static CLI_PLANE_LOCK: RefCell<OnceCell<bool>> = RefCell::new(OnceCell::new());
);

/// Reexport of [`libnotcurses-sys`](https://docs.rs/libnotcurses-sys).
///
/// ---
#[doc(inline)]
pub use libnotcurses_sys as sys;

pub(crate) use sys::from_primitive;
pub(crate) use sys::unit_impl_fmt;
pub(crate) use sys::unit_impl_ops;
