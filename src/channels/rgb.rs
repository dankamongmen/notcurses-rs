use core::convert::TryInto;
use std::fmt;

use crate::sys::NcRgb;

/// A `u32` of 24bit `Rgb` data, part of a [`Channel`][crate::Channel].
///
/// # Diagram
/// ```txt
/// --------|RRRRRRRR|GGGGGGGG|BBBBBBBB
/// ```
///
/// See also: [`Rgba`][crate::Rgba], [`Channel`][crate::Channel]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb(pub NcRgb);

impl Rgb {
    pub const WHITE: Rgb = Self(0xFFFFFF);
    pub const SILVER: Rgb = Self(0xC0C0C0);
    pub const LIGHT_GREY: Rgb = Self(0xA0A0A0);
    pub const GREY: Rgb = Self(0x808080);
    pub const DARK_GREY: Rgb = Self(0x404040);
    pub const BLACK: Rgb = Self(0x000000);

    pub const RED: Rgb = Self(0xFF0000);
    pub const GREEN: Rgb = Self(0x00FF00);
    pub const BLUE: Rgb = Self(0x0000FF);
    pub const YELLOW: Rgb = Self(0xFFFF00);
    pub const CYAN: Rgb = Self(0x00FFFF);
    pub const MAGENTA: Rgb = Self(0xFF00FF);

    pub const DARK_RED: Rgb = Self(0x800000);
    pub const DARK_GREEN: Rgb = Self(0x008000);
    pub const DARK_BLUE: Rgb = Self(0x000080);
    pub const DARK_YELLOW: Rgb = Self(0x808000);
    pub const DARK_CYAN: Rgb = Self(0x008080);
    pub const DARK_MAGENTA: Rgb = Self(0x800080);

    /// Returns the red component.
    pub const fn r(&self) -> u8 {
        ((self.0 & 0xFF0000) >> 16_u8) as u8
    }

    /// Returns the green component.
    pub const fn g(&self) -> u8 {
        ((self.0 & 0x00FF00) >> 8_u8) as u8
    }

    /// Returns the blue component.
    pub const fn b(&self) -> u8 {
        (self.0 & 0x0000FF) as u8
    }

    /// Returns a copy of this `Rgb` with the added red component.
    pub const fn add_r(&self, r: u8) -> Self {
        Self(self.0 & !0xFF0000 | (r as u32) << 16_u8)
    }

    /// Returns a copy of this `Rgb` with the added green component.
    pub const fn add_g(&self, g: u8) -> Self {
        Self(self.0 & !0x00FF00 | (g as u32) << 8_u8)
    }

    /// Returns a copy of this `Rgb` with the added blue component.
    pub const fn add_b(&self, b: u8) -> Self {
        Self(self.0 & !0x0000FF | b as u32)
    }

    /// Sets the red component, and returns the resulting `Rgb`.
    pub fn set_r(&mut self, r: u8) -> Self {
        self.0 = self.0 & !0xFF0000 | (r as u32) << 16_u8;
        *self
    }

    /// Sets the green component, and returns the resulting `Rgb`.
    pub fn set_g(&mut self, g: u8) -> Self {
        self.0 = self.0 & !0x00FF00 | (g as u32) << 8_u8;
        *self
    }

    /// Sets the green component, and returns the resulting `Rgb`.
    pub fn set_b(&mut self, b: u8) -> Self {
        self.0 = self.0 & !0x0000FF | b as u32;
        *self
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "0x{:06X}", self.0)
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Self(0x000000) // black
    }
}

impl From<Rgb> for NcRgb {
    fn from(rgb: Rgb) -> Self {
        rgb.0
    }
}

/// Allow a tuple of (r, g, b) values where an `Rgb` is expected.
impl From<(u8, u8, u8)> for Rgb {
    fn from(t: (u8, u8, u8)) -> Self {
        Self((t.0 as u32) << 16_u8 | (t.1 as u32) << 8_u8 | t.2 as u32)
    }
}

/// Allow an `Rgb` where a tuple of (r, g, b) values is expected.
impl From<Rgb> for (u8, u8, u8) {
    fn from(rgb: Rgb) -> Self {
        (rgb.r(), rgb.g(), rgb.b())
    }
}

/// Allow an array of [r, g, b] values where an `Rgb` is expected.
impl From<[u8; 3]> for Rgb {
    fn from(t: [u8; 3]) -> Self {
        Self((t[0] as u32) << 16_u8 | (t[1] as u32) << 8_u8 | t[2] as u32)
    }
}

/// Allow an `Rgb` where an array of [r, g, b] values is expected.
impl From<Rgb> for [u8; 3] {
    fn from(rgb: Rgb) -> Self {
        [rgb.r(), rgb.g(), rgb.b()]
    }
}

/// This allows to accept Rust's default integer `i32`.
impl From<i32> for Rgb {
    fn from(int: i32) -> Self {
        Self(int.try_into().unwrap_or(0))
    }
}

// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::Rgb;

    #[test]
    fn rgb_r() {
        assert_eq!(0x11, Rgb(0x11_22_33).r());
    }
    #[test]
    fn rgb_g() {
        assert_eq!(0x22, Rgb(0x11_22_33).g());
    }
    #[test]
    fn rgb_b() {
        assert_eq!(0x33, Rgb(0x11_22_33).b());
    }
    #[test]
    fn rgb_add_r() {
        assert_eq!(Rgb(0x99_22_33), Rgb(0x11_22_33).add_r(0x99));
    }
    #[test]
    fn rgb_add_g() {
        assert_eq!(Rgb(0x11_99_33), Rgb(0x11_22_33).add_g(0x99));
    }
    #[test]
    fn rgb_add_b() {
        assert_eq!(Rgb(0x11_22_99), Rgb(0x11_22_33).add_b(0x99));
    }
    #[test]
    fn rgb_set_r() {
        let mut rgb = Rgb(0x11_22_33);
        assert_eq!(Rgb(0x99_22_33), rgb.set_r(0x99));
    }
    #[test]
    fn rgb_set_g() {
        let mut rgb = Rgb(0x11_22_33);
        assert_eq!(Rgb(0x11_99_33), rgb.set_g(0x99));
    }
    #[test]
    fn rgb_set_b() {
        let mut rgb = Rgb(0x11_22_33);
        assert_eq!(Rgb(0x11_22_99), rgb.set_b(0x99));
    }

    #[test]
    fn rgb_from_tuple() {
        assert_eq!(Rgb(0x11_22_33), Rgb::from((0x11, 0x22, 0x33)));
    }
    #[test]
    fn rgb_to_tuple() {
        assert_eq!((0x11, 0x22, 0x33), Rgb(0x11_22_33).into());
    }
    #[test]
    fn rgb_from_array() {
        assert_eq!(Rgb(0x11_22_33), Rgb::from([0x11, 0x22, 0x33]));
    }
    #[test]
    fn rgb_to_array() {
        assert_eq!([0x11, 0x22, 0x33], <[u8; 3]>::from(Rgb(0x11_22_33)));
    }
}
