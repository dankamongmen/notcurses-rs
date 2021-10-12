//!

use crate::sys::{NcStyle, NcStyleApi};

bitflags! {
    /// A `u16` bitfield of all the styles you can apply to text.
    pub struct Style: u16 {
        /// Undercurl.
        const UNDERCURL= NcStyle::UNDERCURL as u16;

        /// Strikethrough.
        const STRUCK = NcStyle::STRUCK as u16;

        /// Italic.
        const ITALIC = NcStyle::ITALIC as u16;

        /// Underline.
        const UNDERLINE = NcStyle::UNDERLINE as u16;

        /// Extra bright or bold.
        const BOLD = NcStyle::BOLD as u16;

        /// No styles.
        const NONE = NcStyle::NOSTYLE as u16;
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
            NcStyle::UNDERCURL => Style::UNDERCURL,
            NcStyle::STRUCK => Style::STRUCK,
            NcStyle::ITALIC => Style::ITALIC,
            NcStyle::UNDERLINE => Style::UNDERLINE,
            NcStyle::BOLD => Style::BOLD,
            NcStyle::NOSTYLE => Style::NONE,
            _ => Style::default(),
        }
    }
}
