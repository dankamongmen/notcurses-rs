// notcurses::color::channels
//
//!
//

use crate::{
    color::{Alpha, Channel, Rgb},
    sys::{c_api::NcChannels_u64, NcChannels},
};

/// A pair of both foreground and background [`Channel`]s.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Channels {
    pub nc: NcChannels,
}

mod core_impls {
    use super::*;
    use core::fmt;

    impl fmt::Display for Channels {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let (fg, bg) = self.into();
            write!(f, "[{fg}, {bg}]")
        }
    }
    impl fmt::Debug for Channels {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let (fg, bg) = self.into();
            write!(f, "Channels {{ fg: {fg:?}, bg: {bg:?} }}")
        }
    }

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

    impl From<NcChannels_u64> for Channels {
        fn from(nc_u64: NcChannels_u64) -> Channels {
            NcChannels::from(nc_u64).into()
        }
    }

    /// Converts a `Channels` into a (fg, bg) `Channel` tuple.
    impl From<Channels> for (Channel, Channel) {
        fn from(c: Channels) -> (Channel, Channel) {
            (c.fg(), c.bg())
        }
    }
    impl From<&Channels> for (Channel, Channel) {
        fn from(c: &Channels) -> (Channel, Channel) {
            (c.fg(), c.bg())
        }
    }

    /// Helper for `impl From<{int} | ({int}, …) | [{int; …}]> for Channels`.
    macro_rules! impl_from_int_tuple_array {
        ($( $int:ty ),+) => {
            $( impl_from_int_tuple_array![single: $int]; )+
        };
        (single: $i:ty) => {
            /* different fg & bg channels */

            impl From<($i, $i, $i, $i, $i, $i)> for Channels {
                /// New `Channels` from different foreground & background (r, g, b, r, g, b).
                ///
                /// Performs a saturating cast to `[u8; 3], [u8; 3]`.
                fn from(tuple: ($i, $i, $i, $i, $i, $i)) -> Channels {
                    let fg_arr_u8 = [
                        tuple.0.clamp(0, <$i>::MAX) as u8,
                        tuple.1.clamp(0, <$i>::MAX) as u8,
                        tuple.2.clamp(0, <$i>::MAX) as u8,
                    ];
                    let bg_arr_u8 = [
                        tuple.3.clamp(0, <$i>::MAX) as u8,
                        tuple.4.clamp(0, <$i>::MAX) as u8,
                        tuple.5.clamp(0, <$i>::MAX) as u8,
                    ];
                    Self::from_rgb(fg_arr_u8, bg_arr_u8)
                }
            }
            impl From<(($i, $i, $i), ($i, $i, $i))> for Channels {
                /// New `Channels` from different foreground & background ((r, g, b), (r, g, b)).
                ///
                /// Performs a saturating cast to `[u8; 3], [u8; 3]`.
                fn from(tuple: (($i, $i, $i), ($i, $i, $i))) -> Channels {
                    Self::from((
                        tuple.0.0,
                        tuple.0.1,
                        tuple.0.2,
                        tuple.1.0,
                        tuple.1.1,
                        tuple.1.2,
                    ))
                }
            }
            impl From<[$i; 6]> for Channels {
                /// New `Channels` from different foreground & background [r, g, b, r, g, b]).
                ///
                /// Performs a saturating cast to `[u8; 3], [u8; 3]`.
                fn from(arr: [$i; 6]) -> Channels {
                    Self::from((
                        arr[0],
                        arr[1],
                        arr[2],
                        arr[3],
                        arr[4],
                        arr[5],
                    ))
                }
            }
            impl From<[[$i; 3]; 2]> for Channels {
                /// New `Channels` from different foreground & background [[r, g, b], [r, g, b]]).
                ///
                /// Performs a saturating cast to `[u8; 3], [u8; 3]`.
                fn from(arr: [[$i; 3]; 2]) -> Channels {
                    Self::from((
                        arr[0][0],
                        arr[0][1],
                        arr[0][2],
                        arr[1][0],
                        arr[1][1],
                        arr[1][2],
                    ))
                }
            }

            /* same bg & fg channels */

            impl From<($i, $i, $i)> for Channels {
                /// New `Channels` from same foreground & background (r, g, b).
                ///
                /// Performs a saturating cast to `[u8; 3], [u8; 3]`.
                fn from(tuple: ($i, $i, $i)) -> Channels {
                    let arr_u8 = [
                        tuple.0.clamp(0, <$i>::MAX) as u8,
                        tuple.1.clamp(0, <$i>::MAX) as u8,
                        tuple.2.clamp(0, <$i>::MAX) as u8,
                    ];
                    Self::from_rgb_both(arr_u8)
                }
            }
            impl From<[$i; 3]> for Channels {
                /// New `Channels` from same foreground & background (r, g, b).
                ///
                /// Performs a saturating cast to `[u8; 3], [u8; 3]`.
                fn from(arr: [$i; 3]) -> Channels {
                    Self::from((arr[0], arr[1], arr[2]))
                }
            }

            /* from impl Into<Channels> */

            impl From<($i, $i)> for Channels {
                /// New `Channels` from a pair of (fg, bg) impl Into<`Channel`>'s.
                fn from(tuple: ($i, $i)) -> Channels {
                    Channels::combine(tuple.0, tuple.1)
                }
            }
            impl From<[$i; 2]> for Channels {
                /// New `Channels` from a pair of [fg, bg] impl Into<`Channel`>'s.
                fn from(arr: [$i; 2]) -> Channels {
                    Channels::combine(arr[0], arr[1])
                }
            }
        };
    }
    impl_from_int_tuple_array!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
}

