// notcurses::input::key_mod
//
//!
//

use crate::sys::{NcKeyMod};

/// A bitmask of keyboard modifiers.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KeyMod(u32);

/// # Flags
#[allow(non_upper_case_globals)]
impl KeyMod {
    ///
    pub const Shift: Self = Self(NcKeyMod::Shift.0);

    ///
    pub const Alt: Self = Self(NcKeyMod::Alt.0);

    ///
    pub const Ctrl: Self = Self(NcKeyMod::Ctrl.0);

    ///
    pub const Super: Self = Self(NcKeyMod::Super.0);

    ///
    pub const Hyper: Self = Self(NcKeyMod::Hyper.0);

    ///
    pub const Meta: Self = Self(NcKeyMod::Meta.0);

    ///
    pub const CapsLock: Self = Self(NcKeyMod::CapsLock.0);

    ///
    pub const NumLock: Self = Self(NcKeyMod::NumLock.0);

    /// None of the modifiers (all bits set to 0).
    pub const None: Self = Self(0);

    /// The modifier mask (all bits set to 1).
    pub const Mask: Self = Self(u32::MAX);
}

mod std_impls {
    use super::{KeyMod, NcKeyMod};
    use std::fmt;

    impl Default for KeyMod {
        fn default() -> Self {
            Self::None
        }
    }

    impl fmt::Display for KeyMod {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut string = String::new();

            if self.has_none() {
                string += "None ";
            } else {
                if self.has_capslock() {
                    string += "CapsLock+";
                }
                if self.has_numlock() {
                    string += "NumLock+";
                }
                if self.has_ctrl() {
                    string += "Ctrl+";
                }
                if self.has_shift() {
                    string += "Shift+";
                }
                if self.has_alt() {
                    string += "Alt+";
                }
                if self.has_meta() {
                    string += "Meta+";
                }
                if self.has_super() {
                    string += "Super+";
                }
                if self.has_hyper() {
                    string += "Hyper+";
                }
            }
            string.pop();

            write!(f, "{}", string)
        }
    }

    impl fmt::Debug for KeyMod {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "KeyMod::{}", self)
        }
    }
    crate::from_primitive![KeyMod, u32];
    crate::unit_impl_ops![bitwise; KeyMod, u32];
    crate::unit_impl_fmt![bases; KeyMod];

    impl From<NcKeyMod> for KeyMod {
        fn from(nc: NcKeyMod) -> Self {
            Self(nc.into())
        }
    }
    impl From<KeyMod> for NcKeyMod {
        fn from(k: KeyMod) -> Self {
            k.0.into()
        }
    }

    impl From<u32> for KeyMod {
        fn from(u: u32) -> KeyMod {
            Self(u)
        }
    }
}

/// # methods
impl KeyMod {
    /// Returns true if no modifiers are present.
    pub fn has_none(&self) -> bool {
        *self == KeyMod::None
    }

    /// Returns true if the `Shift` modifier is present.
    pub fn has_shift(&self) -> bool {
        *self & KeyMod::Shift != KeyMod::None
    }

    /// Returns true if the `Alt` modifier is present.
    pub fn has_alt(&self) -> bool {
        *self & KeyMod::Alt != KeyMod::None
    }

    /// Returns true if the `Ctrl` modifier is present.
    pub fn has_ctrl(&self) -> bool {
        *self & KeyMod::Ctrl != KeyMod::None
    }

    /// Returns true if the `Super` modifier is present.
    pub fn has_super(&self) -> bool {
        *self & KeyMod::Super != KeyMod::None
    }

    /// Returns true if the `Hyper` modifier is present.
    pub fn has_hyper(&self) -> bool {
        *self & KeyMod::Hyper != KeyMod::None
    }

    /// Returns true if the `Meta` modifier is present.
    pub fn has_meta(&self) -> bool {
        *self & KeyMod::Meta != KeyMod::None
    }

    /// Returns true if the `CapsLock` modifier is present.
    pub fn has_capslock(&self) -> bool {
        *self & KeyMod::CapsLock != KeyMod::None
    }

    /// Returns true if the `NumLock` modifier is present.
    pub fn has_numlock(&self) -> bool {
        *self & KeyMod::NumLock != KeyMod::None
    }
}
