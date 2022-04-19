//
//!
//

use crate::{sys::Nc, Geometry, Result};

mod capabilities;
pub use capabilities::Capabilities;

/// *Notcurses* state for a given terminal, composed of *planes*.
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

    /// Returns the geometry of the terminal.
    pub fn geometry(&self) -> Geometry {
        let g = unsafe { self.into_ref().stdplane_const().pixel_geom() };
        Geometry {
            x: g.term_x,
            y: g.term_y,
            cols: g.term_x / g.cell_x,
            rows: g.term_y / g.cell_y,
            bx: g.max_bitmap_x,
            by: g.max_bitmap_y,
            bcols: g.max_bitmap_x / g.cell_x,
            brows: g.max_bitmap_y / g.cell_y,
            cx: g.cell_x,
            cy: g.cell_y,
        }
    }

    /// Returns the terminal dimensions in `(rows, columns)`.
    pub fn rows_cols(&self) -> (u32, u32) {
        self.into_ref().term_dim_yx()
    }

    // TODO: visual_geometry

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

    // TODO: supported_styles
}
