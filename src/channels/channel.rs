#![allow(dead_code)]

use crate::{
    sys::{NcChannel, NcChannelMethods},
    Alpha, Channels, Rgb,
};

/// A `u32` of 24bit [`Rgb`] data + 2bit [Alpha]
/// surrounded by context dependent bits.
///
/// # Diagram
/// ```txt
/// ~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB
/// ```
///
/// See also [`Channels`][crate::Channels]
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Channel(pub NcChannel);

impl Default for Channel {
    fn default() -> Self {
        Self(0)
    }
}

impl From<Channel> for NcChannel {
    fn from(c: Channel) -> NcChannel {
        c.0
    }
}
impl From<&Channel> for NcChannel {
    fn from(c: &Channel) -> NcChannel {
        c.0
    }
}
impl From<&mut Channel> for NcChannel {
    fn from(c: &mut Channel) -> NcChannel {
        c.0
    }
}
impl From<NcChannel> for Channel {
    fn from(nc: NcChannel) -> Channel {
        Channel(nc)
    }
}

impl From<Rgb> for Channel {
    fn from(rgb: Rgb) -> Channel {
        Channel(rgb.into())
    }
}

impl From<Channel> for Rgb {
    fn from(c: Channel) -> Rgb {
        Rgb(c.into())
    }
}

impl Channel {
    // constructors

    /// New [`Rgb`] `Channel`.
    pub fn new<RGB: Into<Rgb>>(rgb: RGB) -> Self {
        Self(NcChannel::from_rgb(rgb.into().into()))
    }

    /// New `Channel` marked as using the "default color".
    pub fn with_default<RGB: Into<Rgb>>(rgb: RGB) -> Self {
        Self(NcChannel::from_rgb(rgb.into().into()).set_default())
    }

    /// New `Channel` that uses the provided [`Alpha`].
    pub fn with_alpha<RGB: Into<Rgb>>(rgb: RGB, alpha: Alpha) -> Self {
        Self(NcChannel::from_rgb_alpha(rgb.into().into(), alpha.into()))
    }

    // methods

    /// Returns a new [`Channels`], by combining this `Channel` as foreground
    /// with `bchannel` as the background.
    pub fn fcombine(&self, bchannel: Channel) -> Channels {
        NcChannel::from(self).fcombine(bchannel.into()).into()
    }
    /// Returns a new [`Channels`], by combining this `Channel` as background,
    /// with `fchannel` as the foreground.
    pub fn bcombine(&self, fchannel: Channel) -> Channels {
        NcChannel::from(self).bcombine(fchannel.into()).into()
    }

    /// Returns the [`Alpha`] bits.
    pub fn alpha(&self) -> Alpha {
        NcChannel::from(self).alpha().into()
    }

    /// Sets the [`Alpha`] bits, and returns the resulting `Channel`.
    pub fn set_alpha(&mut self, alpha: Alpha) -> Self {
        NcChannel::from(self).set_alpha(alpha.into()).into()
    }

    /// Is this `Channel` using the "default color" rather than RGB/palette-indexed?
    pub fn is_default(&self) -> bool {
        NcChannel::from(self).default_p()
    }

    /// Marks this `Channel` as using its "default color", which also marks it
    /// [`OPAQUE`][crate::Alpha::OPAQUE], and returns the resulting `Channel`.
    pub fn default(&mut self) -> Self {
        NcChannel::from(self).set_default().into()
    }

    /// Marks this `Channel` as *NOT* using its "default color",
    /// and returns the resulting `Channel`.
    ///
    /// The [`new`][Channel#method.new] & [`set`][Channel#method.set]
    /// methods also marks the channel as not using the "default color".
    pub fn ndefault(&mut self) -> Self {
        NcChannel::from(self).set_not_default().into()
    }

    /// Is this `Channel` using palette-indexed color rather than RGB?
    pub fn is_palindex(&self) -> bool {
        NcChannel::from(self).palindex_p()
    }
}
