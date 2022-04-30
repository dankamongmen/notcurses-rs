// notcurses::cell::channels
//
//!
//

use crate::{
    sys::{c_api::NcChannels_u64, NcChannels},
    Channel,
};

/// A foreground and background [`Channel`]s.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Channels {
    pub nc: NcChannels,
}

mod std_impls {
    use super::*;
    use std::fmt;

    impl fmt::Display for Channels {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let (fg, bg) = self.into();
            write!(f, "[{fg}, {bg}]")
        }
    }
    impl fmt::Debug for Channels {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let (fg, bg) = self.into();
            write!(f, "fg:{fg} bg:{bg}")
        }
    }

    impl From<Channels> for NcChannels {
        fn from(c: Channels) -> NcChannels {
            c.nc
        }
    }

    impl From<NcChannels> for Channels {
        fn from(nc: NcChannels) -> Channels {
            Self { nc }
        }
    }

    impl From<NcChannels_u64> for Channels {
        fn from(nc_u64: NcChannels_u64) -> Channels {
            NcChannels::from(nc_u64).into()
        }
    }

    //

    /// Converts a `Channels` into a (fg, bg) `Channel` tuple.
    impl From<Channels> for (Channel, Channel) {
        fn from(c: Channels) -> (Channel, Channel) {
            (c.fg(), c.bg())
        }
    }
    impl From<&Channels> for (Channel, Channel) {
        fn from(c: &Channels) -> (Channel, Channel) {
            (c.fg(), c.bg())
        }
    }
}

/// # constructors
impl Channels {
    pub fn new() -> Channels {
        NcChannels::new().into()
    }

    // pub fn from_rgb() -> Channel {
    // }
}

/// # methods
impl Channels {
    /// Gets the foreground channel.
    pub fn fg(&self) -> Channel {
        self.nc.fchannel().into()
    }

    /// Gets the background channel.
    pub fn bg(&self) -> Channel {
        self.nc.bchannel().into()
    }
}
