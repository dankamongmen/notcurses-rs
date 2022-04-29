// notcurses::cell::channel
//
//!
//

use crate::{sys::NcChannel, Alpha, Rgb};

/// [`Rgb`] + [`Alpha`]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Channel {
    pub nc: NcChannel,
}

mod std_impls {
    use super::*;
    use std::fmt;

    impl fmt::Debug for Channel {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut is = String::new();
            if self.is_rgb() {
                is += "rgb";
            };
            if self.is_default() {
                is += "defaultcolor";
            }
            if self.is_palindex() {
                is += "palindex";
            }
            let alpha = self.alpha();

            let mut s = String::new();
            if self.is_rgb() {
                s += &format![
                    "Channel {{{is} {:02X} {:02X} {:02X}}}",
                    self.r(),
                    self.g(),
                    self.b()
                ];
            } else if self.is_palindex() {
                s += &format!["Channel {{{is} {:03}}}", self.palindex()];
            } else {
                s += &format!["Channel {{{is}}}"];
            };

            s += &format![
                " (b{:08b}+x{:06X})",
                (self.nc.0 >> 24),
                (self.nc.0 & 0xFFFFFF)
            ];

            write!(f, "{} {}", s, alpha)
        }
    }

    impl From<Channel> for NcChannel {
        fn from(channel: Channel) -> NcChannel {
            channel.nc
        }
    }

    impl From<NcChannel> for Channel {
        fn from(nc: NcChannel) -> Channel {
            Self { nc }
        }
    }
}

/// # Constructors
impl Channel {
    pub fn new() -> Channel {
        NcChannel::new().into()
    }

    /// Creates a new channel with the default color.
    pub fn with_default() -> Channel {
        NcChannel::with_default().into()
    }

    pub fn from_rgb(rgb: impl Into<Rgb>) -> Channel {
        NcChannel::from_rgb(rgb.into()).into()
    }

    pub fn from_rgb_alpha(rgb: impl Into<Rgb>, alpha: Alpha) -> Channel {
        NcChannel::from_rgb_alpha(rgb.into(), alpha).into()
    }
}

/// # Default color methods
impl Channel {
    /// Is this channel using the default color? (vs. RGB or palette indexed).
    pub fn is_default(&self) -> bool {
        self.nc.default_p()
    }

    /// (Un)Sets the usage of the default color.
    ///
    /// Setting default to true also marks the channel as [`Opaque`][Alpha::Opaque].
    pub fn set_default(&mut self, default: bool) {
        if default {
            self.nc.set_default();
        } else {
            self.nc.set_not_default();
        }
    }
}

/// # Alpha and RGB methods
impl Channel {
    /// Gets the Alpha.
    pub fn alpha(&self) -> Alpha {
        self.nc.alpha()
    }

    /// Sets the Alpha.
    ///
    /// Also marks the channel as NOT using the “default color”.
    pub fn set_alpha(&mut self, alpha: Alpha) {
        self.nc.set_alpha(alpha);
    }

    /// Is this channel using RGB color? (vs. default or palette indexed).
    pub fn is_rgb(&self) -> bool {
        self.nc.rgb_p()
    }

    /// Gets the RGB values.
    pub fn rgb(&self) -> Rgb {
        self.nc.rgb()
    }

    /// Gets the red color component.
    pub fn r(&self) -> u8 {
        self.nc.r()
    }

    /// Gets the green color component.
    pub fn g(&self) -> u8 {
        self.nc.g()
    }

    /// Gets the blue color component.
    pub fn b(&self) -> u8 {
        self.nc.b()
    }

    /// Sets the RGB value.
    ///
    /// Also marks the channel as NOT using the “default color”.
    pub fn set_rgb(&mut self, rgb: impl Into<Rgb>) {
        self.nc.set_rgb(rgb.into());
    }

    /// Sets the red color component.
    pub fn set_r(&mut self, red: impl Into<u8>) {
        self.nc.set_r(red.into());
    }

    /// Sets the green color component.
    pub fn set_g(&mut self, green: impl Into<u8>) {
        self.nc.set_r(green.into());
    }

    /// Sets the blue color component.
    pub fn set_b(&mut self, blue: impl Into<u8>) {
        self.nc.set_b(blue.into());
    }
}

/// # Indexed palette methods
impl Channel {
    /// Is this channel using indexed palette colors? (vs. default or RGB)
    pub fn is_palindex(&self) -> bool {
        self.nc.palindex_p()
    }

    /// Gets the palette index from the channel.
    pub fn palindex(&self) -> u8 {
        self.nc.palindex()
    }

    /// Sets the palette index of the channel.
    ///
    /// Also marks the channel as [`Opaque`][Alpha::Opaque].
    pub fn set_palindex(&mut self, index: impl Into<u8>) {
        self.nc.set_palindex(index.into());
    }
}
