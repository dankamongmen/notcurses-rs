//
//!
//

use crate::Result;
use libnotcurses_sys::Nc;

mod capabilities;
pub use capabilities::Capabilities;

/// `Notcurses` state for a given terminal, composed of [`Plane`][crate::Plane]s.
///
/// There can only be a single `Notcurses` instance per thread at any given moment.
#[derive(Debug)]
pub struct Notcurses {
    nc: *mut Nc,
}

impl Drop for Notcurses {
    fn drop(&mut self) {
        let _ = unsafe { self.into_ref_mut().stop().expect("Notcurses.drop()") };
    }
}

/// # `Notcurses` constructors & deconstructors.
impl Notcurses {
    /// Returns a new `Notcurses` context.
    pub fn new() -> Result<Self> {
        let nc = unsafe { Nc::new()? };
        Ok(Notcurses { nc })
    }

    /// Returns a new `Notcurses` context, without banners.
    pub fn new_silent() -> Result<Self> {
        let nc = unsafe { Nc::new_silent()? };
        Ok(Notcurses { nc })
    }

    /// Returns a new `Notcurses` context in `CLI` mode.
    pub fn new_cli() -> Result<Self> {
        let nc = unsafe { Nc::new_cli()? };
        Ok(Notcurses { nc })
    }

    /// Returns a new `Notcurses` context in `CLI` mode, without banners.
    pub fn new_cli_silent() -> Result<Self> {
        let nc = unsafe { Nc::new_cli_silent()? };
        Ok(Notcurses { nc })
    }

    /// Returns a shared reference to the inner [`Nc`].
    pub fn into_ref(&self) -> &Nc {
        unsafe { &*self.nc }
    }

    /// Returns an exclusive reference to the inner [`Nc`].
    pub fn into_ref_mut(&mut self) -> &mut Nc {
        unsafe { &mut *self.nc }
    }
}

/// # `Plane` constructors
impl Notcurses {
    // Returns a new plane.
    // pub fn plane_new(&mut self) -> Plane {
    //     Plane::new(self)
    // }
}

/// # `Visual` constructors
impl Notcurses {}

/// # Methods
impl Notcurses {
    /// Returns the capabilities of the terminal.
    pub fn capabilities(&self) -> Capabilities {
        Capabilities {
            halfblock: self.into_ref().canhalfblock(),
            quadrant: self.into_ref().canquadrant(),
            sextant: self.into_ref().cansextant(),
            braille: self.into_ref().canbraille(),
            utf8: self.into_ref().canutf8(),
            images: self.into_ref().canopen_images(),
            videos: self.into_ref().canopen_videos(),
            pixel: self.into_ref().canpixel(),
            pixel_impl: self.into_ref().check_pixel_support(),
            truecolor: self.into_ref().cantruecolor(),
            fade: self.into_ref().canfade(),
            palette_change: self.into_ref().canchangecolor(),
            palette_size: self.into_ref().palette_size().unwrap_or(0),
            cursor: true,
        }
    }
}
