// notcurses::color::palette
//
//!
//

use crate::{sys::NcPalette, Channel, Notcurses, Result, Rgb};

/// An array of 256 `Channel`s.
#[derive(Clone, PartialEq, Eq)]
pub struct Palette {
    nc: *mut NcPalette, // TODO FIX
}

mod std_impls {
    use super::{NcPalette, Palette};
    use std::fmt;

    impl Drop for Palette {
        fn drop(&mut self) {
            self.into_ref_mut().free()
        }
    }

    impl fmt::Debug for Palette {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Palette {{ {:?}, {:?}, {:?}, … }}",
                self.channel(0),
                self.channel(1),
                self.channel(2),
            )
        }
    }
}

/// # `Palette` Constructors & desconstructors.
impl Palette {
    /// Creates a new palette, that's initialized with our best
    /// knowledge of the currently configured palette.
    pub fn new(terminal: &mut Notcurses) -> Palette {
        Self {
            nc: NcPalette::new(terminal.into_ref_mut())
        }
    }

    //

    /// Returns a shared reference to the inner [`NcPalette`].
    pub fn into_ref(&self) -> &NcPalette {
        unsafe { &*self.nc }
    }

    /// Returns an exclusive reference to the inner [`NcPalette`].
    pub fn into_ref_mut(&mut self) -> &mut NcPalette {
        unsafe { &mut *self.nc }
    }
}


/// # Methods
impl Palette {
    /// Attempts to use this palette in the `terminal`.
    pub fn use_in(&self, terminal: &mut Notcurses) -> Result<()> {
        Ok(self.into_ref().r#use(terminal.into_ref_mut())?)
    }

    /// Returns the `Channel` value at `index`.
    pub fn channel(&self, index: impl Into<u8>) -> Channel {
        crate::sys::NcChannel::from(self.into_ref().chans[index.into() as usize]).into()
    }
}
