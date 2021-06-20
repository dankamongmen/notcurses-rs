use crate::sys::{self, NcAlphaBits};

/// A `u32` of 2bit alpha, part of a [`Channel`][crate::Channel].
///
/// # Diagram
/// ```txt
/// ~~AA~~~~|--------|--------|--------
/// ```
//
// IMPROVE: store it as a u8, and bitshift on From …
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Alpha {
    /// The [`Cell`][crate::Cell]'s foreground or background color will be a
    /// composite between its color and the corresponding colors underneath it.
    Blend = sys::NCALPHA_BLEND,

    /// The [`Cell`][crate::Cell]'s foreground color will be high-contrast
    /// (relative to the computed background).
    ///
    /// Note that the background cannot be highcontrast.
    HighContrast = sys::NCALPHA_HIGHCONTRAST,

    /// The [`Cell`][crate::Cell]'s foreground or background color is used unchanged.
    Opaque = sys::NCALPHA_OPAQUE,

    /// The [`Cell`][crate::Cell]'s foreground or background color is derived
    /// entirely from the `Cell`s underneath it.
    Transparent = sys::NCALPHA_TRANSPARENT,
}

impl From<Alpha> for NcAlphaBits {
    fn from(a: Alpha) -> NcAlphaBits {
        a as NcAlphaBits
    }
}

impl From<NcAlphaBits> for Alpha {
    fn from(na: NcAlphaBits) -> Alpha {
        match na {
            sys::NCALPHA_OPAQUE => Alpha::Opaque,
            sys::NCALPHA_BLEND => Alpha::Blend,
            sys::NCALPHA_TRANSPARENT => Alpha::Transparent,
            sys::NCALPHA_HIGHCONTRAST => Alpha::HighContrast,
            _ => Alpha::Opaque,
        }
    }
}
