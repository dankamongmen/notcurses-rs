//! A Rust idiomatic wrapper over the notcurses C library
//!
//! If you prefer a unsafe wrapper with an API closer to the original one,
//! you can use [libnotcurses-sys](https://crates.io/crates/libnotcurses-sys)
//!
#![allow(unused_imports, dead_code)]
#![allow(
    clippy::declare_interior_mutable_const,
    clippy::temporary_cstring_as_ptr
)]

pub mod sys {
    //! `libnotcurses-sys`
    //! low-level & unsafe Rust bindings for the notcurses C library
    pub use libnotcurses_sys::*;
}

#[macro_use]
extern crate strum_macros;

mod directmode;
mod error;
mod fullmode;
mod plane;
mod types;
mod visual;

pub use directmode::{DirectMode, DirectModeOptions};
pub use error::Error;
pub use fullmode::{FullMode, FullModeFlag, FullModeOptions};
pub use plane::{Plane, PlaneOptions};
pub use types::{Align, Blitter, LogLevel, NcChannels, NcRgb, Scale, Style};
pub use visual::{Visual, VisualOptions};
