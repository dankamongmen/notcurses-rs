#![allow(dead_code, clippy::unusual_byte_groupings)]

use core::convert::TryInto;

use crate::sys::{NcRgb, NcRgba};

/// A `u32` of RGB data.
///
/// # Diagram
/// ```txt
/// -------- RRRRRRRR GGGGGGGG BBBBBBBB
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb(pub NcRgb);

impl Default for Rgb {
    fn default() -> Self {
        Self(0) // black
    }
}

impl From<Rgb> for NcRgb {
    fn from(rgb: Rgb) -> Self {
        rgb.0
    }
}

/// This allows to accept Rust's default integer `i32` as an argument to the
/// functions expecting to receive an `Rgb`.
impl Into<Rgb> for i32 {
    fn into(self) -> Rgb {
        Rgb(self.try_into().unwrap_or(0))
    }
}

impl Rgb {
    pub const BLACK: Rgb = Self(0x000000);
    pub const WHITE: Rgb = Self(0xFFFFFF);

    // …
}

/// A `u32` of RGBA data.
///
/// # Diagram
/// ```txt
/// AAAAAAAA RRRRRRRR GGGGGGGG BBBBBBBB
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba(pub NcRgba);

impl Default for Rgba {
    fn default() -> Self {
        // Self(0x00000000) // black
        Self(0xFF000000) // black with full alpha
    }
}

impl From<Rgba> for NcRgba {
    fn from(rgba: Rgba) -> Self {
        rgba.0
    }
}

/// This allows to accept Rust's default integer `i32` as an argument to the
/// functions expecting to receive an `Rgba`.
impl Into<Rgba> for i32 {
    fn into(self) -> Rgba {
        Rgba(self.try_into().unwrap_or(0))
    }
}

impl Rgba {
    pub const BLACK: Rgba = Self(0xFF_000000);
    pub const WHITE: Rgba = Self(0xFF_FFFFFF);

    const ALPHA_MASK: u32 = 0xFF_000000;

    /// Change the alpha
    pub fn set_alpha(self, alpha: u8) -> Self {
        Self((self.0 & 0x00_FFFFFF) | (alpha as NcRgb) << (3 * 8))
    }

    // pub fn less_alpha()
    // pub fn more_alpha()
}

#[cfg(test)]
mod tests {
    use crate::Rgba;

    #[test]
    fn rgba_set_alpha() {
        let rgba = Rgba(0xAA_332211);
        assert_eq!(Rgba(0xBB_332211), rgba.set_alpha(0xBB));
    }
}
