//! An ergonomic & safe wrapper for the notcurses C library
//!
//! ### notcurses C API docs:
//!
//! - [Doxygen Documentation](https://nick-black.com/notcurses/html/index.html)
//! - [API reference (man pages)](https://nick-black.com/notcurses/)
//! - [Wiki](https://nick-black.com/dankwiki/index.php/Notcurses)
//! - [The Book Guide (pdf)](https://nick-black.com/htp-notcurses.pdf)
//! - [USAGE.md](https://github.com/dankamongmen/notcurses/blob/master/USAGE.md)
//!
#![allow(unused_imports, dead_code)]
#![allow(
    clippy::declare_interior_mutable_const,
    clippy::temporary_cstring_as_ptr
)]

pub mod sys {
    pub use libnotcurses_sys::*;
}

#[macro_use]
extern crate strum_macros;

mod direct;
mod error;
mod notcurses;
mod plane;
mod types;
mod visual;

pub use crate::notcurses::{Notcurses, OptionFlag, Options};
pub use direct::Direct;
pub use error::Error;
pub use plane::{Plane, PlaneOptions};
pub use types::{Align, Blitter, NcChannels, DirectModeOptions, LogLevel, NcRgb, Scale, Style};
pub use visual::{Visual, VisualOptions};
