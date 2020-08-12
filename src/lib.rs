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

mod error;
mod direct;
mod notcurses;
mod plane;
mod visual;

pub use error::{
    NcError,
    NcVisualError,
};
pub use direct::NcDirect;
pub use crate::notcurses::{
    NotCurses,
    NcLogLevel,
    NcOptionFlag,
    NcOptions,
    NcStyle,
};
pub use visual::{
    NcScale,
    NcAlign,
    NcBlitter,
    NcVisualOptions,
    NcVisual,
};
