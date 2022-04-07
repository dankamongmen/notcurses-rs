// notcurses::visual
//
//!
//

use libnotcurses_sys::NcVisual;

/// A visual bit of multimedia.
#[derive(Debug)]
pub struct Visual {
    nc: *mut NcVisual,
}

/// # `Visual` constructors and deconstructors.
impl Visual {
    /// Returns a shared reference to the inner [`NcVisual`].
    pub fn into_ref(&self) -> &NcVisual {
        unsafe { &*self.nc }
    }

    /// Returns an exclusive reference to the inner [`NcVisual`].
    pub fn into_ref_mut(&mut self) -> &mut NcVisual {
        unsafe { &mut *self.nc }
    }
}

impl Drop for Visual {
    fn drop(&mut self) {
        self.into_ref_mut().destroy()
    }
}
