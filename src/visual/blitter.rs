#![allow(dead_code)]

use crate::sys::{self, NcBlitter};

/// A `u16` of [`Visual`][crate::Visual] blitter mode for rasterizing.
///
/// There is a default mechanism of graceful degradation, that works as follows:
/// - without braille support, BRAILLE decays to SEXTANT.
/// - without bitmap support, PIXEL decays to SEXTANT.
/// - without sextant support, SEXTANT decays to QUADRANT.
/// - without quadrant support, QUADRANT decays to HALF.
/// - the only viable blitters in ASCII are SPACE and PIXEL.
///
/// If you don't want this behaviour you have to call
/// [`no_degrade`][crate::builders::VisualBuilder#method.no_degrade] in the
/// [VisualBuilder][crate::builders::VisualBuilder].
//
// data type in C: u32
#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum Blitter {
    /// Blitter mode where the blitter is automatically chosen.
    Default = sys::NCBLIT_DEFAULT as u16,

    /// Blitter mode using space (compatible with ASCII).
    Space = sys::NCBLIT_1x1 as u16,

    /// Blitter mode using halves + 1x1 (space).
    /// ▄▀
    Half = sys::NCBLIT_2x1 as u16,

    /// Blitter mode using quadrants + 2x1.
    /// ▗▐ ▖▀▟▌▙
    Quadrant = sys::NCBLIT_2x2 as u16,

    /// Blitter mode using sextants.
    /// 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
    Sextant = sys::NCBLIT_3x2 as u16,

    /// Blitter mode using braille (4 rows, 2 cols).
    /// ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    Braille = sys::NCBLIT_BRAILLE as u16,

    /// Blitter mode using pixels/sixels.
    Pixel = sys::NCBLIT_PIXEL as u16,

    /// Blitter mode using four vertical levels.
    /// █▆▄▂
    Four = sys::NCBLIT_4x1 as u16,

    /// Blitter mode using eight vertical levels.
    /// █▇▆▅▄▃▂▁
    Eight = sys::NCBLIT_8x1 as u16,
}

/// Defaults to [`Blitter::Default`].
impl Default for Blitter {
    fn default() -> Self {
        Blitter::Default
    }
}

impl From<Blitter> for NcBlitter {
    fn from(blitter: Blitter) -> NcBlitter {
        blitter as NcBlitter
    }
}

/// Any value that is not a valid [`NcBlitter`] related constant
/// will be converted to [`Blitter::Default`].
impl From<NcBlitter> for Blitter {
    fn from(na: NcBlitter) -> Blitter {
        match na {
            sys::NCBLIT_DEFAULT => Blitter::Default,
            sys::NCBLIT_PIXEL => Blitter::Pixel,
            sys::NCBLIT_2x2 => Blitter::Quadrant,
            sys::NCBLIT_3x2 => Blitter::Sextant,
            sys::NCBLIT_1x1 => Blitter::Space,
            sys::NCBLIT_BRAILLE => Blitter::Braille,
            sys::NCBLIT_4x1 => Blitter::Four,
            sys::NCBLIT_8x1 => Blitter::Eight,
            _ => Blitter::Default,
        }
    }
}
