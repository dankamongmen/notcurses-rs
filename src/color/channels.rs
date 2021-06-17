#![allow(dead_code)]

use crate::{
    sys::{NcChannels, NcChannelsMethods},
    Alpha, Rgb, Channel,
};

/// A `u64`containing 2x [`Channel`]s.
///
/// *A wrapper around [`NcChannels`].*
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Channels(pub NcChannels);

impl Default for Channels {
    fn default() -> Self {
        Self::new()
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

    /// New `Channels`, set to black and NOT using the "default color".
    pub fn new() -> Self {
        Self(NcChannels::new())
    }

    /// New `Channels`, set to black but using the "default color".
    pub fn with_default() -> Self {
        Self(NcChannels::with_default())
    }

    /// New `Channels`, expects two separate [`Rgb`]s for the foreground
    /// and background `Channel`s.
    pub fn from_rgb<RGB1, RGB2>(fg: RGB1, bg: RGB2) -> Self
    where
        RGB1: Into<Rgb>,
        RGB2: Into<Rgb>,
    {
        Self(NcChannels::from_rgb(fg.into().into(), bg.into().into()))
    }

    // New NcChannels, expects three RGB [`NcComponent`][sys::NcComponent]s
    // per channel.
}
