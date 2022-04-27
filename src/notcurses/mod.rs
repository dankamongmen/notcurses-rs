// notcurses::notcurses
//
//!
//

use crate::{sys::Nc, Blitter, Geometry, Result, Size, Style};

mod capabilities;
pub use capabilities::Capabilities;

/// *Notcurses* state for a given terminal, composed of [`Plane`][crate::Plane]s.
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

/// # `Notcurses` methods.
impl Notcurses {
    /// Refreshes the physical screen to match what was last rendered (i.e.,
    /// without reflecting any changes since the last call to
    /// [`render`][crate::Notcurses#method.render]).
    ///
    /// Returns the current screen geometry (`y`, `x`).
    ///
    /// This is primarily useful if the screen is externally corrupted, or if a
    /// resize] event has been read and you're not yet ready to render.
    pub fn refresh(&mut self) -> Result<(u32, u32)> {
        Ok(self.into_ref_mut().refresh()?)
    }
}

/// # `Notcurses` general information methods.
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

    /// Returns the terminal geometry with the best resolution blitter available,
    /// by following the [*rules of blitter degradation*].
    ///
    /// [*rules of blitter degradation*]: crate::sys::NcBlitter#degradation
    pub fn geometry_best(&self) -> Geometry {
        todo![]
    }

    /// Returns the terminal geometry using the requested blitter, if available.
    pub fn geometry_if(&self, blitter: Blitter) -> Option<Geometry> {
        todo![]
    }

    /// Returns the first terminal geometry available from the provided list.
    pub fn geometry_first(&self, blitters: Vec<Blitter>) -> Option<Geometry> {
        todo![]
    }

    /// Returns all the availeble terminal geometries from the provided list.
    pub fn geometries_all(&self, blitters: Vec<Blitter>) -> Vec<Geometry> {
        todo![]
    }

    /// Returns the terminal size `(height, width)`.
    pub fn size(&self) -> Size {
        self.into_ref().term_dim_yx().into()
    }

    /// Returns a human-readable string describing the running notcurses version.
    pub fn version() -> String {
        Nc::version()
    }

    /// Returns the running notcurses version components
    /// (major, minor, patch, tweak).
    pub fn version_components() -> (u32, u32, u32, u32) {
        Nc::version_components()
    }

    /// Returns the name of the user under which we are running.
    pub fn accountname() -> String {
        Nc::accountname()
    }

    /// Returns the name of the local hostname.
    pub fn hostname() -> String {
        Nc::hostname()
    }

    /// Returns the name of the detected terminal.
    pub fn detected_terminal(&self) -> String {
        self.into_ref().detected_terminal()
    }

    /// Returns the name of the detected OS version.
    pub fn osversion(&self) -> String {
        self.into_ref().osversion()
    }

    /// Returns an [`Style`] with the supported curses-style attributes.
    ///
    /// The attribute is only indicated as supported if the terminal can support
    /// it together with color.
    pub fn supported_styles(&self) -> Style {
        self.into_ref().supported_styles()
    }
}
