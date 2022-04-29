// notcurses::cell::channels
//
//!
//

use crate::sys::NcChannels;

/// A foreground and background [`Channel`]s.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Channels {
    nc: NcChannels,
}

mod std_impls {
    use super::*;

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
}

/// # Methods.
impl Channels {
    pub fn new() -> Channels {
        NcChannels::new().into()
    }

    // pub fn from_rgb() -> Channel {
    // }
}

/// # Constructors
impl Channels {}
