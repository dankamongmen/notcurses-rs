#![allow(dead_code)]

use crate::{
    sys::{NcChannel, NcChannelMethods, NcChannels, NcChannelsMethods},
    Alpha, Rgb,
};

/// A `u32`containing: 24bit RGB + 2bit alpha
///
/// *A wrapper around [`NcChannel`].*
///
/// See also [`Channels`]
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Channel(pub NcChannel);

///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Channels(pub NcChannels);

impl Default for Channel {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Channels {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------

impl From<Channel> for NcChannel {
    fn from(c: Channel) -> NcChannel {
        c.0
    }
}

impl From<Channels> for NcChannels {
    fn from(cp: Channels) -> NcChannels {
        cp.0
    }
}

// -----------------------------------------------------------------------------

// RETHINK: methods that (un)sets the default color… CHECK in practice
impl Channel {
    // constructors

    /// New `Channel`, set to black and NOT using the "default color".
    pub fn new() -> Self {
        Self(NcChannel::new())
    }

    /// New `Channel`, set to black but using the "default color".
    pub fn with_default() -> Self {
        Self(NcChannel::with_default())
    }

    /// New NcChannel, expects [`Rgb`].
    pub fn from_rgb<RGB: Into<Rgb>>(rgb: RGB) -> Self {
        Self(NcChannel::from_rgb(rgb.into().into()))
    }

    /// New NcChannel, expects [`Rgb`] & [`Alpha`].
    pub fn from_rgb_alpha<RGB: Into<Rgb>>(rgb: RGB, alpha: Alpha) -> Self {
        Self(NcChannel::from_rgb_alpha(rgb.into().into(), alpha.bits()))
    }

    // TODO:

    // methods

    // fn fcombine(&self, bchannel: NcChannel) -> NcChannels {  }
    // fn bcombine(&self, fchannel: NcChannel) -> NcChannels {  }
    //
    // fn alpha(&self) -> Alpha {  }
    // fn set_alpha(&mut self, alpha: Alpha) -> Self {  }
    //
    // fn set(&mut self, rgb: NcRgb) -> Self {  }
    //
    // fn r(&self) -> NcColor {  }
    // fn g(&self) -> NcColor {  }
    // fn b(&self) -> NcColor {  }
    // fn set_r(&mut self, r: NcColor) -> Self {  }
    // fn set_g(&mut self, g: NcColor) -> Self {  }
    // fn set_b(&mut self, b: NcColor) -> Self {  }
    //
    // fn rgb(&self) -> NcRgb {  }
    // fn set_rgb(&mut self, rgb: NcRgb) -> Self {  }
    //
    // fn default_p(&self) -> bool {  }
    // fn set_default(&mut self) -> Self {  }
    // fn set_not_default(&mut self) -> Self {  }
    //
    // fn palindex_p(&self) -> bool {  }
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
