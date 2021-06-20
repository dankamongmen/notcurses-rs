//!

use crate::sys;

bitflags! {
    /// A `u16` bitfield of all the styles you can apply to text.
    pub struct Style: u16 {
        /// Blinking.
        const UNDERCURL= sys::ffi::NCSTYLE_UNDERCURL as u16;

        /// Strikethrough.
        const STRUCK = sys::ffi::NCSTYLE_STRUCK as u16;

        /// Italic.
        const ITALIC = sys::ffi::NCSTYLE_ITALIC as u16;

        /// Best highlighting mode of the terminal.
        const STANDOUT = sys::ffi::NCSTYLE_STANDOUT as u16;

        /// Underlining.
        const UNDERLINE = sys::ffi::NCSTYLE_UNDERLINE as u16;

        /// Blinking.
        const BLINK= sys::ffi::NCSTYLE_BLINK as u16;

        /// Half bright.
        const DIM = sys::ffi::NCSTYLE_DIM as u16;

        /// Extra bright or bold.
        const BOLD = sys::ffi::NCSTYLE_BOLD as u16;

        /// Invisible or blank mode.
        const INVIS = sys::ffi::NCSTYLE_INVIS as u16;

        /// Protected mode.
        const PROTECT = sys::ffi::NCSTYLE_PROTECT as u16;

        /// Reverse video.
        const REVERSE = sys::ffi::NCSTYLE_REVERSE as u16;

        /// No styles.
        const NONE = sys::ffi::NCSTYLE_NONE as u16;
    }
}
