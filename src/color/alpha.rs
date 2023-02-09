// notcurses::color::alpha
//
//!
//

/// Alpha information, part of a [`Channel`][super::Channel].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alpha {
    /// Indicates a [`Cell`][crate::plane::Cell]'s foreground or background color
    /// is used unchanged.
    ///
    /// This is the default.
    Opaque,

    /// Indicates a [`Cell`][crate::plane::Cell]'s foreground or background color
    /// is derived entirely from the `Cell`s underneath it.
    Transparent,

    /// Indicates a [`Cell`][crate::plane::Cell]'s foreground or background color will
    /// be a composite between its color and the `Cell`s' corresponding colors.
    Blend,

    /// Indicates the foreground color will be high-contrast,
    /// relative to the computed background.
    ///
    /// The background cannot be high-contrast.
    HighContrast,
}

mod core_impls {
    use super::Alpha;
    use crate::sys::{c_api::NcAlpha_u32, NcAlpha};
    use core::fmt;

    impl Default for Alpha {
        fn default() -> Self {
            Self::Opaque
        }
    }

    impl fmt::Display for Alpha {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use Alpha::*;
            write!(
                f,
                "{}",
                match self {
                    Opaque => "Opaque",
                    Transparent => "Transparent",
                    Blend => "Blend",
                    HighContrast => "HighContrast",
                }
            )
        }
    }

    //

    impl From<NcAlpha> for Alpha {
        fn from(nc: NcAlpha) -> Alpha {
            match nc {
                NcAlpha::Opaque => Alpha::Opaque,
                NcAlpha::Transparent => Alpha::Transparent,
                NcAlpha::Blend => Alpha::Blend,
                NcAlpha::HighContrast => Alpha::HighContrast,
            }
        }
    }
    impl From<Alpha> for NcAlpha {
        fn from(alpha: Alpha) -> NcAlpha {
            match alpha {
                Alpha::Opaque => NcAlpha::Opaque,
                Alpha::Transparent => NcAlpha::Transparent,
                Alpha::Blend => NcAlpha::Blend,
                Alpha::HighContrast => NcAlpha::HighContrast,
            }
        }
    }

    impl From<NcAlpha_u32> for Alpha {
        fn from(ncu: NcAlpha_u32) -> Alpha {
            NcAlpha::from(ncu).into()
        }
    }
    impl From<Alpha> for NcAlpha_u32 {
        fn from(align: Alpha) -> NcAlpha_u32 {
            NcAlpha::from(align).into()
        }
    }
}

impl Alpha {
    /// Displays the short name identifier of the alpha value.
    pub fn display_short(&self) -> &str {
        use Alpha::*;
        match self {
            Blend => "B",
            HighContrast => "H",
            Opaque => "O",
            Transparent => "T",
        }
    }
}
