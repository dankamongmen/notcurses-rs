// notcurses::input::input_type
//
//!
//

/// The type of the [`Input`][crate::Input] event.
///
/// Note: *Unknown* and *Press* are considered equivalent.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    ///
    Unknown,

    ///
    Press,

    ///
    Repeat,

    ///
    Release,
}

impl InputType {
    /// Returns `true` if it's a `Press` input type.
    pub const fn is_press(&self) -> bool {
        matches!(self, Self::Press)
    }

    /// Returns `true` if it's a `Repeat` input type.
    pub const fn is_repeat(&self) -> bool {
        matches!(self, Self::Repeat)
    }

    /// Returns `true` if it's a `Release` input type.
    pub const fn is_release(&self) -> bool {
        matches!(self, Self::Release)
    }
}

mod core_impls {
    use super::InputType;
    use crate::sys::{c_api::NcInputType_u32, NcInputType};
    use core::fmt;

    impl Default for InputType {
        fn default() -> Self {
            Self::Unknown
        }
    }

    impl fmt::Display for InputType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    InputType::Unknown => "Unknown",
                    InputType::Press => "Press",
                    InputType::Repeat => "Repeat",
                    InputType::Release => "Release",
                }
            )
        }
    }

    impl fmt::Debug for InputType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "InputType::{}", self)
        }
    }

    impl From<NcInputType> for InputType {
        fn from(nc: NcInputType) -> Self {
            match nc {
                NcInputType::Unknown => InputType::Unknown,
                NcInputType::Press => InputType::Press,
                NcInputType::Repeat => InputType::Repeat,
                NcInputType::Release => InputType::Release,
            }
        }
    }
    impl From<InputType> for NcInputType {
        fn from(me: InputType) -> Self {
            match me {
                InputType::Unknown => NcInputType::Unknown,
                InputType::Press => NcInputType::Press,
                InputType::Repeat => NcInputType::Repeat,
                InputType::Release => NcInputType::Release,
            }
        }
    }

    impl From<NcInputType_u32> for InputType {
        fn from(ncu: NcInputType_u32) -> Self {
            NcInputType::from(ncu).into()
        }
    }
}
