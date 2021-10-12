#![allow(dead_code)]

use crate::sys::{NcAlign, NcAlignApi};

/// A `u8` of alignment within a [`Plane`][crate::Plane] or terminal.
//
// data type in C: u32
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Align {
    /// Left [`Align`]ment.
    Left = NcAlign::LEFT as u8,

    /// Right [`Align`]ment.
    Right = NcAlign::RIGHT as u8,

    /// Center [`Align`]ment.
    Center = NcAlign::CENTER as u8,

    /// Not [`Align`]ed.
    Unaligned = NcAlign::UNALIGNED as u8,
}

/// Defaults to [`Align::Unaligned`].
impl Default for Align {
    fn default() -> Self {
        Align::Unaligned
    }
}

impl From<Align> for NcAlign {
    fn from(align: Align) -> NcAlign {
        align as NcAlign
    }
}

/// Any value that is not a valid [`NcAlign`] related constant
/// will be converted to the default [`Align::Unaligned`].
impl From<NcAlign> for Align {
    fn from(na: NcAlign) -> Align {
        match na {
            NcAlign::LEFT => Align::Left,
            NcAlign::RIGHT => Align::Right,
            NcAlign::CENTER => Align::Center,
            NcAlign::UNALIGNED => Align::Unaligned,
            _ => Align::default(),
        }
    }
}
