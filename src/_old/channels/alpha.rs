use crate::sys::{NcAlpha, c_api::NcAlpha_u32};

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
    Blend = (NcAlpha::Blend as u32 >> 24) as u8,

    /// The [`Cell`][crate::Cell]'s foreground color will be high-contrast
    /// (relative to the computed background).
    ///
    /// Note that the background cannot be highcontrast.
    HighContrast = (NcAlpha::HighContrast as u32 >> 24) as u8,

    /// The [`Cell`][crate::Cell]'s foreground or background color is used unchanged.
    Opaque = (NcAlpha::Opaque as u32 >> 24) as u8,

    /// The [`Cell`][crate::Cell]'s foreground or background color is derived
    /// entirely from the `Cell`s underneath it.
    Transparent = (NcAlpha::Transparent as u32 >> 24) as u8,
}

/// Defaults to [`Alpha::Opaque`].
impl Default for Alpha {
    fn default() -> Self {
        Alpha::Opaque
    }
}

impl From<Alpha> for NcAlpha {
    fn from(alpha: Alpha) -> NcAlpha {
        ((alpha as u8 as NcAlpha_u32) << 24).into()
    }
}

/// Any value that is not a valid [`NcAlpha`] related constant
/// will be converted to the default [`Alpha::Opaque`].
impl From<NcAlpha> for Alpha {
    fn from(na: NcAlpha) -> Alpha {
        match na {
            NcAlpha::Opaque => Alpha::Opaque,
            NcAlpha::Blend => Alpha::Blend,
            NcAlpha::Transparent => Alpha::Transparent,
            NcAlpha::HighContrast => Alpha::HighContrast,
            _ => Alpha::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Alpha, NcAlpha};
    #[test]
    fn alpha_shifts() {
        assert_eq![NcAlpha::Blend, u32::from(Alpha::Blend)];
        assert_eq![NcAlpha::Blend, Alpha::Blend.into()];
        assert_eq![NcAlpha::Blend, Alpha::from(NcAlpha::Blend).into()];
    }
    #[test]
    fn alpha_default_conversion_opaque() {
        assert_eq![NcAlpha::Opaque, Alpha::from(1337).into()];
    }
}
