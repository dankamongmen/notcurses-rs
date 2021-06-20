#![allow(dead_code)]

use crate::sys::{self, NcAlign};

/// A `u8` of alignment within a plane or terminal.
//
// data type in C: u32
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Align {
    /// Left [`Align`]ment.
    Left = sys::NCALIGN_LEFT as u8,

    /// Right [`Align`]ment.
    Right = sys::NCALIGN_RIGHT as u8,

    /// Center [`Align`]ment.
    Center = sys::NCALIGN_CENTER as u8,

    /// Not [`Align`]ed.
    Unaligned = sys::NCALIGN_UNALIGNED as u8,
}

impl From<Align> for NcAlign {
    fn from(align: Align) -> NcAlign {
        align as NcAlign
    }
}

/// Any value that is not a valid [`NcAlign`] related constant
/// will be converted to [`Align::Unaligned`].
#[allow(clippy::wildcard_in_or_patterns)]
impl From<NcAlign> for Align {
    fn from(na: NcAlign) -> Align {
        match na {
            sys::NCALIGN_LEFT => Align::Left,
            sys::NCALIGN_RIGHT => Align::Right,
            sys::NCALIGN_CENTER => Align::Center,
            sys::NCALIGN_UNALIGNED | _ => Align::Unaligned,
        }
    }
}
