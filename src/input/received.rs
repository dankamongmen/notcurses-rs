// notcurses::input::received
//
//!
//

use super::Key;

/// A received [`char`] or [`Key`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Received {
    /// No input was received.
    ///
    /// A `0x00` (NUL) was received, meaning no input.
    NoInput,

    /// A synthesized event was received.
    Key(Key),

    /// A valid [`char`] was received.
    Char(char),
}

impl Received {
    /// Returns `true` if a specific `Key` has been received.
    #[inline]
    pub fn is_key(&self, key: Key) -> bool {
        matches!(self, Self::Key(k) if *k == key)
    }

    /// Returns `true` if a specific `char` has been received.
    #[inline]
    pub const fn is_char(&self, c: char) -> bool {
        matches!(self, Self::Char(ch) if *ch == c)
    }

    /// Returns the received `Key`, if any.
    #[inline]
    pub const fn key(&self) -> Option<Key> {
        if let Self::Key(k) = self {
            Some(*k)
        } else {
            None
        }
    }

    /// Returns the received `char`, if any.
    #[inline]
    pub const fn char(&self) -> Option<char> {
        if let Self::Char(c) = self {
            Some(*c)
        } else {
            None
        }
    }
}

mod core_impls {
    use super::Received;
    use crate::sys::NcReceived;
    use core::fmt;

    impl Default for Received {
        fn default() -> Self {
            Self::NoInput
        }
    }

    impl fmt::Display for Received {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use Received::*;
            let string = match self {
                Key(k) => format!["{k}"],
                Char(c) => format!["{c:?}"],
                NoInput => "NoInput".to_string(),
            };
            write!(f, "{}", string)
        }
    }
    impl fmt::Debug for Received {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use Received::*;
            let string = match self {
                Key(k) => format!["Key({k})"],
                Char(c) => format!["Char({c:?})"],
                NoInput => "NoInput".to_string(),
            };
            write!(f, "Received::{}", string)
        }
    }

    impl From<NcReceived> for Received {
        fn from(nc: NcReceived) -> Self {
            match nc {
                NcReceived::NoInput => Received::NoInput,
                NcReceived::Key(k) => Received::Key(k.into()),
                NcReceived::Char(c) => Received::Char(c),
            }
        }
    }
    impl From<Received> for NcReceived {
        fn from(r: Received) -> Self {
            match r {
                Received::NoInput => NcReceived::NoInput,
                Received::Key(k) => NcReceived::Key(k.into()),
                Received::Char(c) => NcReceived::Char(c),
            }
        }
    }
}
