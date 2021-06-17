use crate::sys::{self, NcAlphaBits};

bitflags! {
    /// 2 bits of alpha (surrounded by context dependent bits).
    /// It is part of an [`NcChannel`].
    ///
    pub struct Alpha: NcAlphaBits {
        /// The [`Cell`]'s foreground or background color will be a composite
        /// between its color and the corresponding colors underneath it.
        const BLEND = sys::NCALPHA_BLEND;

        /// The [`Cell`]'s foreground color will be high-contrast
        /// (relative to the computed background).
        ///
        /// Note that the background cannot be highcontrast.
        const HIGHCONTRAST = sys::NCALPHA_HIGHCONTRAST;

        /// The [`Cell`]'s foreground or background color is used unchanged.
        const OPAQUE = sys::NCALPHA_OPAQUE;

        /// The [`Cell`]'s foreground or background color is derived entirely
        /// from the `Cell`s underneath it.
        const TRANSPARENT = sys::NCALPHA_TRANSPARENT;
    }
}

impl From<Alpha> for NcAlphaBits {
    fn from(a: Alpha) -> NcAlphaBits {
        a.bits()
    }
}
impl From<&Alpha> for NcAlphaBits {
    fn from(a: &Alpha) -> NcAlphaBits {
        a.bits()
    }
}
impl From<&mut Alpha> for NcAlphaBits {
    fn from(a: &mut Alpha) -> NcAlphaBits {
        a.bits()
    }
}
impl From<NcAlphaBits> for Alpha {
    fn from(na: NcAlphaBits) -> Alpha {
        Alpha { bits: na }
    }
}
