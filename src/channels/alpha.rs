use crate::sys::{NcAlpha, NcAlphaApi};

/// A `u8` of 2bit alpha, part of a [`Channel`][crate::Channel].
///
/// # Diagram
/// ```txt
/// --AA----
/// ```
///
/// Shifted right into an `u32`, as part of a [`Channel`][crate::Channel]:
/// ```txt
/// --AA----|--------|--------|--------
/// ```
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Alpha {
    /// The [`Cell`][crate::Cell]'s foreground or background color will be a
    /// composite between its color and the corresponding colors underneath it.
    Blend = (NcAlpha::BLEND >> 24_u8) as u8,

    /// The [`Cell`][crate::Cell]'s foreground color will be high-contrast
    /// (relative to the computed background).
    ///
    /// Note that the background cannot be highcontrast.
    HighContrast = (NcAlpha::HIGHCONTRAST >> 24_u8) as u8,

    /// The [`Cell`][crate::Cell]'s foreground or background color is used unchanged.
    Opaque = (NcAlpha::OPAQUE >> 24_u8) as u8,

    /// The [`Cell`][crate::Cell]'s foreground or background color is derived
    /// entirely from the `Cell`s underneath it.
    Transparent = (NcAlpha::TRANSPARENT >> 24_u8) as u8,
}

/// Defaults to [`Alpha::Opaque`].
impl Default for Alpha {
    fn default() -> Self {
        Alpha::Opaque
    }
}

impl From<Alpha> for NcAlpha {
    fn from(alpha: Alpha) -> NcAlpha {
        (alpha as NcAlpha) << 24_u8
    }
}

/// Any value that is not a valid [`NcAlpha`] related constant
/// will be converted to the default [`Alpha::Opaque`].
impl From<NcAlpha> for Alpha {
    fn from(na: NcAlpha) -> Alpha {
        match na {
            NcAlpha::OPAQUE => Alpha::Opaque,
            NcAlpha::BLEND => Alpha::Blend,
            NcAlpha::TRANSPARENT => Alpha::Transparent,
            NcAlpha::HIGHCONTRAST => Alpha::HighContrast,
            _ => Alpha::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Alpha, NcAlpha, NcAlphaApi};
    #[test]
    fn alpha_shifts() {
        assert_eq![NcAlpha::BLEND, u32::from(Alpha::Blend)];
        assert_eq![NcAlpha::BLEND, Alpha::Blend.into()];
        assert_eq![NcAlpha::BLEND, Alpha::from(NcAlpha::BLEND).into()];
    }
    #[test]
    fn alpha_default_conversion_opaque() {
        assert_eq![NcAlpha::OPAQUE, Alpha::from(1337).into()];
    }
}
