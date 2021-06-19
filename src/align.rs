#![allow(dead_code)]

use crate::sys;

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
