// notcurses::visual
//
//!
//

use libnotcurses_sys::NcVisual;

///
#[derive(Debug)]
pub struct Visual {
    nc: *mut NcVisual,
}

impl Visual {
    pub(crate) fn nc_ref(&self) -> &NcVisual {
        unsafe { &*self.nc }
    }

    pub(crate) fn nc_ref_mut(&mut self) -> &mut NcVisual {
        unsafe { &mut *self.nc }
    }
}

impl Drop for Visual {
    fn drop(&mut self) {
        self.nc_ref_mut().destroy()
    }
}
