#![allow(dead_code)]

use std::fmt;

use crate::{
    sys::{NcChannel, NcChannelMethods},
    Alpha, Channels, Rgb,
};

/// A `u32` of 24bit [`Rgb`] data + 2bit [Alpha] + context dependent bits,
/// part of [`Channels`].
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
    /// Returns a black `Channel` configured to show the default colors.
    fn default() -> Self {
        Self::with_default(0_u32)
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "0x{:016X}", self.0)
    }
}

// NcChannel Conversions

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

// Rgb Conversions

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

impl<C> From<&[C]> for Channel
where
    C: Into<Channel> + Copy,
{
    fn from(c: &[C]) -> Channel {
        Channel(c[0].into().into())
    }
}

impl<C> From<[C; 1]> for Channel
where
    C: Into<Channel> + Copy,
{
    fn from(c: [C; 1]) -> Channel {
        Channel(c[0].into().into())
    }
}

impl Channel {
    // constructors

    /// New [`Rgb`] `Channel`. Marked as NOT using the "default color".
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
    /// [`OPAQUE`][crate::Alpha::Opaque], and returns the resulting `Channel`.
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

// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::Channel;

    #[test]
    fn channel_from_rgb() {
        assert_eq!(Channel(0x112233), 0x112233.into());
    }

    #[test]
    fn channel_new_not_default() {
        // check it marks as NOT using the default color
        assert_eq!(Channel(0x40_112233), Channel::new(0x112233));
    }
}
