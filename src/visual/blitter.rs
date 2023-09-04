// notcurses::visual::blitter
//
//!
//

/// Blitter mode to use for rasterizing a [`Visual`].
///
/// We never blit full blocks, but instead spaces (more efficient) with the
/// background set to the desired foreground.
///
/// # Degradation
///
/// There is a mechanism of graceful degradation, that works as follows:
///
/// [`Pixel`] > [`Sextant`] > [`Quadrant`] > [`Half`] > [`Ascii`].
///
/// If you don't want this behaviour you have to call the [`degrade`] method of
/// [`VisualBuilder`] (or the [`set_degrade`] method of [`Visual`]) to *false*.
///
/// [`Visualbuilder`]: super::VisualBuilder
/// [`degrade`]: super::VisualBuilder#method.degrade
/// [`Visual`]: super::Visual
/// [`set_degrade`]: super::Visual#method.set_degrade
///
/// [`Pixel`]: super::Blitter#variant.Pixel
/// [`Sextant`]: super::Blitter#variant.Sextant
/// [`Quadrant`]: super::Blitter#variant.Quadrant
/// [`Half`]: super::Blitter#variant.Half
/// [`Ascii`]: super::Blitter#variant.Ascii
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Blitter {
    ///
    Default,

    /// Blitter mode using only spaces, compatible with ASCII (1x1).
    Ascii,

    /// Blitter mode using halves + `Ascii` (2x1).
    ///
    /// ▄▀
    Half,

    /// Blitter mode using quadrants + `Half` (2x2).
    ///
    /// ▗▐ ▖▀▟▌▙
    Quadrant,

    /// Blitter mode using sextants + `Quadrant` (3x2).
    ///
    /// 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
    Sextant,

    /// Blitter mode using braille (4x2).
    ///
    /// ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    Braille,

    /// Blitter mode using Pixels/Sixels.
    ///
    Pixel,

    /// Blitter mode using: four vertical levels (4x1).
    ///
    /// █▆▄▂
    _4x1,

    /// Blitter mode using: eight vertical levels (8x1).
    ///
    /// █▇▆▅▄▃▂▁
    _8x1,
}

/// # aliases
#[allow(non_upper_case_globals)]
impl Blitter {
    pub const _1x1: Blitter = Blitter::Ascii;
    pub const _2x1: Blitter = Blitter::Half;
    pub const _2x2: Blitter = Blitter::Quadrant;
    pub const _3x2: Blitter = Blitter::Sextant;
}

mod core_impls {
    use super::Blitter;
    use crate::sys::{c_api::NcBlitter_u32, NcBlitter};
    use core::fmt;

    impl Default for Blitter {
        fn default() -> Self {
            Self::Default
        }
    }

    impl fmt::Display for Blitter {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Blitter::Default => "Default",
                    Blitter::Ascii => "Ascii",
                    Blitter::Half => "Half",
                    Blitter::Quadrant => "Quadrant",
                    Blitter::Sextant => "Sextant",
                    Blitter::Braille => "Braille",
                    Blitter::Pixel => "Pixel",
                    Blitter::_4x1 => "4x1",
                    Blitter::_8x1 => "8x1",
                }
            )
        }
    }

    impl fmt::Debug for Blitter {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Blitter::{}", self)
        }
    }

    //

    impl From<NcBlitter> for Blitter {
        fn from(nc: NcBlitter) -> Blitter {
            match nc {
                NcBlitter::Default => Blitter::Default,
                NcBlitter::Ascii => Blitter::Ascii,
                NcBlitter::Half => Blitter::Half,
                NcBlitter::Quadrant => Blitter::Quadrant,
                NcBlitter::Sextant => Blitter::Sextant,
                NcBlitter::Braille => Blitter::Braille,
                NcBlitter::Pixel => Blitter::Pixel,
                NcBlitter::_4x1 => Blitter::_4x1,
                NcBlitter::_8x1 => Blitter::_8x1,
                _ => Blitter::default(),
            }
        }
    }
    impl From<Blitter> for NcBlitter {
        fn from(blitter: Blitter) -> NcBlitter {
            match blitter {
                Blitter::Default => NcBlitter::Default,
                Blitter::Ascii => NcBlitter::Ascii,
                Blitter::Half => NcBlitter::Half,
                Blitter::Quadrant => NcBlitter::Quadrant,
                Blitter::Sextant => NcBlitter::Sextant,
                Blitter::Braille => NcBlitter::Braille,
                Blitter::Pixel => NcBlitter::Pixel,
                Blitter::_4x1 => NcBlitter::_4x1,
                Blitter::_8x1 => NcBlitter::_8x1,
                // _ => NcBlitter::default(),
            }
        }
    }

    impl From<NcBlitter_u32> for Blitter {
        fn from(ncu: NcBlitter_u32) -> Blitter {
            NcBlitter::from(ncu).into()
        }
    }
    impl From<Blitter> for NcBlitter_u32 {
        fn from(blitter: Blitter) -> NcBlitter_u32 {
            NcBlitter::from(blitter).into()
        }
    }
}

/// # methods
impl Blitter {
    /// The number of `height` subdivisions in a cell using the current blitter.
    ///
    /// Default & Pixel returns `None`.
    pub const fn cell_height(&self) -> Option<u8> {
        // self.cell_size().and_then(|size| Some(size.0) ) // not const
        if let Some(size) = self.cell_size() {
            Some(size.1)
        } else {
            None
        }
    }

    /// The number of `width` subdivisions in a cell using the current blitter.
    ///
    /// Default & Pixel returns `None`.
    pub const fn cell_width(&self) -> Option<u8> {
        // self.cell_size().and_then(|size| Some(size.1) ) // not const
        if let Some(size) = self.cell_size() {
            Some(size.0)
        } else {
            None
        }
    }

    /// The inner Cell's dimensions `(width, height)` using the current blitter.
    ///
    /// Default & Pixel returns `None`.
    pub const fn cell_size(&self) -> Option<(u8, u8)> {
        match self {
            Blitter::Ascii => Some((1, 1)),
            Blitter::Half => Some((1, 2)),
            Blitter::Quadrant => Some((2, 2)),
            Blitter::Sextant => Some((2, 3)),
            Blitter::Braille => Some((2, 4)),
            Blitter::_4x1 => Some((1, 4)),
            Blitter::_8x1 => Some((1, 8)),
            _ => None, // Default, Pixel, …
        }
    }
}
