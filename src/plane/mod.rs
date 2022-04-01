// notcurses::plane
//
//!
//

use libnotcurses_sys::NcPlane;

///
#[derive(Debug)]
pub struct Plane {
    nc: *mut NcPlane,
}

impl Plane {
    pub(crate) fn nc_ref(&self) -> &NcPlane {
        unsafe { &*self.nc }
    }

    pub(crate) fn nc_ref_mut(&mut self) -> &mut NcPlane {
        unsafe { &mut *self.nc }
    }
}

impl Drop for Plane {
    fn drop(&mut self) {
        let _ = self.nc_ref_mut().destroy().expect("Plane.drop()");
    }
}
