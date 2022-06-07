// notcurses::color::rgb
//
//!
//

use crate::sys::{NcRgb, NcRgba};

/// A 24-bit RGB value.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Rgb(NcRgb);

/// A 32-bit RGBA value.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Rgba(NcRgba);

mod std_impls {
    use super::{Rgb, Rgba};
    use crate::sys::{
        c_api::{NcRgb_u32, NcRgba_u32},
        NcRgb, NcRgba,
    };
    use std::fmt;

    impl fmt::Display for Rgb {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:06X}", self.0)
        }
    }
    impl fmt::Debug for Rgb {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Rgb({})", self.0)
        }
    }

    impl fmt::Display for Rgba {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:08X}", self.0)
        }
    }
    impl fmt::Debug for Rgba {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Rgb({})", self.0)
        }
    }

    //

    impl From<NcRgb> for Rgb {
        fn from(nc: NcRgb) -> Rgb {
            Rgb(nc)
        }
    }
    impl From<Rgb> for NcRgb {
        fn from(rgb: Rgb) -> NcRgb {
            rgb.0
        }
    }

    impl From<NcRgba> for Rgba {
        fn from(nc: NcRgba) -> Rgba {
            Rgba(nc)
        }
    }
    impl From<Rgba> for NcRgba {
        fn from(rgba: Rgba) -> NcRgba {
            rgba.0
        }
    }

    //

    impl From<NcRgb_u32> for Rgb {
        fn from(ncu: NcRgb_u32) -> Rgb {
            Rgb(NcRgb::from(ncu))
        }
    }
    impl From<Rgb> for NcRgb_u32 {
        fn from(rgb: Rgb) -> NcRgb_u32 {
            rgb.0.into()
        }
    }

    impl From<NcRgba_u32> for Rgba {
        fn from(ncu: NcRgba_u32) -> Rgba {
            Rgba(NcRgba::from(ncu))
        }
    }
    impl From<Rgba> for NcRgba_u32 {
        fn from(rgba: Rgba) -> NcRgba_u32 {
            rgba.0.into()
        }
    }

    //

    impl From<[u8; 3]> for Rgb {
        fn from(array: [u8; 3]) -> Self {
            Rgb(array.into())
        }
    }
    impl From<Rgb> for [u8; 3] {
        fn from(rgb: Rgb) -> Self {
            rgb.0.into()
        }
    }
    impl From<(u8, u8, u8)> for Rgb {
        fn from(tuple: (u8, u8, u8)) -> Self {
            Rgb(tuple.into())
        }
    }
    impl From<Rgb> for (u8, u8, u8) {
        fn from(rgb: Rgb) -> Self {
            rgb.0.into()
        }
    }

    impl From<[u8; 4]> for Rgba {
        fn from(array: [u8; 4]) -> Self {
            Rgba(array.into())
        }
    }
    impl From<Rgba> for [u8; 4] {
        fn from(rgba: Rgba) -> Self {
            rgba.0.into()
        }
    }
    impl From<(u8, u8, u8, u8)> for Rgba {
        fn from(tuple: (u8, u8, u8, u8)) -> Self {
            Rgba(tuple.into())
        }
    }
    impl From<Rgba> for (u8, u8, u8, u8) {
        fn from(rgba: Rgba) -> Self {
            rgba.0.into()
        }
    }
}
