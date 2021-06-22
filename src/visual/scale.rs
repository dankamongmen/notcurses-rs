#![allow(dead_code)]

use crate::sys::{self, NcScale};

/// A `u8` of [`Visual`][crate::Visual] scaling during rendering.
//
// data type in C: u32
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Scale {
    /// Maintains the original size.
    None = sys::NCSCALE_NONE as u8,

    /// Maintains the aspect ratio.
    Scale = sys::NCSCALE_SCALE as u8,

    /// Throws away the aspect ratio.
    Stretch = sys::NCSCALE_STRETCH as u8,

    /// Maintains the original size, admitting high-resolution blitters
    /// that don't preserve aspect ratio.
    NoneHires = sys::NCSCALE_NONE_HIRES as u8,

    /// Maintains the aspect ratio, admitting high-resolution blitters
    /// that don't preserve aspect ratio.
    ScaleHires = sys::NCSCALE_SCALE_HIRES as u8,
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
            sys::NCSCALE_NONE => Scale::None,
            sys::NCSCALE_SCALE => Scale::Scale,
            sys::NCSCALE_STRETCH => Scale::Stretch,
            sys::NCSCALE_NONE_HIRES => Scale::NoneHires,
            sys::NCSCALE_SCALE_HIRES => Scale::ScaleHires,
            _ => Scale::None,
        }
    }
}