/// # constructors
impl Channels {
    /// A new pair of channels, set to black and NOT using the default colors.
    pub fn new() -> Channels {
        NcChannels::new().into()
    }

    /// A new pair of channels set to black and using the default colors.
    pub fn with_default() -> Channels {
        NcChannels::with_default().into()
    }

    /// A new pair of channels, using separate foreground and background colors.
    pub fn from_rgb(fg_rgb: impl Into<Rgb>, bg_rgb: impl Into<Rgb>) -> Channels {
        NcChannels::from_rgb(fg_rgb.into(), bg_rgb.into()).into()
    }

    /// A new pair of channels, using the same foreground and background colors.
    pub fn from_rgb_both(rgb: impl Into<Rgb>) -> Channels {
        NcChannels::from_rgb_both(rgb.into()).into()
    }

    /// A new pair of channels, using separate foreground and background colors,
    /// and alpha.
    pub fn from_rgb_alpha(
        fg_rgb: impl Into<Rgb>,
        fg_alpha: impl Into<Alpha>,
        bg_rgb: impl Into<Rgb>,
        bg_alpha: impl Into<Alpha>,
    ) -> Channels {
        NcChannels::from_rgb_alpha(
            fg_rgb.into(),
            fg_alpha.into(),
            bg_rgb.into(),
            bg_alpha.into(),
        )
        .into()
    }

    /// A new pair of channels, using the same foreground and background colors.
    pub fn from_rgb_alpha_both(rgb: impl Into<Rgb>, alpha: impl Into<Alpha>) -> Channels {
        NcChannels::from_rgb_alpha_both(rgb.into(), alpha.into()).into()
    }

    /// Combines two separate `Channel`s into `Channels`.
    pub fn combine(fg: impl Into<Channel>, bg: impl Into<Channel>) -> Channels {
        NcChannels::combine(fg.into(), bg.into()).into()
    }
}

/// # methods
impl Channels {
    /// Gets the foreground channel.
    pub fn fg(&self) -> Channel {
        self.nc.fchannel().into()
    }

    /// Gets the background channel.
    pub fn bg(&self) -> Channel {
        self.nc.bchannel().into()
    }

    /// Sets the foreground channel.
    pub fn set_fg(&mut self, fg: impl Into<Channel>) -> Channels {
        self.nc.set_fchannel(fg.into()).into()
    }

    /// Sets the background channel.
    pub fn set_bg(&mut self, bg: impl Into<Channel>) -> Channels {
        self.nc.set_fchannel(bg.into()).into()
    }

    /// Gets the alpha and coloring bits as `Channels`.
    pub fn channels(&self) -> Channels {
        self.nc.channels().into()
    }

    /// Sets the alpha and coloring bits from another `Channels`.
    pub fn set_channels(&mut self, other: impl Into<Channels>) -> Channels {
        self.nc.set_channels(other.into()).into()
    }

    /// Returns the Channels with the foreground and background's color information
    /// swapped, but without touching the rest of the bits.
    ///
    /// Alpha is retained unless it would lead to an illegal state:
    /// [`HighContrast`], [`Transparent`] and [`Blend`] are taken to [`Opaque`]
    /// unless the new value is Rgb.
    ///
    /// [`HighContrast`]: Alpha#HighContrast
    /// [`Transparent`]: Alpha#Transparent
    /// [`Blend`]: Alpha#Blend
    /// [`Opaque`]: Alpha#Opaque
    pub fn reverse(&mut self) -> Self {
        self.nc.reverse().into()
    }
}
