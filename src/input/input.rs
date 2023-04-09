// notcurses::input::input
//
//!
//

use crate::{
    input::{InputType, Key, KeyMod, Received},
    Position,
};

/// A received input.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Input {
    /// The received input event.
    pub received: Received,

    /// Keyboard modifiers.
    pub keymod: KeyMod,

    /// The type of the input.
    pub itype: InputType,

    /// The cell position of the event, if defined.
    pub cell: Option<Position>,

    /// Pixel offset within the cell, if defined.
    pub offset: Option<Position>,
}

mod core_impls {
    use super::{Input, Position};
    use crate::sys::{NcInput, NcReceived};
    use core::fmt;

    impl fmt::Display for Input {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let cell = if let Some(c) = self.cell { c.to_string() } else { "None".into() };
            let offset = if let Some(o) = self.offset { o.to_string() } else { "None".into() };
            write!(f,
                "{} {} {} {} {}",
                self.received, self.keymod, self.itype, cell, offset,
            )
        }
    }

    impl fmt::Debug for Input {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let cell = if let Some(c) = self.cell { c.to_string() } else { "None".into() };
            let offset = if let Some(o) = self.offset { o.to_string() } else { "None".into() };
            write!(f,
                "Input {{received:{} mod:{} type:{} cell:{} offset:{} }}",
                self.received, self.keymod, self.itype, cell, offset,
            )
        }
    }

    impl From<(NcReceived, NcInput)> for Input {
        fn from(received_input: (NcReceived, NcInput)) -> Input {
            let (received, input) = received_input;

            // cell position & offset is only relevant for mouse events
            let (mut cell, mut offset) = (None, None);
            if let NcReceived::Key(k) = received {
                if k.is_mouse() {
                    if input.y != -1 {
                        // != undefined
                        cell = Some(Position::new(input.x, input.y));
                    }
                    if input.ypx != -1 {
                        offset = Some(Position::new(input.xpx, input.ypx));
                    }
                }
            };

            Input {
                received: received.into(),
                keymod: input.modifiers.into(),
                itype: input.evtype.into(),
                cell,
                offset,
            }
        }
    }
}

/// # methods
impl Input {
    /* Received */

    /// Returns `true` if any actual input has been received.
    #[inline]
    pub const fn received(&self) -> bool {
        !matches![self.received, Received::NoInput]
    }

    /// Returns `true` if some [`Key`] has been received.
    #[inline]
    pub const fn some_key(&self) -> bool {
        matches!(self.received, Received::Key(_))
    }

    /// Returns `true` if a specific [`Key`] has been received.
    #[inline]
    pub fn is_key(&self, key: Key) -> bool {
        self.received.is_key(key)
    }

    /// Returns `true` if some `character` has been received.
    #[inline]
    pub const fn some_char(&self) -> bool {
        matches!(self.received, Received::Char(_))
    }

    /// Returns `true` if a specific `character` has been received.
    #[inline]
    pub const fn is_char(&self, character: char) -> bool {
        self.received.is_char(character)
    }

    /* InputType */

    /// Returns `true` if this' a `Press` input type.
    #[inline]
    pub const fn is_press(&self) -> bool {
        self.itype.is_press()
    }

    /// Returns `true` if this' a `Repeat` input type.
    #[inline]
    pub const fn is_repeat(&self) -> bool {
        self.itype.is_repeat()
    }

    /// Returns `true` if this' a `Release` input type.
    #[inline]
    pub const fn is_release(&self) -> bool {
        self.itype.is_release()
    }
}
