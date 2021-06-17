use crate::sys::{self, NcAlphaBits};

/// A `u32` of 2bit alpha, part of a [`Channel`][crate::Channel],
/// and surrounded by context dependent bits.
///
/// # Diagram
/// ```txt
/// ~~AA~~~~|--------|--------|--------
/// ```
pub struct Alpha(NcAlphaBits);

impl Alpha {
    /// The [`Cell`][crate::Cell]'s foreground or background color will be a
    /// composite between its color and the corresponding colors underneath it.
    pub const BLEND: Alpha = Self(sys::NCALPHA_BLEND);

    /// The [`Cell`][crate::Cell]'s foreground color will be high-contrast
    /// (relative to the computed background).
    ///
    /// Note that the background cannot be highcontrast.
    pub const HIGHCONTRAST: Alpha = Self(sys::NCALPHA_HIGHCONTRAST);

    /// The [`Cell`][crate::Cell]'s foreground or background color is used unchanged.
    pub const OPAQUE: Alpha = Self(sys::NCALPHA_OPAQUE);

    /// The [`Cell`][crate::Cell]'s foreground or background color is derived
    /// entirely from the `Cell`s underneath it.
    pub const TRANSPARENT: Alpha = Self(sys::NCALPHA_TRANSPARENT);
}

impl From<Alpha> for NcAlphaBits {
    fn from(a: Alpha) -> NcAlphaBits {
        a.0
    }
}
impl From<&Alpha> for NcAlphaBits {
    fn from(a: &Alpha) -> NcAlphaBits {
        a.0
    }
}
impl From<&mut Alpha> for NcAlphaBits {
    fn from(a: &mut Alpha) -> NcAlphaBits {
        a.0
    }
}
impl From<NcAlphaBits> for Alpha {
    fn from(na: NcAlphaBits) -> Alpha {
        Alpha(na)
    }
}
