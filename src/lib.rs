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
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate strum_macros;

mod direct;
mod error;
mod notcurses;
mod plane;
mod types;
mod visual;

pub use crate::notcurses::{NcLogLevel, NcOptionFlag, NcOptions, NotCurses};
pub use direct::NcDirect;
pub use error::{NcError, NcVisualError};
pub use types::{Rgb, ChannelPair, NcStyle};
pub use visual::{NcAlign, NcBlitter, NcScale, NcVisual, NcVisualOptions};
