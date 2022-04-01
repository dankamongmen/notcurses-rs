//
//!
//

use crate::NcResult;
use libnotcurses_sys::Nc;

/// Notcurses state for a given terminal, composed of [`Plane`][crate::Plane]s.
#[derive(Debug)]
pub struct Notcurses {
    nc: *mut Nc,
}

// private functions
impl Notcurses {
    //
    pub(crate) fn nc_ref(&self) -> &Nc {
        unsafe { &*self.nc }
    }

    //
    pub(crate) fn nc_ref_mut(&mut self) -> &mut Nc {
        unsafe { &mut *self.nc }
    }
}

/// # Constructors
// WIP
impl Notcurses {
    ///
    pub fn new() -> NcResult<Self> {
        let nc = unsafe { Nc::new()? };
        Ok(Notcurses { nc })
    }
}

/// # Methdos
// TODO
impl Notcurses {}

impl Drop for Notcurses {
    fn drop(&mut self) {
        let _ = unsafe { self.nc_ref_mut().stop().expect("Notcurses.drop()") };
    }
}
