//!

use crate::sys;

bitflags! {
    /// The style to apply to the text.
    pub struct Style: u16 {
        // The bitmask covering all the styles.
        // const MASK = sys::ffi::NCSTYLE_MASK as u16;

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

        /// No style.
        const NONE = sys::ffi::NCSTYLE_NONE as u16;
    }
}
