// notcurses::color::palette
//
//!
//

use crate::{
    sys::{NcChannel, NcPalette},
    Channel, Notcurses, Result, Rgb,
};

/// An array of 256 `Channel`s.
#[derive(Clone, PartialEq, Eq)]
pub struct Palette {
    nc: *mut NcPalette,
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
                self.get_channel(0),
                self.get_channel(1),
                self.get_channel(2),
            )
        }
    }

    impl From<&mut NcPalette> for Palette {
        fn from(ncplane: &mut NcPalette) -> Palette {
            Palette {
                nc: ncplane as *mut NcPalette,
            }
        }
    }
}

/// # `Palette` Constructors & desconstructors.
impl Palette {
    /// Creates a new palette, that's initialized with our best
    /// knowledge of the currently configured palette.
    pub fn new(terminal: &mut Notcurses) -> Palette {
        Self {
            nc: NcPalette::new(terminal.into_ref_mut()),
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

    /// Returns the `Rgb` value at `index`.
    pub fn get(&self, index: impl Into<u8>) -> Rgb {
        self.into_ref().get(index.into())
    }

    /// Sets the `Rgb` value at `index`.
    pub fn set(&mut self, index: impl Into<u8>, rgb: impl Into<Rgb>) {
        self.into_ref_mut().set(index.into(), rgb);
    }

    /// Returns the channel at `index`.
    pub fn get_channel(&self, index: impl Into<u8>) -> Channel {
        NcChannel::from(self.into_ref().chans[index.into() as usize]).into()
    }

    /// Sets the `channel` value at `index`.
    pub fn set_channel(&mut self, index: impl Into<u8>, channel: impl Into<Channel>) {
        let ncc = NcChannel::from(channel.into());
        self.into_ref_mut().chans[index.into() as usize] = ncc.into();
    }
}
