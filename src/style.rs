//!

use crate::sys::{self, NcStyle};

bitflags! {
    /// A `u16` bitfield of all the styles you can apply to text.
    pub struct Style: u16 {
        /// Undercurl.
        const UNDERCURL= sys::ffi::NCSTYLE_UNDERCURL as u16;

        /// Strikethrough.
        const STRUCK = sys::ffi::NCSTYLE_STRUCK as u16;

        /// Italic.
        const ITALIC = sys::ffi::NCSTYLE_ITALIC as u16;

        /// Underline.
        const UNDERLINE = sys::ffi::NCSTYLE_UNDERLINE as u16;

        /// Extra bright or bold.
        const BOLD = sys::ffi::NCSTYLE_BOLD as u16;

        /// No styles.
        const NONE = sys::ffi::NCSTYLE_NONE as u16;
    }
}

/// Defaults to [`Style::NONE`].
impl Default for Style {
    fn default() -> Self {
        Style::NONE
    }
}

impl From<Style> for NcStyle {
    fn from(align: Style) -> NcStyle {
        align.bits() as NcStyle
    }
}

/// Any value that is not a valid [`NcStyle`] related constant
/// will be converted to the default [`Style::NONE`].
impl From<NcStyle> for Style {
    fn from(na: NcStyle) -> Style {
        match na {
            sys::NCSTYLE_UNDERCURL => Style::UNDERCURL,
            sys::NCSTYLE_STRUCK => Style::STRUCK,
            sys::NCSTYLE_ITALIC => Style::ITALIC,
            sys::NCSTYLE_UNDERLINE => Style::UNDERLINE,
            sys::NCSTYLE_BOLD => Style::BOLD,
            sys::NCSTYLE_NONE => Style::NONE,
            _ => Style::default(),
        }
    }
}
