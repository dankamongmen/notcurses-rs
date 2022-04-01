#![allow(dead_code)]

use crate::sys::NcScale;

/// A `u8` of [`Visual`][crate::Visual] scaling during rendering.
//
// data type in C: u32
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Scale {
    /// Maintains the original size.
    None = NcScale::None as u8,

    /// Maintains the aspect ratio.
    Scale = NcScale::Scale as u8,

    /// Throws away the aspect ratio.
    Stretch = NcScale::Stretch as u8,

    /// Maintains the original size, admitting high-resolution blitters
    /// that don't preserve aspect ratio.
    NoneHiRes = NcScale::NoneHiRes as u8,

    /// Maintains the aspect ratio, admitting high-resolution blitters
    /// that don't preserve aspect ratio.
    ScaleHiRes = NcScale::ScaleHiRes as u8,
}

impl Default for Scale {
    fn default() -> Self {
        Scale::None
    }
}

impl From<Scale> for NcScale {
    fn from(scale: Scale) -> NcScale {
        scale as NcScale
    }
}

/// Any value that is not a valid [`NcScale`] related constant
/// will be converted to [`Scale::None`].
impl From<NcScale> for Scale {
    fn from(na: NcScale) -> Scale {
        match na {
            NcScale::None => Scale::None,
            NcScale::Scale => Scale::Scale,
            NcScale::Stretch => Scale::Stretch,
            NcScale::NoneHiRes => Scale::NoneHiRes,
            NcScale::ScaleHiRes => Scale::ScaleHiRes,
            _ => Scale::None,
        }
    }
}
