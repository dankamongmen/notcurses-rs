// notcurses::plane
//
//! Planes are the fundamental drawing object of notcurses.
//!
//! ## The base cell
//!
//! The base cell of a plane is used for purposes of rendering anywhere the
//! cell's grapheme cluster is empty (it has no text).
//!
//! Note that the base cell is not affected by erase
//!
//! # Colors and styles
//!
//! Setting the color or style will affect subsequent text writes to have the same.
//!
//! # The *CLI* plane
//!
//! There's only one *cli* plane per notcurses instance, and it's always the same
//! size as the screen.
//!
//! You can instantiate it either using the `Notcurses::`[`cli_plane`][Notcurses#cli_plane],
//! or the `Plane::`[`new_cli`][Plane::from_cli] methods.
//!
//! You can only get one *cli* `Plane` per `Notcurses` instance.
//!
//! There are several operations that can't be done on the standard plane.
//!
//! [`cli_plane`]: [Notcurses#cli_plane],
//! [`new_cli`]: [Plane#new_cli],

mod align;
mod builder;
mod cell;
mod geometry;
mod plane;
mod style;

pub use align::Align;
pub use builder::PlaneBuilder;
pub use cell::Cell;
pub use geometry::PlaneGeometry;
pub use plane::Plane;
pub use style::Style;
