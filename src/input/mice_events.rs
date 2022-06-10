// notcurses::input::mice_events
//
//!
//

use crate::sys::{c_api::NcMiceEvents_u32, NcMiceEvents};

/// A bitmask of mice input events.
///
/// # Used by
/// - [`Notcurses.mice_enable`][crate::Notcurses#method.mice_enable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MiceEvents(NcMiceEvents_u32);

/// # Flags
#[allow(non_upper_case_globals)]
impl MiceEvents {
    /// Disables all mice events.
    pub const None: Self = Self(NcMiceEvents::None.0);

    /// Enables mice move events.
    pub const Move: Self = Self(NcMiceEvents::Move.0);

    /// Enables mice button events.
    pub const Button: Self = Self(NcMiceEvents::Button.0);

    /// Enables mice drag events.
    pub const Drag: Self = Self(NcMiceEvents::Drag.0);

    /// Enables all mice tracking events.
    pub const All: Self = Self(NcMiceEvents::All.0);
}

mod std_impls {
    use super::{MiceEvents, NcMiceEvents, NcMiceEvents_u32};

    impl Default for MiceEvents {
        fn default() -> Self {
            Self::None
        }
    }

    crate::from_primitive![MiceEvents, NcMiceEvents_u32];
    crate::unit_impl_ops![bitwise; MiceEvents, NcMiceEvents_u32];
    crate::unit_impl_fmt![bases; MiceEvents];

    impl From<NcMiceEvents> for MiceEvents {
        fn from(nc: NcMiceEvents) -> Self {
            match nc {
                NcMiceEvents::None => MiceEvents::None,
                NcMiceEvents::Move => MiceEvents::Move,
                NcMiceEvents::Button => MiceEvents::Button,
                NcMiceEvents::Drag => MiceEvents::Drag,
                NcMiceEvents::All => MiceEvents::All,
                _ => MiceEvents::None,
            }
        }
    }
    impl From<MiceEvents> for NcMiceEvents {
        fn from(me: MiceEvents) -> Self {
            match me {
                MiceEvents::None => NcMiceEvents::None,
                MiceEvents::Move => NcMiceEvents::Move,
                MiceEvents::Button => NcMiceEvents::Button,
                MiceEvents::Drag => NcMiceEvents::Drag,
                MiceEvents::All => NcMiceEvents::All,
                _ => NcMiceEvents::None,
            }
        }
    }
}

/// # methods
impl MiceEvents {
    /// Returns true if the current mice events has `other` included.
    pub fn has(&self, other: NcMiceEvents) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Adds `other` to the current mice events.
    pub fn add(&mut self, other: NcMiceEvents) {
        self.0 |= other.0
    }
}
