#![allow(dead_code)]

use crate::sys::{NcScale, NcScaleApi};

/// A `u8` of [`Visual`][crate::Visual] scaling during rendering.
//
// data type in C: u32
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Scale {
    /// Maintains the original size.
    None = NcScale::NOSCALE as u8,

    /// Maintains the aspect ratio.
    Scale = NcScale::SCALE as u8,

    /// Throws away the aspect ratio.
    Stretch = NcScale::STRETCH as u8,

    /// Maintains the original size, admitting high-resolution blitters
    /// that don't preserve aspect ratio.
    NoneHires = NcScale::NONE_HIRES as u8,

    /// Maintains the aspect ratio, admitting high-resolution blitters
    /// that don't preserve aspect ratio.
    ScaleHires = NcScale::SCALE_HIRES as u8,
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
            NcScale::NOSCALE => Scale::None,
            NcScale::SCALE => Scale::Scale,
            NcScale::STRETCH => Scale::Stretch,
            NcScale::NONE_HIRES => Scale::NoneHires,
            NcScale::SCALE_HIRES => Scale::ScaleHires,
            _ => Scale::None,
        }
    }
}
