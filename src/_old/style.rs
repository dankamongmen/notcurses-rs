//!

use crate::sys::NcStyle;

bitflags! {
    /// A `u16` bitfield of all the styles you can apply to text.
    pub struct Style: u16 {
        /// Undercurl.
        const Undercurl= NcStyle::Undercurl.into();

        /// Strikethrough.
        const Struck = NcStyle::Struck.into();

        /// Italic.
        const Italic = NcStyle::Italic.into();

        /// Underline.
        const Underline = NcStyle::Underline.into();

        /// Extra bright or bold.
        const Bold = NcStyle::Bold.into();

        /// No styles.
        const None = NcStyle::None.into();
    }
}

/// Defaults to [`Style::None`].
impl Default for Style {
    fn default() -> Self {
        Style::None
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
            NcStyle::Undercurl => Style::Undercurl,
            NcStyle::Struck => Style::Struck,
            NcStyle::Italic => Style::Italic,
            NcStyle::Underline => Style::Underline,
            NcStyle::Bold => Style::Bold,
            NcStyle::None => Style::None,
            _ => Style::default(),
        }
    }
}
