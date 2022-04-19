// notcurses::plane
//
//!
//

use crate::{sys::NcPlane, Notcurses, Result};

mod builder;
pub use builder::PlaneBuilder;

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
    /// Returns a new [`PlaneBuilder`].
    pub fn builder() -> PlaneBuilder {
        PlaneBuilder::new()
    }

    /// New `Plane` with default options.
    pub fn new(nc: &mut Notcurses) -> Result<Self> {
        Self::builder().build(nc)
    }

    //

    /// New child `Plane` with default options.
    pub fn new_child(&mut self) -> Result<Self> {
        Self::builder().build_child(self)
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
