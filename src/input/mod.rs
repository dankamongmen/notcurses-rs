// notcurses::input
//
//!
//

mod input;
mod input_type;
mod key_mod;
mod mice_events;
mod received;

pub use input::Input;
pub use input_type::InputType;
pub use key_mod::KeyMod;
pub use mice_events::MiceEvents;
pub use received::Received;

/// A synthesized [`Received`] event other than a `char`.
pub use crate::sys::NcKey as Key;
