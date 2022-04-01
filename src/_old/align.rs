#![allow(dead_code)]

use crate::sys::NcAlign;

/// A `u8` of alignment within a [`Plane`][crate::Plane] or terminal.
//
// data type in C: u32
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Align {
    /// Left [`Align`]ment.
    Left = NcAlign::Left as u8,

    /// Right [`Align`]ment.
    Right = NcAlign::Right as u8,

    /// Center [`Align`]ment.
    Center = NcAlign::Center as u8,

    /// Not [`Align`]ed.
    Unaligned = NcAlign::Unaligned as u8,
}

/// Defaults to [`Align::Unaligned`].
impl Default for Align {
    fn default() -> Self {
        Align::Unaligned
    }
}

impl From<Align> for NcAlign {
    fn from(align: Align) -> NcAlign {
        align.into()
    }
}

/// Any value that is not a valid [`NcAlign`] related constant
/// will be converted to the default [`Align::Unaligned`].
impl From<NcAlign> for Align {
    fn from(na: NcAlign) -> Align {
        match na {
            NcAlign::Left => Align::Left,
            NcAlign::Right => Align::Right,
            NcAlign::Center => Align::Center,
            NcAlign::Unaligned => Align::Unaligned,
            _ => Align::default(),
        }
    }
}
