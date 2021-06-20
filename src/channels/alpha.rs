use crate::sys::{self, NcAlphaBits};

/// A `u8` of 2bit alpha, part of a [`Channel`][crate::Channel].
///
/// # Diagram
/// ```txt
/// --AA----
/// ```
///
/// Shifted right into an `u32`, as part of a `Channel`:
/// ```txt
/// --AA----|--------|--------|--------
/// ```
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Alpha {
    /// The [`Cell`][crate::Cell]'s foreground or background color will be a
    /// composite between its color and the corresponding colors underneath it.
    Blend = (sys::NCALPHA_BLEND >> 24_u8) as u8,

    /// The [`Cell`][crate::Cell]'s foreground color will be high-contrast
    /// (relative to the computed background).
    ///
    /// Note that the background cannot be highcontrast.
    HighContrast = (sys::NCALPHA_HIGHCONTRAST >> 24_u8) as u8,

    /// The [`Cell`][crate::Cell]'s foreground or background color is used unchanged.
    Opaque = (sys::NCALPHA_OPAQUE >> 24_u8) as u8,

    /// The [`Cell`][crate::Cell]'s foreground or background color is derived
    /// entirely from the `Cell`s underneath it.
    Transparent = (sys::NCALPHA_TRANSPARENT >> 24_u8) as u8,
}

impl From<Alpha> for NcAlphaBits {
    fn from(alpha: Alpha) -> NcAlphaBits {
        (alpha as NcAlphaBits) << 24_u8
    }
}

/// Any value that is not a valid [`NcAlphaBits`] related constant
/// will be converted to [`Alpha::Opaque`].
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

#[cfg(test)]
mod test {
    use super::{sys, Alpha};
    #[test]
    fn alpha_shifts() {
        assert_eq![sys::NCALPHA_BLEND, u32::from(Alpha::Blend)];
        assert_eq![sys::NCALPHA_BLEND, Alpha::Blend.into()];
        assert_eq![sys::NCALPHA_BLEND, Alpha::from(sys::NCALPHA_BLEND).into()];
    }
    #[test]
    fn alpha_default_conversion_opaque() {
        assert_eq![sys::NCALPHA_OPAQUE, Alpha::from(1337).into()];
    }
}
