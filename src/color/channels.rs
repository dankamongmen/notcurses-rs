#![allow(dead_code)]

use crate::{
    sys::{NcChannels, NcChannelsMethods},
    Alpha, Rgb, Channel,
};

/// A `u64` composed of 2 × [`Channel`]s.
///
/// # Diagram
///
/// ```txt
/// ~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB|~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB
/// ↑↑↑↑↑↑↑↑↑↑↑↑ foreground ↑↑↑↑↑↑↑↑↑↑↑|↑↑↑↑↑↑↑↑↑↑↑↑ background ↑↑↑↑↑↑↑↑↑↑↑
///                channel                            channel
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Channels(pub NcChannels);

impl Default for Channels {
    fn default() -> Self {
        Self::new(0, 0)
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

    /// New [`Rgb`] `Channels`.
    pub fn new<RGB1, RGB2>(fg: RGB1, bg: RGB2) -> Self
    where
        RGB1: Into<Rgb>,
        RGB2: Into<Rgb>,
    {
        Self(NcChannels::from_rgb(fg.into().into(), bg.into().into()))
    }

    /// New `Channels` marked as using the "default color".
    pub fn with_default<RGB1, RGB2>(fg: RGB1, bg: RGB2) -> Self
    where
        RGB1: Into<Rgb>,
        RGB2: Into<Rgb>,
    {
        Self(NcChannels::from_rgb(fg.into().into(), bg.into().into()).set_default())
    }


    // New NcChannels, expects three RGB [`NcComponent`][sys::NcComponent]s
    // per channel.
}
