// notcurses::input
//
//!
//

mod input;
mod mice_events;
mod received;

pub use input::Input;
pub use mice_events::MiceEvents;
pub use received::Received;

use crate::sys;

/// The type of the [`Input`] event.
pub use sys::NcInputType as InputType;

/// A synthesized [`Received`] event other than a `char`.
pub use sys::NcKey as Key;

/// A bitmask of mice events.
pub use sys::NcMiceEvents as MouseInput;

/// [`Key`] modifiers bitflag.
pub use sys::NcKeyMod as KeyMod;
