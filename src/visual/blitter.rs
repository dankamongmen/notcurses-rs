#![allow(dead_code)]

use crate::sys;


/// A `u8` of [`Visual`][crate::Visual] blitter mode for rasterizing.
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
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Blitter {
    /// Blitter mode where the blitter is automatically chosen.
    Default = sys::NCBLIT_DEFAULT as u8,

    /// Blitter mode using pixels/sixels.
    Pixel = sys::NCBLIT_PIXEL as u8,

    /// Blitter mode using space (compatible with ASCII).
    Space = sys::NCBLIT_1x1 as u8,

    /// Blitter mode using halves + 1x1 (space).
    /// ▄▀
    Half = sys::NCBLIT_2x1 as u8,

    /// Blitter mode using quadrants + 2x1.
    /// ▗▐ ▖▀▟▌▙
    Quadrant = sys::NCBLIT_2x2 as u8,

    /// Blitter mode using sextants.
    /// 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
    Sextant = sys::NCBLIT_3x2 as u8,

    /// Blitter mode using four vertical levels.
    /// █▆▄▂
    Four = sys::NCBLIT_4x1 as u8,

    /// Blitter mode using eight vertical levels.
    /// █▇▆▅▄▃▂▁
    Eight = sys::NCBLIT_8x1 as u8,

    /// Blitter mode using braille (4 rows, 2 cols).
    /// ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    Braille = sys::NCBLIT_BRAILLE as u8,
}

impl Default for Blitter {
    fn default() -> Self {
        Blitter::Default
    }
}
