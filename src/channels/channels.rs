#![allow(dead_code)]

use std::fmt;

use crate::{
    sys::{NcChannels, NcChannelsMethods},
    Channel,
};

/// A `u64` of foreground [`Channel`] + background [`Channel`].
///
/// # Diagram
///
/// ```txt
/// ~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB|~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB
/// ↑↑↑↑↑↑↑↑↑↑↑↑ foreground ↑↑↑↑↑↑↑↑↑↑↑|↑↑↑↑↑↑↑↑↑↑↑↑ background ↑↑↑↑↑↑↑↑↑↑↑
///                channel                            channel
/// ```
///
/// See also: [`Cell`][crate::Cell].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Channels(pub NcChannels);

impl Default for Channels {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl fmt::Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "0x{0:08X}_{1:08x}",
            (self.0 & 0xFFFFFFFF00000000) >> 32,
            self.0 & 0xFFFFFFFF
        )
    }
}

impl From<Channels> for NcChannels {
    fn from(cp: Channels) -> NcChannels {
        cp.0
    }
}

impl From<&Channels> for NcChannels {
    fn from(c: &Channels) -> NcChannels {
        c.0
    }
}
impl From<&mut Channels> for NcChannels {
    fn from(c: &mut Channels) -> NcChannels {
        c.0
    }
}

impl From<NcChannels> for Channels {
    fn from(nc: NcChannels) -> Channels {
        Channels(nc)
    }
}

impl Channels {
    // constructors

    /// New `Channels`.
    pub fn new<C1, C2>(fg: C1, bg: C2) -> Self
    where
        C1: Into<Channel>,
        C2: Into<Channel>,
    {
        Self(NcChannels::combine(fg.into().into(), bg.into().into()))
    }

    /// New `Channels` marked as using the "default color".
    pub fn with_default<C1, C2>(fg: C1, bg: C2) -> Self
    where
        C1: Into<Channel>,
        C2: Into<Channel>,
    {
        Self(NcChannels::combine(fg.into().into(), bg.into().into()).set_default())
    }

    // New NcChannels, expects three RGB [`NcComponent`][sys::NcComponent]s
    // per channel.
}
