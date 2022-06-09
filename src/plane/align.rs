// notcurses::plane::align
//
//!
//

/// Alignment within a [`Plane`][crate::Plane] or terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    /// Anyhing unaligned wont be rendered.
    Unaligned,
    /// Left (== Top) alignment.
    ///
    /// This is the default alignment.
    Left,
    /// Center alignment.
    Center,
    /// Right (== Bottom) alignment.
    Right,
}

/// # aliases
#[allow(non_upper_case_globals)]
impl Align {
    /// Top (== Left) alignment.
    ///
    /// This is the default alignment.
    pub const Top: Align = Align::Left;
    /// Bottom (== Right]) alignment.
    pub const Bottom: Align = Align::Right;
}

mod std_impls {
    use super::Align;
    use crate::sys::{c_api::NcAlign_u32, NcAlign};
    use std::fmt;

    impl Default for Align {
        fn default() -> Self {
            Self::Left
        }
    }

    impl fmt::Display for Align {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use Align::*;
            write!(
                f,
                "{}",
                match self {
                    Left => "Left",
                    Center => "Center",
                    Right => "Right",
                    Unaligned => "Unaligned",
                }
            )
        }
    }

    //

    impl From<NcAlign> for Align {
        fn from(nc: NcAlign) -> Align {
            match nc {
                NcAlign::Left => Align::Left,
                NcAlign::Center => Align::Center,
                NcAlign::Right => Align::Right,
                NcAlign::Unaligned => Align::Unaligned,
            }
        }
    }
    impl From<Align> for NcAlign {
        fn from(align: Align) -> NcAlign {
            match align {
                Align::Left => NcAlign::Left,
                Align::Center => NcAlign::Center,
                Align::Right => NcAlign::Right,
                Align::Unaligned => NcAlign::Unaligned,
            }
        }
    }

    impl From<NcAlign_u32> for Align {
        fn from(ncu: NcAlign_u32) -> Align {
            NcAlign::from(ncu).into()
        }
    }
    impl From<Align> for NcAlign_u32 {
        fn from(align: Align) -> NcAlign_u32 {
            NcAlign::from(align).into()
        }
    }
    impl From<i32> for Align {
        fn from(nci: i32) -> Align {
            NcAlign::from(nci).into()
        }
    }
    impl From<Align> for i32 {
        fn from(align: Align) -> i32 {
            NcAlign::from(align).into()
        }
    }
}
