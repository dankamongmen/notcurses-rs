#![allow(dead_code)]

use crate::{
    sys::{self, NcAlphaBits, NcChannel, NcChannelMethods, NcChannels, NcChannelsMethods},
    Rgb,
};

bitflags! {
    /// 2 bits of alpha (surrounded by context dependent bits).
    /// It is part of an [`NcChannel`].
    ///
    pub struct AlphaBits: NcAlphaBits {
        /// The [`Cell`]'s foreground or background color will be a composite
        /// between its color and the corresponding colors underneath it.
        const BLEND = sys::NCALPHA_BLEND;

        /// The [`Cell`]'s foreground color will be high-contrast
        /// (relative to the computed background).
        ///
        /// Note that the background cannot be highcontrast.
        const HIGHCONTRAST = sys::NCALPHA_HIGHCONTRAST;

        /// The [`Cell`]'s foreground or background color is used unchanged.
        const OPAQUE = sys::NCALPHA_OPAQUE;

        /// The [`Cell`]'s foreground or background color is derived entirely
        /// from the `Cell`s underneath it.
        const TRANSPARENT = sys::NCALPHA_TRANSPARENT;
    }
}

impl AlphaBits {}

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
    ///
    /// Note you can also use rgb.into().
    pub fn from_rgb<RGB: Into<Rgb>>(rgb: RGB) -> Self {
        Self(NcChannel::from_rgb(rgb.into().into()))
    }

    /// New NcChannel, expects [`Rgb`] & [`AlphaBits`].
    pub fn from_rgb_alpha<RGB: Into<Rgb>>(rgb: RGB, alpha: AlphaBits) -> Self {
        Self(NcChannel::from_rgb_alpha(rgb.into().into(), alpha.bits()))
    }

    /// Sets the three background `r, g, b` components and marks the background
    /// [`Channel`] as not using the "default color".
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self(NcChannel::from_rgb8(r, g, b))
    }

    // TODO:

    //fn with_rgb8_alpha(r: NcColor, g: NcColor, b: NcColor, alpha: NcAlphaBits) -> Self { Self {  } }

    // methods

    // fn fcombine(&self, bchannel: NcChannel) -> NcChannels {  }
    // fn bcombine(&self, fchannel: NcChannel) -> NcChannels {  }
    //
    // fn alpha(&self) -> NcAlphaBits {  }
    // fn set_alpha(&mut self, alpha: NcAlphaBits) -> Self {  }
    //
    // fn set(&mut self, rgb: NcRgb) -> Self {  }
    //
    // fn rgb8(&self) -> (NcColor, NcColor, NcColor) {  }
    // fn set_rgb8(&mut self, r: NcColor, g: NcColor, b: NcColor) -> Self {  }
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
    ///
    /// *Note you can also use rgb.into().*
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
