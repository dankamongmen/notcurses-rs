#![allow(dead_code, clippy::unusual_byte_groupings)]

use core::convert::TryInto;
use std::fmt;

use crate::sys::NcRgba;

/// A `u32` of 32bit `Rgba` data, used to build a [`Visual`][crate::Visual].
///
/// # Diagram
/// ```txt
/// AAAAAAAA|RRRRRRRR|GGGGGGGG|BBBBBBBB
/// ```
///
/// See also: [`Rgb`][crate::Rgb]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba(pub NcRgba);

impl Rgba {
    pub const BLACK: Rgba = Self(0xFF_000000);
    pub const WHITE: Rgba = Self(0xFF_FFFFFF);

    const ALPHA_MASK: u32 = 0xFF_000000;

    /// Change the alpha
    pub fn set_alpha(self, alpha: u8) -> Self {
        Self((self.0 & 0x00_FFFFFF) | (alpha as u32) << 24)
    }

    // pub fn less_alpha()
    // pub fn more_alpha()
}

// Traits Implementations
// -----------------------------------------------------------------------------

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "0x{:08X}", self.0)
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Self(0x00000000) // black with no opacity
    }
}

impl From<Rgba> for NcRgba {
    fn from(rgba: Rgba) -> Self {
        rgba.0
    }
}

/// This allows to accept Rust's default integer `i32` as an argument to the
/// functions expecting to receive an `Rgba`.
impl From<i32> for Rgba {
    fn from(int: i32) -> Self {
        Self(int.try_into().unwrap_or(0))
    }
}

// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::Rgba;

    #[test]
    fn rgba_set_alpha() {
        let rgba = Rgba(0xAA_112233);
        assert_eq!(Rgba(0xBB_112233), rgba.set_alpha(0xBB));
    }
}
