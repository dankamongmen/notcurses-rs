// notcurses::color::scale
//
//!
//

/// Indicates how to scale a [`Visual`][crate::Visual] during rendering.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    /// Maintains the original size. Will Apply no scaling.
    ///
    /// This is the default.
    None,

    /// Maintains the aspect ratio.
    ///
    /// Scales a `Visual` to the `Plane`'s size without stretching.
    Scale,

    /// Like `None`, maintains the original size, while admitting
    /// high-resolution blitters that don't preserve the aspect ratio.
    NoneHiRes,

    /// Like `Scale`, maintains the aspect ratio, while admitting
    /// high-resolution blitters that don't preserve the aspect ratio.
    ScaleHiRes,

    /// Throws away aspect ratio.
    ///
    /// Stretches and scales the `Visual` in an attempt to fill the entirety
    /// of the `Plane`.
    Stretch,
}

mod std_impls {
    use super::Scale;
    use crate::sys::{c_api::NcScale_u32, NcScale};
    use std::fmt;

    impl Default for Scale {
        fn default() -> Self {
            Self::None
        }
    }

    impl fmt::Display for Scale {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Scale::None => "None",
                    Scale::Scale => "Scale",
                    Scale::NoneHiRes => "NoneHiRes",
                    Scale::ScaleHiRes => "ScaleHiRes",
                    Scale::Stretch => "Stretch",
                }
            )
        }
    }

    impl fmt::Debug for Scale {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Scale {{ {} }}", self)
        }
    }

    //

    impl From<NcScale> for Scale {
        fn from(nc: NcScale) -> Scale {
            match nc {
                NcScale::None => Scale::None,
                NcScale::Scale => Scale::Scale,
                NcScale::NoneHiRes => Scale::NoneHiRes,
                NcScale::ScaleHiRes => Scale::ScaleHiRes,
                NcScale::Stretch => Scale::Stretch,
                _ => Self::default(),
            }
        }
    }
    impl From<Scale> for NcScale {
        fn from(scale: Scale) -> NcScale {
            match scale {
                Scale::None => NcScale::None,
                Scale::Scale => NcScale::Scale,
                Scale::NoneHiRes => NcScale::NoneHiRes,
                Scale::ScaleHiRes => NcScale::ScaleHiRes,
                Scale::Stretch => NcScale::Stretch,
            }
        }
    }

    impl From<NcScale_u32> for Scale {
        fn from(ncu: NcScale_u32) -> Scale {
            NcScale::from(ncu).into()
        }
    }
    impl From<Scale> for NcScale_u32 {
        fn from(scale: Scale) -> NcScale_u32 {
            NcScale::from(scale).into()
        }
    }
}
