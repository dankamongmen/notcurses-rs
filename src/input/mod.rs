// notcurses::input
//
//!
//

mod input;

use crate::sys;
pub use input::Input;

/// A received [`Input`] event.
pub use sys::NcReceived as Received;

/// The type of the [`Input`] event.
pub use sys::NcInputType as InputType;

/// A synthesized [`Received`] event other than a `char`.
pub use sys::NcKey as Key;

/// A bitmask of mice events.
pub use sys::NcMiceEvents as MouseInput;

/// [`Key`] modifiers bitflag.
pub use sys::NcKeyMod as KeyMod;
