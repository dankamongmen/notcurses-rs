// notcurses::plane
//
//!
//

use crate::{
    sys::{NcPlane, NcPlaneOptions},
    Notcurses, Result,
};

/// A drawable text surface, composed of `Cell`s.
#[derive(Debug)]
pub struct Plane {
    nc: *mut NcPlane,
}

impl Drop for Plane {
    fn drop(&mut self) {
        let _ = self.into_ref_mut().destroy();
    }
}

/// # `Plane` constructors and deconstructors.
impl Plane {
    /// New `Plane`.
    ///
    pub fn new(nc: &mut Notcurses) -> Result<Self> {
        let options = NcPlaneOptions::builder().build();
        Ok(Self {
            nc: NcPlane::new_pile(nc.into_ref_mut(), &options)?,
        })
    }

    /// Returns a shared reference to the inner [`NcPlane`].
    pub fn into_ref(&self) -> &NcPlane {
        unsafe { &*self.nc }
    }

    /// Returns an exclusive reference to the inner [`NcPlane`].
    pub fn into_ref_mut(&mut self) -> &mut NcPlane {
        unsafe { &mut *self.nc }
    }
}
