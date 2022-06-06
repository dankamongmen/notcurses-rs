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

mod color;
mod error;
mod input;
mod macros;
mod notcurses;
mod plane;
mod tuples;
mod visual;

pub use self::notcurses::{Capabilities, Notcurses, NotcursesBuilder};
pub use color::{Channel, Channels, Palette};
pub use error::{Error, Result};
pub use input::Input;
pub use plane::{Cell, Plane, PlaneBuilder, PlaneGeometry};
pub use tuples::{Position, Size};
pub use visual::{Visual, VisualBuilder, VisualGeometry};

// reexports

pub use sys::sleep;

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

/// Alignment within a [`Plane`] or terminal.
#[doc = reexport_doc!("Align", "NcAlign")]
pub use sys::NcAlign as Align;

/// Alpha information, part of a [`Channel`].
#[doc = reexport_doc!("Alpha", "NcAlpha")]
pub use sys::NcAlpha as Alpha;

/// Blitter mode to use for rasterizing a [`Visual`].
#[doc = reexport_doc!("Blitter", "NcBlitter")]
pub use sys::NcBlitter as Blitter;

/// A received [`Input`] event.
#[doc = reexport_doc!("Received", "NcReceived")]
pub use sys::NcReceived as Received;

/// The type of the [`Input`] event.
#[doc = reexport_doc!("InputType", "NcInputType")]
pub use sys::NcInputType as InputType;

/// A synthesized [`Received`] event other than a `char`.
#[doc = reexport_doc!("Key", "NcKey")]
pub use sys::NcKey as Key;

/// Stder log level.
#[doc = reexport_doc!("LogLevel", "NcLogLevel")]
pub use sys::NcLogLevel as LogLevel;

/// A bitmask of mice events.
#[doc = reexport_doc!("MouseInput", "NcMiceEvents")]
pub use sys::NcMiceEvents as MouseInput;

/// [`Key`] modifiers bitflag.
#[doc = reexport_doc!("KeyMod", "NcKeyMod")]
pub use sys::NcKeyMod as KeyMod;

/// Pixel blitting implementations.
#[doc = reexport_doc!("PixelImplementation", "NcPixelImpl")]
pub use sys::NcPixelImpl as PixelImplementation;

/// A 24-bit RGB value.
#[doc = reexport_doc!("Rgb", "NcRgb")]
pub use sys::NcRgb as Rgb;

/// A 32-bit RGBA value.
#[doc = reexport_doc!("Rgba", "NcRgba")]
pub use sys::NcRgba as Rgba;

/// Indicates how to scale a [`Visual`] during rendering.
#[doc = reexport_doc!("Scale", "NcScale")]
pub use sys::NcScale as Scale;

/// Runtime statistics
#[doc = reexport_doc!("Statistics", "NcStats")]
pub use sys::NcStats as Statistics;

/// A bitmask of styles.
#[doc = reexport_doc!("Style", "NcStyle")]
pub use sys::NcStyle as Style;
