#![allow(dead_code)]

use std::fmt;

use crate::{
    sys::{NcChannels, NcChannelsMethods},
    Channel, Rgb,
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
    /// Returns a black `Channels` configured to show the default colors.
    fn default() -> Self {
        Self::with_default(0_u32, 0_u32)
    }
}

impl fmt::Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "0x{0:08X}_{1:08x}",
            (self.0 & 0xFFFFFFFF00000000) >> 32_i32,
            self.0 & 0xFFFFFFFF
        )
    }
}

// NcChannels Conversions

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

// Rgb array, slice & tuple Conversions

impl<RGB> From<[RGB; 2]> for Channels
where
    RGB: Into<Rgb> + Copy,
{
    fn from(rgb_arr: [RGB; 2]) -> Channels {
        Channels::new(rgb_arr[0], rgb_arr[1])
    }
}
impl<RGB> From<&[RGB]> for Channels
where
    RGB: Into<Rgb> + Copy,
{
    fn from(rgb_slice: &[RGB]) -> Channels {
        Channels::new(rgb_slice[0], rgb_slice[1])
    }
}

impl<FgRgb, BgRgb> From<(FgRgb, BgRgb)> for Channels
where
    FgRgb: Into<Rgb>,
    BgRgb: Into<Rgb>,
{
    fn from(rgb_tuple: (FgRgb, BgRgb)) -> Channels {
        Channels::new(rgb_tuple.0, rgb_tuple.1)
    }
}

impl Channels {
    // constructors

    /// New `Channels`.
    pub fn new<F, B>(fg: F, bg: B) -> Self
    where
        F: Into<Rgb>,
        B: Into<Rgb>,
    {
        Self(NcChannels::combine(
            Channel::new(fg.into()).into(),
            Channel::new(bg.into()).into(),
        ))
    }

    /// New `Channels` marked as using the "default color".
    pub fn with_default<F, B>(fg: F, bg: B) -> Self
    where
        F: Into<Channel>,
        B: Into<Channel>,
    {
        Self(NcChannels::combine(fg.into().into(), bg.into().into()).set_default())
    }

    // New NcChannels, expects three RGB [`NcComponent`][sys::NcComponent]s
    // per channel.
}

// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // use crate::Channels;

    // #[test]
    // fn channel_from_rgb() {
    //     assert_eq!(Channel(0x112233), 0x112233.into());
    // }
    //
    // #[test]
    // fn channel_new_not_default() {
    //     // check it marks as NOT using the default color
    //     assert_eq!(Channel(0x40_112233), Channel::new(0x112233));
    // }
}
