#![allow(dead_code)]

use crate::sys::{NcBlitter, NcBlitterApi};

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
    Default = NcBlitter::DEFAULT as u16,

    /// Blitter mode using space (compatible with ASCII).
    Space = NcBlitter::_1x1 as u16,

    /// Blitter mode using halves + 1x1 (space).
    /// ▄▀
    Half = NcBlitter::_2x1 as u16,

    /// Blitter mode using quadrants + 2x1.
    /// ▗▐ ▖▀▟▌▙
    Quadrant = NcBlitter::_2x2 as u16,

    /// Blitter mode using sextants.
    /// 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
    Sextant = NcBlitter::_3x2 as u16,

    /// Blitter mode using braille (4 rows, 2 cols).
    /// ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    Braille = NcBlitter::BRAILLE as u16,

    /// Blitter mode using pixels/sixels.
    Pixel = NcBlitter::PIXEL as u16,

    /// Blitter mode using four vertical levels.
    /// █▆▄▂
    Four = NcBlitter::_4x1 as u16,

    /// Blitter mode using eight vertical levels.
    /// █▇▆▅▄▃▂▁
    Eight = NcBlitter::_8x1 as u16,
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
            NcBlitter::DEFAULT => Blitter::Default,
            NcBlitter::PIXEL => Blitter::Pixel,
            NcBlitter::_2x2 => Blitter::Quadrant,
            NcBlitter::_3x2 => Blitter::Sextant,
            NcBlitter::_1x1 => Blitter::Space,
            NcBlitter::BRAILLE => Blitter::Braille,
            NcBlitter::_4x1 => Blitter::Four,
            NcBlitter::_8x1 => Blitter::Eight,
            _ => Blitter::Default,
        }
    }
}
