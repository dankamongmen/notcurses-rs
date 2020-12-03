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

mod cell;
pub use cell::Cell;

mod directmode;
pub use directmode::{DirectMode, DirectModeOptions};

mod error;
pub use error::Error;

mod fullmode;
pub use fullmode::{FullMode, FullModeFlag, FullModeOptions};

mod plane;
pub use plane::{Plane, PlaneOptions};

mod types;
pub use types::{Align, Blitter, Channel, ChannelPair, Char, LogLevel, Rgb, Scale, Style};

mod visual;
pub use visual::{Visual, VisualOptions};
