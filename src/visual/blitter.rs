//!

use crate::sys;

bitflags! {
    /// Which blitter mode to use for rasterizing a [`Visual`][crate::Visual].
    ///
    /// There is a default mechanism of graceful degradation, that works as follows:
    /// - without braille support, BRAILLE decays to SEXTANT.
    /// - without bitmap support, PIXEL decays to SEXTANT.
    /// - without sextant support, SEXTANT decays to QUADRANT.
    /// - without quadrant support, QUADRANT decays to HALF.
    /// - the only viable blitters in ASCII are SPACE and PIXEL.
    ///
    /// If you don't want this behaviour you have to call [`no_degrade`] in
    /// [VisualBuilder].
    ///
    /// see also: [sys::NcBlitter].
    pub struct Blitter: u32 {
        /// Blitter mode where the blitter is automatically chosen.
        const DEFAULT = sys::NCBLIT_DEFAULT;

        /// Blitter mode using pixels/sixels.
        const PIXEL = sys::NCBLIT_PIXEL;

        /// Blitter mode using space (compatible with ASCII).
        const SPACE = sys::NCBLIT_1x1;

        /// Blitter mode using halves + 1x1 (space).
        /// ▄▀
        const HALF = sys::NCBLIT_2x1;

        /// Blitter mode using quadrants + 2x1.
        /// ▗▐ ▖▀▟▌▙
        const QUADRANT = sys::NCBLIT_2x2;

        /// Blitter mode using sextants.
        /// 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
        const SEXTANT = sys::NCBLIT_3x2;

        /// Blitter mode using four vertical levels.
        /// █▆▄▂
        const FOUR = sys::NCBLIT_4x1;

        /// Blitter mode using eight vertical levels.
        /// █▇▆▅▄▃▂▁
        const EIGHT = sys::NCBLIT_8x1;

        /// Blitter mode using braille (4 rows, 2 cols).
        /// ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
        const BRAILLE = sys::NCBLIT_BRAILLE;
    }
}

impl Default for Blitter {
    fn default() -> Self {
        Blitter::DEFAULT
    }
}
